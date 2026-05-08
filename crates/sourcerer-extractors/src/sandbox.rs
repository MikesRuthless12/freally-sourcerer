//! Per-extraction supervisor.
//!
//! Wraps a single [`Extractor::extract`] call with two budgets:
//!
//!   1. **Time** — default 5 s. The supervisor polls a one-shot
//!      channel from a dedicated worker thread; on tick boundaries it
//!      compares `Instant::now() - start` to the configured budget.
//!      When the budget is exceeded the supervisor flips the
//!      [`TextSink`](crate::TextSink) cancel flag and waits up to
//!      `cancel_grace` for the worker to bail cooperatively. If the
//!      worker doesn't yield within the grace window the supervisor
//!      *returns* with [`SandboxError::TimeBudget`] anyway — the worker
//!      thread is left to finish in the background.
//!
//!      That last point is the framework contract: extractors **must**
//!      check `sink.is_cancelled()` on coarse boundaries (per-page,
//!      per-row, per-archive-entry). A non-cooperative extractor
//!      leaks a worker thread per breach. Phase 8 extractors are all
//!      designed against this contract; Phase 13 evaluates subprocess
//!      isolation for hostile third-party formats.
//!
//!   2. **Memory** — default 256 MiB. Polled per tick from the host
//!      RSS API (`/proc/self/status` on Linux, `GetProcessMemoryInfo`
//!      on Windows, no-op on macOS — Phase 7 prompt). Process-wide,
//!      not per-extraction; concurrent extractions all observe the
//!      same value. Same cancel-on-breach posture as the time budget.
//!
//! On macOS the RSS guard is a no-op by spec — the time budget is the
//! sole guard. Other Unixes (FreeBSD / OpenBSD / illumos) also fall
//! through to no-op so the workspace still builds — Sourcerer's three
//! support tiers are Win + macOS + Linux, but `cargo check` on a
//! BSD CI runner shouldn't break the build.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{RecvTimeoutError, sync_channel};
use std::time::{Duration, Instant};

use tracing::{debug, info, warn};

use crate::error::{ExtractError, SandboxError};
use crate::settings::{
    DEFAULT_MEMORY_CEILING_BYTES, DEFAULT_TEXT_CAP_BYTES, DEFAULT_TIME_BUDGET, PipelineSettings,
};
use crate::sink::TextSink;
use crate::{ExtractionStats, Extractor};

/// How often the supervisor polls for budget breaches when the worker
/// hasn't yet returned. 100 ms is the smallest interval that doesn't
/// burn measurable CPU yet still keeps the worst-case time-budget
/// overshoot below the budget itself (so a 5 s budget stops between
/// 5.0 s and 5.1 s).
pub const SUPERVISOR_TICK: Duration = Duration::from_millis(100);

/// How long the supervisor waits for the worker to bail after flipping
/// the cancel flag before returning anyway. 250 ms is enough for any
/// cooperative extractor to surface a `Cancelled`; a non-cooperative
/// one leaks a worker thread per breach (documented contract).
pub const CANCEL_GRACE: Duration = Duration::from_millis(250);

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub time_budget: Duration,
    pub memory_ceiling_bytes: usize,
    pub text_cap_bytes: usize,
    pub tick: Duration,
    pub cancel_grace: Duration,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            time_budget: DEFAULT_TIME_BUDGET,
            memory_ceiling_bytes: DEFAULT_MEMORY_CEILING_BYTES,
            text_cap_bytes: DEFAULT_TEXT_CAP_BYTES,
            tick: SUPERVISOR_TICK,
            cancel_grace: CANCEL_GRACE,
        }
    }
}

