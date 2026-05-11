//! Real Windows shell-icon extraction for the result list.
//!
//! The TS side calls `icon_for_ext(ext, is_dir)` once per unique
//! (extension, is_dir) pair and caches the result as a `data:image/png`
//! URL. We use `SHGetFileInfoW` with `SHGFI_USEFILEATTRIBUTES` so a
//! dummy path like `_.xml` resolves the system-registered icon without
//! the file actually existing on disk. Folders pass any directory path
//! (or a dummy with `FILE_ATTRIBUTE_DIRECTORY`).
//!
//! Non-Windows platforms return `None`; the UI falls back to a textual
//! glyph for now.

#[cfg(windows)]
mod imp {
    use base64::Engine;
    use image::{ImageBuffer, ImageFormat, Rgba};
    use windows::Win32::Graphics::Gdi::{
        BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC,
        GetDIBits, GetObjectW, HDC, ReleaseDC,
    };
    use windows::Win32::Storage::FileSystem::{FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL};
    use windows::Win32::UI::Shell::{
        SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHGFI_USEFILEATTRIBUTES, SHGetFileInfoW,
    };
    use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};
    use windows::core::PCWSTR;

    /// Returns a `data:image/png;base64,…` URL for the system shell icon
    /// associated with the given extension. `is_dir = true` forces the
    /// generic folder icon. Returns `None` if extraction fails.
    pub fn icon_data_url(ext: &str, is_dir: bool) -> Option<String> {
        let png = extract_png(ext, is_dir)?;
        let b64 = base64::engine::general_purpose::STANDARD.encode(png);
        Some(format!("data:image/png;base64,{b64}"))
    }

    fn extract_png(ext: &str, is_dir: bool) -> Option<Vec<u8>> {
        let (path_w, attrs) = if is_dir {
            // Any non-empty path works; SHGFI_USEFILEATTRIBUTES + the
            // DIRECTORY attribute tell Shell to return the folder icon.
            (encode_wide("folder"), FILE_ATTRIBUTE_DIRECTORY)
        } else {
            // Build a dummy filename with the requested extension so the
            // Shell looks up its associated icon (e.g. ".xml" → the
            // registered handler's icon) without needing a real file.
            let dummy = if ext.is_empty() {
                "_".to_string()
            } else {
                format!("_.{}", ext)
            };
            (encode_wide(&dummy), FILE_ATTRIBUTE_NORMAL)
        };

        let mut sfi = SHFILEINFOW::default();
        let ok = unsafe {
            SHGetFileInfoW(
                PCWSTR(path_w.as_ptr()),
                attrs,
                Some(&mut sfi),
                std::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_ICON | SHGFI_LARGEICON | SHGFI_USEFILEATTRIBUTES,
            )
        };
        if ok == 0 || sfi.hIcon.is_invalid() {
            return None;
        }
        let png = icon_to_png(sfi.hIcon);
        unsafe {
            let _ = DestroyIcon(sfi.hIcon);
        }
        png
    }

    fn icon_to_png(hicon: HICON) -> Option<Vec<u8>> {
        let mut info = ICONINFO::default();
        unsafe { GetIconInfo(hicon, &mut info).ok()? };
        let color = info.hbmColor;
        let mask = info.hbmMask;

        // Pull the bitmap dimensions.
        let mut bmp = BITMAP::default();
        let got = unsafe {
            GetObjectW(
                color.into(),
                std::mem::size_of::<BITMAP>() as i32,
                Some(&mut bmp as *mut _ as *mut _),
            )
        };
        if got == 0 {
            unsafe {
                let _ = DeleteObject(color.into());
                let _ = DeleteObject(mask.into());
            }
            return None;
        }
        let width = bmp.bmWidth as i32;
        let height = bmp.bmHeight as i32;
        if width <= 0 || height <= 0 {
            unsafe {
                let _ = DeleteObject(color.into());
                let _ = DeleteObject(mask.into());
            }
            return None;
        }

        // Pull 32-bit BGRA pixels out of the icon's color bitmap.
        let mut bi = BITMAPINFO::default();
        bi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bi.bmiHeader.biWidth = width;
        // Negative height = top-down DIB so row 0 is the top row.
        bi.bmiHeader.biHeight = -height;
        bi.bmiHeader.biPlanes = 1;
        bi.bmiHeader.biBitCount = 32;
        bi.bmiHeader.biCompression = BI_RGB.0;

        let pixel_count = (width as usize) * (height as usize);
        let mut bgra = vec![0u8; pixel_count * 4];

        let hdc: HDC = unsafe { GetDC(None) };
        let scanlines = unsafe {
            GetDIBits(
                hdc,
                color,
                0,
                height as u32,
                Some(bgra.as_mut_ptr() as *mut _),
                &mut bi,
                DIB_RGB_COLORS,
            )
        };
        unsafe {
            ReleaseDC(None, hdc);
            let _ = DeleteObject(color.into());
            let _ = DeleteObject(mask.into());
        }
        if scanlines == 0 {
            return None;
        }

        // Convert BGRA → RGBA. The icon's BGRA alpha channel is usually
        // already premultiplied per Win32 convention; treating it as
        // straight alpha looks fine for the small sizes the row uses.
        let mut rgba = bgra;
        for px in rgba.chunks_exact_mut(4) {
            px.swap(0, 2);
        }

        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width as u32, height as u32, rgba)?;
        let mut out = Vec::with_capacity(2048);
        let mut cursor = std::io::Cursor::new(&mut out);
        img.write_to(&mut cursor, ImageFormat::Png).ok()?;
        Some(out)
    }

    fn encode_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

}

#[cfg(not(windows))]
mod imp {
    pub fn icon_data_url(_ext: &str, _is_dir: bool) -> Option<String> {
        None
    }
}

/// Tauri command: returns the icon for an extension (or a folder) as a
/// base64-encoded PNG data URL, or `None` if the system can't supply
/// one. The TS side caches by `(ext, is_dir)` so we only do this work
/// once per unique key.
///
/// Async + `spawn_blocking` so the Win32 SHGetFileInfo + DIB extraction
/// + PNG encoding don't block Tauri's IPC dispatch thread. With 200
/// result rows triggering icon lookups on first render, a synchronous
/// version was freezing the entire UI for seconds.
#[tauri::command]
pub async fn icon_for_ext(ext: String, is_dir: bool) -> Option<String> {
    let t0 = std::time::Instant::now();
    let ext_for_log = ext.clone();
    tracing::info!(target: "sourcerer::icons",
        ext = %ext_for_log, is_dir, "icon_for_ext ENTER");
    let result = tokio::task::spawn_blocking(move || imp::icon_data_url(&ext, is_dir))
        .await
        .ok()
        .flatten();
    tracing::info!(target: "sourcerer::icons",
        ext = %ext_for_log,
        is_dir,
        ms = t0.elapsed().as_millis() as u64,
        ok = result.is_some(),
        "icon_for_ext EXIT");
    result
}
