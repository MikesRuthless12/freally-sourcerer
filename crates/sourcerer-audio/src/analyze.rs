//! Symphonia-driven audio analyzer.
//!
//! Decodes one packet at a time, converts the audio buffer to
//! interleaved `f32`, and feeds it into both [`LoudnessAccumulator`]
//! (LUFS + true peak) and [`SilenceCounter`]. Returns a complete
//! [`AudioAttributes`] when the stream is exhausted or a typed
//! [`AudioError`] otherwise.

use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::attributes::AudioAttributes;
use crate::error::AudioError;
use crate::is_audio_extension;
use crate::measure::loudness::LoudnessAccumulator;
use crate::measure::silence::SilenceCounter;

/// Optional knobs for [`analyze_with_opts`]. The cooperative `cancel`
/// flag is the primary one — wrap an `Arc<AtomicBool>` and flip it
/// from another thread to abort a long decode.
///
/// `time_budget` is the higher-level "give up after N seconds" knob
/// the cache uses by default. When set, the analyzer spawns a
/// detached supervisor thread that flips the cancel flag once the
/// budget is exhausted. The supervisor self-cleans when the analyzer
/// returns normally — there is no leaked thread on the success path.
#[derive(Debug, Clone, Default)]
pub struct AnalysisOpts {
    pub cancel: Option<Arc<AtomicBool>>,
    pub time_budget: Option<std::time::Duration>,
}

impl AnalysisOpts {
    pub fn with_cancel(cancel: Arc<AtomicBool>) -> Self {
        Self {
            cancel: Some(cancel),
            time_budget: None,
        }
    }

    pub fn with_time_budget(budget: std::time::Duration) -> Self {
        Self {
            cancel: None,
            time_budget: Some(budget),
        }
    }
}

/// Default per-extraction time budget. Mirrors the Phase-7 sandbox's
/// `DEFAULT_TIME_BUDGET = 5 s`. Audio decode is cooperative (the
/// analyzer checks `cancel` per packet) so this is a soft cap — a
/// hostile single-packet file that out-runs the cancel check still
/// finishes the packet, but cannot loop indefinitely.
pub const DEFAULT_AUDIO_TIME_BUDGET: Duration = Duration::from_secs(5);

/// Time-budget supervisor. Spawns a background thread that blocks on
/// a `done` flag with timeout; on timeout it flips `cancel`. On clean
/// completion the analyzer's `Drop` flips `done`, which signals the
/// supervisor to exit immediately rather than waiting out the budget.
struct TimeBudgetSupervisor {
    done: Arc<AtomicBool>,
}

impl TimeBudgetSupervisor {
    fn spawn(budget: Duration, cancel: Arc<AtomicBool>) -> Self {
        let done = Arc::new(AtomicBool::new(false));
        let done_clone = Arc::clone(&done);
        // Detached thread. The thread's only state is the two
        // `Arc<AtomicBool>`s; it sleeps in 50 ms ticks and exits when
        // either the budget elapses (flip `cancel`) or the analyzer
        // calls `Drop` on the supervisor (flip `done`).
        std::thread::Builder::new()
            .name("sourcerer-audio-budget".into())
            .spawn(move || {
                let start = std::time::Instant::now();
                let tick = Duration::from_millis(50);
                while !done_clone.load(Ordering::Acquire) {
                    if start.elapsed() >= budget {
                        cancel.store(true, Ordering::Release);
                        return;
                    }
                    std::thread::sleep(tick);
                }
            })
            .expect("supervisor thread spawn");
        Self { done }
    }
}

impl Drop for TimeBudgetSupervisor {
    fn drop(&mut self) {
        // Signal the supervisor to exit early on the success path.
        self.done.store(true, Ordering::Release);
    }
}

/// Analyze a file at `path`, returning a fresh [`AudioAttributes`].
/// Equivalent to `analyze_with_opts(path, AnalysisOpts::default())`.
pub fn analyze_file(path: &Path) -> Result<AudioAttributes, AudioError> {
    analyze_with_opts(path, AnalysisOpts::default())
}