impl SandboxConfig {
    pub fn from_settings(s: &PipelineSettings) -> Self {
        Self {
            time_budget: s.time_budget,
            memory_ceiling_bytes: s.memory_ceiling_bytes,
            text_cap_bytes: s.text_cap_bytes,
            tick: SUPERVISOR_TICK,
            cancel_grace: CANCEL_GRACE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SandboxOutput {
    pub bytes: Vec<u8>,
    pub stats: ExtractionStats,
}

#[derive(Clone)]
pub struct Sandbox {
    config: SandboxConfig,
}

impl Sandbox {
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(SandboxConfig::default())
    }

    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    /// Run `extractor.extract(path, sink)` under the sandbox budgets.
    /// Returns the captured text bytes plus the stats the extractor
    /// reported on success; one of the [`SandboxError`] variants on
    /// any failure mode.
    pub fn execute(
        &self,
        extractor: Arc<dyn Extractor>,
        path: PathBuf,
    ) -> Result<SandboxOutput, SandboxError> {
        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_for_worker = Arc::clone(&cancel);
        let text_cap = self.config.text_cap_bytes;
        let path_for_worker = path.clone();

        let (tx, rx) = sync_channel::<Result<(Vec<u8>, ExtractionStats), ExtractError>>(1);

        std::thread::Builder::new()
            .name("sourcerer-extract-worker".into())
            .spawn(move || {
                let mut sink = TextSink::with_cancel(text_cap, cancel_for_worker);
                let result = match extractor.extract(&path_for_worker, &mut sink) {
                    Ok(stats) => Ok((sink.into_bytes(), stats)),
                    Err(e) => Err(e),
                };
                // Receiver may have hung up if the supervisor already
                // returned (cancel grace exceeded). Best-effort send.
                let _ = tx.send(result);
            })
            .map_err(|source| {
                warn!(error = %source, "failed to spawn extractor worker thread");
                SandboxError::SpawnFailed { source }
            })?;

        let start = Instant::now();
        loop {
            match rx.recv_timeout(self.config.tick) {
                Ok(Ok((bytes, stats))) => {
                    debug!(
                        elapsed_ms = start.elapsed().as_millis() as u64,
                        bytes = bytes.len(),
                        "extraction completed"
                    );
                    return Ok(SandboxOutput { bytes, stats });
                }
                Ok(Err(ExtractError::Cancelled)) => {
                    // Worker bailed before the supervisor decided why —
                    // most plausibly the caller flipped its own cancel
                    // arc, or some other race. Surface as TimeBudget so
                    // the daemon's retry policy treats it as a transient.
                    return Err(SandboxError::TimeBudget {
                        budget_ms: self.config.time_budget.as_millis() as u64,
                    });
                }
                Ok(Err(ExtractError::OutputTooLarge { cap })) => {
                    return Err(SandboxError::OutputTooLarge { cap });
                }
                Ok(Err(e)) => return Err(SandboxError::Extractor(e)),
                Err(RecvTimeoutError::Disconnected) => return Err(SandboxError::WorkerPanic),
                Err(RecvTimeoutError::Timeout) => {
                    let breach = if start.elapsed() >= self.config.time_budget {
                        Some(SandboxError::TimeBudget {
                            budget_ms: self.config.time_budget.as_millis() as u64,
                        })
                    } else if let Some(rss) = current_rss()
                        && rss > self.config.memory_ceiling_bytes
                    {
                        Some(SandboxError::MemoryCeiling {
                            ceiling_bytes: self.config.memory_ceiling_bytes,
                            rss_bytes: rss,
                        })
                    } else {
                        None
                    };
                    let Some(reason) = breach else { continue };
                    info!(
                        ?reason,
                        elapsed_ms = start.elapsed().as_millis() as u64,
                        "sandbox cancelling extraction"
                    );
                    cancel.store(true, Ordering::Relaxed);
                    // Wait one grace window for the worker. The match
                    // arms below cover every observable outcome.
                    return match rx.recv_timeout(self.config.cancel_grace) {
                        // Worker actually finished cleanly within grace
                        // — honor the result, do not surface the budget
                        // breach. The race window is tight (cancel was
                        // set after the budget check) but real: a fast
                        // extractor that completes between the budget
                        // check and the cancel.store is observably
                        // legitimate.
                        Ok(Ok((bytes, stats))) => Ok(SandboxOutput { bytes, stats }),
                        // Worker noticed cancel, bailed.
                        Ok(Err(ExtractError::Cancelled)) => Err(reason),
                        // Worker hit a sink overflow; surface as-is.
                        Ok(Err(ExtractError::OutputTooLarge { cap })) => {
                            Err(SandboxError::OutputTooLarge { cap })
                        }
                        // Other extractor error — pass through.
                        Ok(Err(e)) => Err(SandboxError::Extractor(e)),
                        // Worker still running past grace — non-
                        // cooperative. Surface the budget breach;
                        // worker thread is documented to leak.
                        Err(RecvTimeoutError::Timeout) => Err(reason),
                        // Worker died (panicked) while we were waiting.
                        // That's a panic, not a budget breach.
                        Err(RecvTimeoutError::Disconnected) => Err(SandboxError::WorkerPanic),
                    };
                }
            }
        }
    }
}

/// Read process RSS from the host. `None` on platforms with no usable
/// API (macOS by spec; other Unixes by build-target).
pub fn current_rss() -> Option<usize> {
    rss::current()
}

#[cfg(target_os = "linux")]
mod rss {
    use std::fs;

    pub fn current() -> Option<usize> {
        // /proc/self/status is line-oriented and ASCII; the VmRSS line
        // is "VmRSS:\t   12345 kB". The kernel always reports kB on
        // this interface, even on hugepage-heavy kernels.
        let s = fs::read_to_string("/proc/self/status").ok()?;
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                let kb_str = rest.split_whitespace().next()?;
                let kb: usize = kb_str.parse().ok()?;
                return Some(kb.saturating_mul(1024));
            }
        }
        None
    }
}

