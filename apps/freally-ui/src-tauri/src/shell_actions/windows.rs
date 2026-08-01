//! Windows shell integration.
//!
//! `SHAssocEnumHandlers` is the API Explorer's own "Open with" list is
//! built from, so what we show matches what the shell shows — including
//! the user's per-extension overrides. Reading the registry by hand
//! would reproduce a subset of the same data and miss the UWP handlers.

use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use ::windows::Win32::System::Com::{
    COINIT_APARTMENTTHREADED, CoInitializeEx, CoTaskMemFree, CoUninitialize, IDataObject,
};
use ::windows::Win32::System::Ole::{
    OleFlushClipboard, OleInitialize, OleSetClipboard, OleUninitialize,
};
use ::windows::Win32::UI::Shell::Common::ITEMIDLIST;
use ::windows::Win32::UI::Shell::{
    ASSOC_FILTER_RECOMMENDED, BHID_DataObject, IAssocHandler, ILFree, IShellItem, IShellItemArray,
    SHAssocEnumHandlers, SHCreateItemFromParsingName, SHCreateShellItemArrayFromIDLists,
    SHGetIDListFromObject,
};
use ::windows::core::{HSTRING, PCWSTR, PWSTR};

use super::{AppHandler, Result, ShellError};

/// RAII COM apartment. Every entry point that touches COM initialises
/// its own — Tauri command handlers run on a pool thread that may not
/// have one, and leaving it initialised would leak apartment state onto
/// a thread the runtime reuses for unrelated work.
struct Apartment {
    /// True only when *our* `CoInitializeEx` call is the one that
    /// incremented this thread's COM init count.
    owned: bool,
}

impl Apartment {
    fn enter() -> Result<Self> {
        // SAFETY: paired with `CoUninitialize` in `Drop` — but only when
        // this call actually initialized COM. `RPC_E_CHANGED_MODE` means
        // the thread was already in a *different* apartment and COM was
        // NOT initialized by us; uninitializing on that path tears COM
        // down under whoever did initialize it, and the next COM call on
        // the reused pool thread fails with `CO_E_NOTINITIALIZED`.
        let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        if hr == ::windows::Win32::Foundation::RPC_E_CHANGED_MODE {
            return Ok(Self { owned: false });
        }
        if hr.is_err() {
            return Err(ShellError::Os(format!("CoInitializeEx failed: {hr:?}")));
        }
        Ok(Self { owned: true })
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        if self.owned {
            // SAFETY: matches the successful `CoInitializeEx` in `enter`.
            unsafe { CoUninitialize() };
        }
    }
}

pub fn candidates(path: &Path) -> Result<Vec<AppHandler>> {
    let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
        return Ok(Vec::new());
    };
    let _apartment = Apartment::enter()?;
    let mut out: Vec<AppHandler> = Vec::new();
    for_each_handler(ext, |handler| {
        // SAFETY: `handler` is live for the duration of the callback;
        // both getters return shell-allocated strings `co_string` frees.
        let (name, id) = unsafe {
            (
                co_string(handler.GetUIName().ok()),
                co_string(handler.GetName().ok()),
            )
        };
        if id.is_empty() {
            return ControlFlow::Continue(());
        }
        out.push(AppHandler {
            name: if name.is_empty() { id.clone() } else { name },
            id,
        });
        // The shell can list dozens for common extensions; the menu
        // stops being usable long before that.
        if out.len() >= MAX_HANDLERS {
            ControlFlow::Break(Ok(()))
        } else {
            ControlFlow::Continue(())
        }
    })?;
    Ok(out)
}

/// The shell can list dozens of handlers for a common extension; the
/// menu stops being usable long before that.
const MAX_HANDLERS: usize = 24;