pub fn analyze_with_opts(path: &Path, opts: AnalysisOpts) -> Result<AudioAttributes, AudioError> {
    // Resolve the cancel flag early so the time-budget supervisor (if
    // any) can flip it. If the caller did not supply one, allocate a
    // private flag so the supervisor still has a wire to pull.
    let cancel = opts
        .cancel
        .clone()
        .unwrap_or_else(|| Arc::new(AtomicBool::new(false)));
    let _supervisor: Option<TimeBudgetSupervisor> = opts
        .time_budget
        .map(|budget| TimeBudgetSupervisor::spawn(budget, Arc::clone(&cancel)));

    let opts = AnalysisOpts {
        cancel: Some(cancel),
        time_budget: None,
    };
    do_analyze(path, opts)
}

fn do_analyze(path: &Path, opts: AnalysisOpts) -> Result<AudioAttributes, AudioError> {
    // Extension gate — keeps the analyzer from probing arbitrary
    // binaries on a Phase-9 lazy lookup. Ext is checked case-insensitively.
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    if !is_audio_extension(&ext) {
        return Err(AudioError::NotAudio { ext });
    }

    let file = File::open(path).map_err(|e| AudioError::io(path, e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(&ext);

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| AudioError::Probe {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| AudioError::Unsupported("no decodable audio track".into()))?
        .clone();
    let track_id = track.id;
    let params = &track.codec_params;
    let sample_rate = params
        .sample_rate
        .ok_or_else(|| AudioError::Unsupported("track missing sample_rate".into()))?;
    let channels_obj = params
        .channels
        .ok_or_else(|| AudioError::Unsupported("track missing channel layout".into()))?;
    let channels: u16 = channels_obj.count() as u16;
    if channels == 0 {
        return Err(AudioError::Unsupported("track reports 0 channels".into()));
    }
    // Cap defensively. ebur128 supports up to 64 channels; anything
    // higher is either malformed or an exotic atmos / object-audio
    // file we don't aim to measure. A claimed-1024-channel header
    // would otherwise force enormous per-tap allocations before any
    // sandbox catches it.
    const MAX_CHANNELS: u16 = 64;
    if channels > MAX_CHANNELS {
        return Err(AudioError::Unsupported(format!(
            "track reports {channels} channels (limit {MAX_CHANNELS})"
        )));
    }
    let bit_depth = params.bits_per_sample.map(|b| b as u16);
    let codec = codec_to_short_string(params.codec);

    let mut decoder = symphonia::default::get_codecs()
        .make(params, &DecoderOptions::default())
        .map_err(|e| AudioError::Decode {
            packet: 0,
            reason: e.to_string(),
        })?;

    let mut loudness = LoudnessAccumulator::new(channels, sample_rate)?;
    let mut silence = SilenceCounter::new();
    let mut packet_idx = 0usize;
    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    loop {
        // Cooperative cancel — checked once per packet. Symphonia
        // packets are typically 10-50 ms of audio so a multi-hour
        // decode aborts within tens of milliseconds of the flag flip.
        // `Acquire` here pairs with the `Release` flip from
        // `TimeBudgetSupervisor::spawn` (and from any external
        // setter, which is the documented contract on
        // `AnalysisOpts::cancel`).
        if let Some(c) = &opts.cancel
            && c.load(Ordering::Acquire)
        {
            return Err(AudioError::Cancelled);
        }
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(SymphoniaError::ResetRequired) => {
                // Format requires a re-bind (rare; some Ogg streams).
                // Drop the current decoder and rebuild from the same
                // track parameters. The cached `SampleBuffer` was
                // sized from the *previous* decoder's `capacity()` —
                // the new decoder may report a different value, and
                // re-using a too-small buffer panics inside
                // `SampleBuffer::copy_interleaved_*` (debug-assert on
                // capacity). Reset the cached buffer too so the next
                // packet allocates fresh.
                decoder = symphonia::default::get_codecs()
                    .make(params, &DecoderOptions::default())
                    .map_err(|e| AudioError::Decode {
                        packet: packet_idx,
                        reason: e.to_string(),
                    })?;
                sample_buf = None;
                continue;
            }
            Err(e) => {
                return Err(AudioError::Decode {
                    packet: packet_idx,
                    reason: e.to_string(),
                });
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                let frames = decoded.capacity();
                if sample_buf.is_none() {
                    sample_buf = Some(SampleBuffer::<f32>::new(frames as u64, spec));
                }
                if let Some(sb) = sample_buf.as_mut() {
                    sb.copy_interleaved_ref(decoded);
                    let interleaved = sb.samples();
                    silence.add(interleaved);
                    loudness.feed(interleaved)?;
                }
            }
            Err(SymphoniaError::DecodeError(_)) => {
                // Per symphonia docs, decode errors on a packet are
                // recoverable — just skip the packet.
                tracing::debug!(?path, packet_idx, "skipping malformed audio packet");
            }
            Err(e) => {
                return Err(AudioError::Decode {
                    packet: packet_idx,
                    reason: e.to_string(),
                });
            }
        }
        packet_idx += 1;
    }

    let frames = loudness.frames_total();
    if frames == 0 {
        return Err(AudioError::Empty);
    }
    // Saturate the cast — `u64::MAX` ns is ≈ 584 years; any real audio
    // file is a tiny fraction of that, but a malformed header could
    // claim a frame count near `u64::MAX` and overflow the cast. Use
    // `try_into` so the worst case is a saturated duration rather
    // than wraparound.
    let duration_ns_u128 = frames as u128 * 1_000_000_000u128 / sample_rate as u128;
    let duration_ns: u64 = duration_ns_u128.try_into().unwrap_or(u64::MAX);

    let report = loudness.finish()?;
    let dr = if report.short_term_p99.is_finite() && report.short_term_p10.is_finite() {
        report.short_term_p99 - report.short_term_p10
    } else {
        0.0
    };

    Ok(AudioAttributes {
        codec,
        sample_rate,
        channels,
        bit_depth,
        duration_ns,
        lufs_integrated: report.integrated,
        lufs_short_term_p99: report.short_term_p99,
        lufs_short_term_p10: report.short_term_p10,
        peak_dbfs: report.true_peak_dbfs,
        silence_ratio: silence.ratio(),
        dynamic_range_lu: dr,
    })
}

/// Map symphonia's `CodecType` (a NewType around `u32`) to a short
/// lowercase identifier suitable for the `codec:` query modifier. We
/// hand-roll this rather than using symphonia's `CodecRegistry::descriptor`
/// because the registry-derived `short_name` strings vary by feature
/// set; a stable `codec:` key has to match across builds.
fn codec_to_short_string(c: symphonia::core::codecs::CodecType) -> String {
    use symphonia::core::codecs::*;
    match c {
        CODEC_TYPE_PCM_S16LE | CODEC_TYPE_PCM_S16BE => "pcm_s16".into(),
        CODEC_TYPE_PCM_S24LE | CODEC_TYPE_PCM_S24BE => "pcm_s24".into(),
        CODEC_TYPE_PCM_S32LE | CODEC_TYPE_PCM_S32BE => "pcm_s32".into(),
        CODEC_TYPE_PCM_F32LE | CODEC_TYPE_PCM_F32BE => "pcm_f32".into(),
        CODEC_TYPE_PCM_F64LE | CODEC_TYPE_PCM_F64BE => "pcm_f64".into(),
        CODEC_TYPE_PCM_S8 => "pcm_s8".into(),
        CODEC_TYPE_PCM_U8 => "pcm_u8".into(),
        CODEC_TYPE_PCM_S16LE_PLANAR | CODEC_TYPE_PCM_S16BE_PLANAR => "pcm_s16p".into(),
        CODEC_TYPE_PCM_S24LE_PLANAR | CODEC_TYPE_PCM_S24BE_PLANAR => "pcm_s24p".into(),
        CODEC_TYPE_PCM_S32LE_PLANAR | CODEC_TYPE_PCM_S32BE_PLANAR => "pcm_s32p".into(),
        CODEC_TYPE_PCM_F32LE_PLANAR | CODEC_TYPE_PCM_F32BE_PLANAR => "pcm_f32p".into(),
        CODEC_TYPE_PCM_F64LE_PLANAR | CODEC_TYPE_PCM_F64BE_PLANAR => "pcm_f64p".into(),
        CODEC_TYPE_PCM_ALAW => "pcm_alaw".into(),
        CODEC_TYPE_PCM_MULAW => "pcm_mulaw".into(),
        CODEC_TYPE_FLAC => "flac".into(),
        CODEC_TYPE_MP1 => "mp1".into(),
        CODEC_TYPE_MP2 => "mp2".into(),
        CODEC_TYPE_MP3 => "mp3".into(),
        CODEC_TYPE_AAC => "aac".into(),
        CODEC_TYPE_VORBIS => "vorbis".into(),
        // CODEC_TYPE_OPUS — symphonia's `opus` feature is intentionally
        // off in our `Cargo.toml` (the upstream test vectors pulled in
        // GPL-flavored data that conflicts with `cargo-deny`). An
        // Opus-bearing OGG container surfaces as "vorbis"/"unknown"
        // depending on the inner codec parameters; users who need
        // Opus today get a typed `Probe`/`Decode` error rather than a
        // misleading match. We revisit when an MIT-only Opus decoder
        // lands.
        CODEC_TYPE_ALAC => "alac".into(),
        // Unknown codec — surface a stable "unknown" tag so the cache
        // still round-trips a reproducible value. The raw `u32`
        // payload of `CodecType` is private as of symphonia 0.5.5, so
        // we cannot encode it as `u32:<n>`. Future symphonia releases
        // can grow a public accessor; until then any codec we don't
        // map by name surfaces as `unknown`.
        _ => "unknown".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    /// Minimal little-endian WAV writer for tests. Returns a
    /// 16-bit-PCM WAV with the given PCM samples.
    fn write_wav_s16(
        path: &Path,
        sample_rate: u32,
        channels: u16,
        samples: &[i16],
    ) -> std::io::Result<()> {
        let mut f = File::create(path)?;
        let bits_per_sample: u16 = 16;
        let byte_rate = sample_rate * channels as u32 * (bits_per_sample as u32 / 8);
        let block_align: u16 = channels * (bits_per_sample / 8);
        let data_size: u32 = (samples.len() * 2) as u32;
        let chunk_size: u32 = 36 + data_size;
        // RIFF header
        f.write_all(b"RIFF")?;
        f.write_all(&chunk_size.to_le_bytes())?;
        f.write_all(b"WAVE")?;
        // fmt chunk
        f.write_all(b"fmt ")?;
        f.write_all(&16u32.to_le_bytes())?;
        f.write_all(&1u16.to_le_bytes())?; // PCM
        f.write_all(&channels.to_le_bytes())?;
        f.write_all(&sample_rate.to_le_bytes())?;
        f.write_all(&byte_rate.to_le_bytes())?;
        f.write_all(&block_align.to_le_bytes())?;
        f.write_all(&bits_per_sample.to_le_bytes())?;
        // data chunk
        f.write_all(b"data")?;
        f.write_all(&data_size.to_le_bytes())?;
        for s in samples {
            f.write_all(&s.to_le_bytes())?;
        }
        Ok(())
    }

    fn sine_wave_s16(amp: f32, sr: u32, ch: u16, secs: f32) -> Vec<i16> {
        let frames = (sr as f32 * secs) as usize;
        let mut out = Vec::with_capacity(frames * ch as usize);
        let two_pi_f = 2.0 * std::f32::consts::PI * 1000.0 / sr as f32;
        for i in 0..frames {
            let s = (i as f32 * two_pi_f).sin() * amp;
            let int = (s * (i16::MAX as f32 - 1.0))
                .round()
                .clamp(i16::MIN as f32 + 1.0, i16::MAX as f32 - 1.0) as i16;
            for _ in 0..ch {
                out.push(int);
            }
        }
        out
    }

    #[test]
    fn rejects_non_audio_extension() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("blob.txt");
        std::fs::write(&path, b"not audio").unwrap();
        let err = analyze_file(&path).unwrap_err();
        assert!(matches!(err, AudioError::NotAudio { .. }));
    }

    #[test]
    fn analyzes_synthetic_sine_wav_within_lufs_tolerance() {
        // 1 kHz sine at −23 dBFS for 5 s, 44.1 kHz / stereo / 16-bit.
        // EBU R128 K-weighted measurement of a 1 kHz sine maps very
        // close to the linear amplitude — within ±1 LU of −23 LUFS.
        let dir = tempdir().unwrap();
        let path = dir.path().join("sine.wav");
        let samples = sine_wave_s16(10f32.powf(-23.0 / 20.0), 44_100, 2, 5.0);
        write_wav_s16(&path, 44_100, 2, &samples).unwrap();

        let attrs = analyze_file(&path).unwrap();
        assert_eq!(attrs.sample_rate, 44_100);
        assert_eq!(attrs.channels, 2);
        assert_eq!(attrs.codec, "pcm_s16");
        assert!(
            (attrs.length_seconds() - 5.0).abs() < 0.05,
            "duration off: {}",
            attrs.length_seconds()
        );
        assert!(
            (attrs.lufs_integrated - -23.0).abs() < 1.0,
            "integrated: {}",
            attrs.lufs_integrated
        );
        assert!(
            attrs.peak_dbfs > -25.0 && attrs.peak_dbfs < -22.0,
            "peak: {}",
            attrs.peak_dbfs
        );
        assert!(
            attrs.silence_ratio < 0.1,
            "sine should not register as silence: {}",
            attrs.silence_ratio
        );
    }

    #[test]
    fn analyzes_silent_wav_as_silent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("silent.wav");
        let zeros = vec![0i16; 44_100 * 2 * 5]; // 5 s stereo
        write_wav_s16(&path, 44_100, 2, &zeros).unwrap();

        let attrs = analyze_file(&path).unwrap();
        assert!(
            attrs.silence_ratio > 0.99,
            "silence ratio: {}",
            attrs.silence_ratio
        );
        assert_eq!(attrs.lufs_integrated, f32::NEG_INFINITY);
    }

    #[test]
    fn cancel_flag_aborts_decode() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("long.wav");
        // 30 s at 44.1 kHz stereo — long enough that even a fast
        // decode visits multiple packets between cancel checks.
        let samples = sine_wave_s16(0.5, 44_100, 2, 30.0);
        write_wav_s16(&path, 44_100, 2, &samples).unwrap();

        let cancel = Arc::new(AtomicBool::new(true));
        let opts = AnalysisOpts::with_cancel(cancel);
        let err = analyze_with_opts(&path, opts).unwrap_err();
        assert!(matches!(err, AudioError::Cancelled));
    }

    #[test]
    fn time_budget_supervisor_aborts_long_decode() {
        // A 60-second WAV with a pathologically tight time budget.
        // The supervisor flips the cancel flag well before the decode
        // can read all the packets; the analyzer surfaces
        // `AudioError::Cancelled`.
        let dir = tempdir().unwrap();
        let path = dir.path().join("long.wav");
        let samples = sine_wave_s16(0.5, 44_100, 2, 60.0);
        write_wav_s16(&path, 44_100, 2, &samples).unwrap();

        let opts = AnalysisOpts::with_time_budget(Duration::from_millis(100));
        let err = analyze_with_opts(&path, opts).unwrap_err();
        assert!(
            matches!(err, AudioError::Cancelled),
            "expected Cancelled, got {err:?}"
        );
    }

    #[test]
    fn time_budget_supervisor_does_not_fire_on_quick_decode() {
        // A short clip with a generous budget — the analyzer
        // completes well before the supervisor would flip the flag,
        // and `AudioAttributes` come back fine.
        let dir = tempdir().unwrap();
        let path = dir.path().join("short.wav");
        let samples = sine_wave_s16(0.5, 44_100, 1, 0.5);
        write_wav_s16(&path, 44_100, 1, &samples).unwrap();

        let opts = AnalysisOpts::with_time_budget(Duration::from_secs(10));
        analyze_with_opts(&path, opts).expect("quick decode under budget");
    }
}