#[cfg(windows)]
mod rss {
    use std::mem::size_of;

    use windows_sys::Win32::System::ProcessStatus::{
        GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    pub fn current() -> Option<usize> {
        // SAFETY: GetCurrentProcess returns a pseudo-handle that the
        // current thread can always use; PROCESS_MEMORY_COUNTERS is
        // POD; the size we pass is the size of the struct we just
        // declared. Failure path (`ok == 0`) returns None — the caller
        // treats a missing reading as "no RSS data available" and
        // skips the memory check this tick.
        unsafe {
            let mut info: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            let ok = GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut info,
                size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );
            if ok == 0 {
                None
            } else {
                Some(info.WorkingSetSize)
            }
        }
    }
}

#[cfg(any(
    target_os = "macos",
    all(not(target_os = "linux"), not(target_os = "macos"), not(windows),),
))]
mod rss {
    pub fn current() -> Option<usize> {
        // macOS: spec says "no-op (rely on time budget)".
        // Other Unixes: keep the build green; treat as no-op.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExtractorId;
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;

    /// Cooperative extractor: writes a fixed string and returns.
    struct Quick;
    impl Extractor for Quick {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("quick")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(&self, _p: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
            sink.push_str("done")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            Ok(ExtractionStats {
                bytes_in: 0,
                bytes_out: sink.len() as u64,
                ..Default::default()
            })
        }
    }

    /// Cooperative-but-slow extractor: spins, yielding at every tick
    /// to check the cancel flag. Used to verify the time budget fires.
    struct CooperativeSlow {
        budget: Duration,
    }
    impl Extractor for CooperativeSlow {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("coop-slow")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(&self, _p: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
            let start = Instant::now();
            while start.elapsed() < self.budget {
                if sink.is_cancelled() {
                    return Err(ExtractError::Cancelled);
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Ok(ExtractionStats::default())
        }
    }

    /// Non-cooperative extractor: ignores the cancel flag entirely,
    /// then eventually returns. Used to verify the supervisor returns
    /// `TimeBudget` even when the worker is selfish.
    struct Naughty {
        sleep: Duration,
    }
    impl Extractor for Naughty {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("naughty")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(
            &self,
            _p: &Path,
            _sink: &mut TextSink,
        ) -> Result<ExtractionStats, ExtractError> {
            std::thread::sleep(self.sleep);
            Ok(ExtractionStats::default())
        }
    }

    /// Extractor that errors out for a canned reason. Used to verify
    /// the sandbox folds `ExtractError::Other` into
    /// `SandboxError::Extractor`.
    struct AlwaysErrs;
    impl Extractor for AlwaysErrs {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("err")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(
            &self,
            _p: &Path,
            _sink: &mut TextSink,
        ) -> Result<ExtractionStats, ExtractError> {
            Err(ExtractError::Malformed("nope".into()))
        }
    }

    /// Extractor that overflows the sink. Verifies the sandbox folds
    /// `ExtractError::OutputTooLarge` into `SandboxError::OutputTooLarge`.
    struct Floods {
        cap: usize,
    }
    impl Extractor for Floods {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("floods")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(&self, _p: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
            // Generate cap+1 bytes — guaranteed overflow.
            let chunk = vec![b'a'; self.cap + 1];
            sink.push_bytes(&chunk)
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            Ok(ExtractionStats::default())
        }
    }

    fn sandbox(time_ms: u64, mem_bytes: usize, text_cap: usize) -> Sandbox {
        Sandbox::new(SandboxConfig {
            time_budget: Duration::from_millis(time_ms),
            memory_ceiling_bytes: mem_bytes,
            text_cap_bytes: text_cap,
            tick: Duration::from_millis(20),
            cancel_grace: Duration::from_millis(100),
        })
    }

    #[test]
    fn quick_extractor_returns_output() {
        let sb = sandbox(2_000, usize::MAX, 1024);
        let out = sb
            .execute(Arc::new(Quick) as Arc<dyn Extractor>, "/x".into())
            .unwrap();
        assert_eq!(out.bytes, b"done");
        assert_eq!(out.stats.bytes_out, 4);
    }

    #[test]
    fn cooperative_slow_hits_time_budget() {
        let sb = sandbox(150, usize::MAX, 1024);
        let err = sb
            .execute(
                Arc::new(CooperativeSlow {
                    budget: Duration::from_secs(5),
                }) as Arc<dyn Extractor>,
                "/x".into(),
            )
            .unwrap_err();
        match err {
            SandboxError::TimeBudget { .. } => {}
            other => panic!("expected TimeBudget, got {other:?}"),
        }
    }

    #[test]
    fn naughty_extractor_returns_time_budget_within_grace() {
        // Worker sleeps 1.5s ignoring cancel; supervisor budget 100ms,
        // grace 100ms — total wall-clock <300ms. Verifies supervisor
        // returns even when worker won't bail.
        let sb = sandbox(100, usize::MAX, 1024);
        let start = Instant::now();
        let err = sb
            .execute(
                Arc::new(Naughty {
                    sleep: Duration::from_millis(1500),
                }) as Arc<dyn Extractor>,
                "/x".into(),
            )
            .unwrap_err();
        let elapsed = start.elapsed();
        assert!(
            matches!(err, SandboxError::TimeBudget { .. }),
            "expected TimeBudget, got {err:?}"
        );
        assert!(
            elapsed < Duration::from_millis(800),
            "supervisor should have returned within ~grace+budget, took {elapsed:?}"
        );
    }

    #[test]
    fn extractor_error_passes_through() {
        let sb = sandbox(2_000, usize::MAX, 1024);
        let err = sb
            .execute(Arc::new(AlwaysErrs) as Arc<dyn Extractor>, "/x".into())
            .unwrap_err();
        match err {
            SandboxError::Extractor(ExtractError::Malformed(_)) => {}
            other => panic!("expected Extractor(Malformed), got {other:?}"),
        }
    }

    #[test]
    fn sink_overflow_surfaces_as_output_too_large() {
        let sb = sandbox(2_000, usize::MAX, 8);
        let err = sb
            .execute(
                Arc::new(Floods { cap: 8 }) as Arc<dyn Extractor>,
                "/x".into(),
            )
            .unwrap_err();
        match err {
            SandboxError::OutputTooLarge { cap } => assert_eq!(cap, 8),
            other => panic!("expected OutputTooLarge, got {other:?}"),
        }
    }

    #[test]
    fn cancel_flag_visible_in_sink() {
        // Smoke for the wiring between Sandbox and TextSink: a sink
        // built with the cancel arc should observe a flag flip even
        // outside the sandbox.
        let cancel = Arc::new(AtomicBool::new(false));
        let sink = TextSink::with_cancel(16, Arc::clone(&cancel));
        assert!(!sink.is_cancelled());
        cancel.store(true, Ordering::Relaxed);
        assert!(sink.is_cancelled());
    }
}