/// Walk the registered handlers for `.ext`, calling `f` for each until
/// it breaks or the enumerator runs out.
///
/// Both callers need this loop, and hand-writing the `unsafe` COM
/// enumeration twice means the SAFETY reasoning has to hold in two
/// places. A COM pointer must not outlive its apartment, which is why
/// `launch` re-enumerates rather than caching what `candidates`
/// returned — the loop is shared, the handler is not.
fn for_each_handler<F>(ext: &str, mut f: F) -> Result<()>
where
    F: FnMut(&IAssocHandler) -> ControlFlow<Result<()>>,
{
    let dotted = HSTRING::from(format!(".{ext}"));
    // SAFETY: `dotted` outlives the call; the enumerator and every
    // handler it yields are COM objects the `windows` crate releases on
    // drop. `Next` writes at most one element into the slice.
    unsafe {
        let enumerator = SHAssocEnumHandlers(PCWSTR(dotted.as_ptr()), ASSOC_FILTER_RECOMMENDED)
            .map_err(|e| ShellError::Os(format!("SHAssocEnumHandlers failed: {e}")))?;
        loop {
            let mut slot: [Option<IAssocHandler>; 1] = [None];
            let mut fetched = 0u32;
            if enumerator.Next(&mut slot, Some(&mut fetched)).is_err() || fetched == 0 {
                return Ok(());
            }
            let Some(handler) = slot[0].take() else {
                return Ok(());
            };
            if let ControlFlow::Break(result) = f(&handler) {
                return result;
            }
        }
    }
}

/// Read a `PWSTR` the shell allocated with `CoTaskMemAlloc`, then free
/// it. Returns an empty string for null.
///
/// SAFETY: the caller must pass a pointer the shell returned and must
/// not use it afterwards.
unsafe fn co_string(p: Option<PWSTR>) -> String {
    let Some(p) = p else { return String::new() };
    if p.is_null() {
        return String::new();
    }
    // SAFETY: shell-allocated, NUL-terminated wide string.
    let s = unsafe { p.to_string() }.unwrap_or_default();
    // SAFETY: allocated by the shell with CoTaskMemAlloc.
    unsafe { CoTaskMemFree(Some(p.as_ptr() as *const _)) };
    s
}

pub fn launch(path: &Path, handler_id: &str) -> Result<()> {
    let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
        return Err(ShellError::NoHandlers);
    };
    let _apartment = Apartment::enter()?;
    let target = HSTRING::from(path.as_os_str());
    let mut launched = false;
    for_each_handler(ext, |handler| {
        // SAFETY: `handler` is live for the callback.
        if unsafe { co_string(handler.GetName().ok()) } != handler_id {
            return ControlFlow::Continue(());
        }
        launched = true;
        ControlFlow::Break(invoke_handler(handler, &target))
    })?;
    if launched {
        Ok(())
    } else {
        Err(ShellError::UnknownHandler(handler_id.to_string()))
    }
}

/// Hand `target` to one enumerated handler.
fn invoke_handler(handler: &IAssocHandler, target: &HSTRING) -> Result<()> {
    // SAFETY: `target` outlives the call; `item` and `data` are COM
    // objects released on drop.
    unsafe {
        let item: IShellItem = SHCreateItemFromParsingName(PCWSTR(target.as_ptr()), None)
            .map_err(|e| ShellError::Os(format!("SHCreateItemFromParsingName: {e}")))?;
        let data: IDataObject = item
            .BindToHandler(None, &BHID_DataObject)
            .map_err(|e| ShellError::Os(format!("BindToHandler(DataObject): {e}")))?;
        handler
            .Invoke(&data)
            .map_err(|e| ShellError::Os(format!("IAssocHandler::Invoke: {e}")))
    }
}

/// Terminals in preference order: Windows Terminal, then PowerShell 7,
/// then Windows PowerShell, then the console. The first that spawns
/// wins — probing with `where.exe` would double the process count for
/// no extra certainty.
const TERMINALS: &[(&str, &[&str])] = &[
    ("wt.exe", &["-d", "."]),
    ("pwsh.exe", &["-NoExit"]),
    ("powershell.exe", &["-NoExit"]),
    ("cmd.exe", &["/K"]),
];

pub fn open_terminal(dir: &Path) -> Result<()> {
    for (program, args) in TERMINALS {
        let mut cmd = std::process::Command::new(program);
        for a in *args {
            // `wt -d .` resolves against the child's working directory,
            // which we set below — so the literal `.` is correct.
            cmd.arg(a);
        }
        cmd.current_dir(dir);
        // A terminal *is* a console window; suppressing it would defeat
        // the point, so this deliberately skips `detach`'s no-window
        // flag and only nulls stdio.
        cmd.stdin(std::process::Stdio::null());
        if cmd.spawn().is_ok() {
            return Ok(());
        }
    }
    Err(ShellError::NoTerminal {
        tried: TERMINALS
            .iter()
            .map(|(p, _)| *p)
            .collect::<Vec<_>>()
            .join(", "),
    })
}

/// Put the files on the clipboard as `CF_HDROP`, which is what Explorer
/// pastes as real files.
///
/// The shell builds the data object for us: `SHCreateItemFromParsingName`
/// per path, then a shell item array bound to an `IDataObject`. Hand-
/// rolling the `DROPFILES` struct would work too but would then need its
/// own global-memory lifetime rules.
pub fn copy_files(paths: &[PathBuf]) -> Result<()> {
    let _apartment = Apartment::enter()?;
    // SAFETY: OleInitialize/OleUninitialize are paired below, and
    // `OleSetClipboard` requires OLE (not just COM) on this thread.
    unsafe { OleInitialize(None) }
        .map_err(|e| ShellError::Os(format!("OleInitialize failed: {e}")))?;
    let result = (|| -> Result<()> {
        let mut items: Vec<IShellItem> = Vec::with_capacity(paths.len());
        for p in paths {
            let h = HSTRING::from(p.as_os_str());
            // SAFETY: `h` outlives the call.
            let item: IShellItem = unsafe { SHCreateItemFromParsingName(PCWSTR(h.as_ptr()), None) }
                .map_err(|e| ShellError::Os(format!("SHCreateItemFromParsingName: {e}")))?;
            items.push(item);
        }
        let Some(first) = items.first() else {
            return Ok(());
        };
        // Single-item arrays go through the item itself; multi-item
        // arrays need the shell's own aggregation so CF_HDROP carries
        // every path.
        let data: IDataObject = if items.len() == 1 {
            // SAFETY: `first` is a live IShellItem.
            unsafe { first.BindToHandler(None, &BHID_DataObject) }
                .map_err(|e| ShellError::Os(format!("BindToHandler(DataObject): {e}")))?
        } else {
            // A multi-file CF_HDROP needs a shell item *array*, and the
            // only constructor that takes more than one item takes
            // PIDLs — so unwrap each item to its id-list, build the
            // array, and free the id-lists once the array owns copies.
            let mut pidls: Vec<*const ITEMIDLIST> = Vec::with_capacity(items.len());
            let mut owned: Vec<*mut ITEMIDLIST> = Vec::with_capacity(items.len());
            let build = (|| -> Result<IShellItemArray> {
                for item in &items {
                    // SAFETY: `item` is a live IShellItem; the returned
                    // PIDL is ours to free.
                    let pidl = unsafe { SHGetIDListFromObject(item) }
                        .map_err(|e| ShellError::Os(format!("SHGetIDListFromObject: {e}")))?;
                    owned.push(pidl);
                    pidls.push(pidl as *const _);
                }
                // SAFETY: every pointer in `pidls` is live for the call.
                unsafe { SHCreateShellItemArrayFromIDLists(&pidls) }
                    .map_err(|e| ShellError::Os(format!("SHCreateShellItemArray: {e}")))
            })();
            for pidl in owned {
                // SAFETY: each was allocated by SHGetIDListFromObject
                // and has not been freed.
                unsafe { ILFree(Some(pidl as *const _)) };
            }
            let array = build?;
            // SAFETY: `array` is live.
            unsafe { array.BindToHandler(None, &BHID_DataObject) }
                .map_err(|e| ShellError::Os(format!("BindToHandler(DataObject): {e}")))?
        };
        // SAFETY: `data` is a live IDataObject; the clipboard takes its
        // own reference.
        unsafe { OleSetClipboard(Some(&data)) }
            .map_err(|e| ShellError::Os(format!("OleSetClipboard: {e}")))?;
        // SAFETY: hands ownership of the clipboard data to the OS so it
        // survives this process exiting.
        unsafe { OleFlushClipboard() }
            .map_err(|e| ShellError::Os(format!("OleFlushClipboard: {e}")))?;
        Ok(())
    })();
    // SAFETY: matches the `OleInitialize` above.
    unsafe { OleUninitialize() };
    result
}
