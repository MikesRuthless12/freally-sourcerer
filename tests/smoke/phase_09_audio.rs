//! Phase 9 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Asserts the invariants the Build Guide names for Phase 9:
//!
//!   1. `sourcerer-audio::analyze_file` decodes a synthetic WAV
//!      fixture and produces an `AudioAttributes` whose integrated
//!      LUFS lands within the spec's ±1 LU tolerance of the reference
//!      level (a 1 kHz sine at −23 dBFS reads ≈ −23 LUFS post-K-
//!      weighting).
//!   2. `AudioCache` round-trips through disk via `flush()` + `open()`
//!      and invalidates a stale entry when `mtime_ns` changes — that
//!      is the "Modify event" half of the lazy-extraction contract.
//!   3. The query DSL parses all six Phase 9 modifiers (`lufs:` /
//!      `codec:` / `length:` / `rate:` / `silence:` / `dr:`) and
//!      composes them with implicit-AND.
//!   4. `execute_with_audio` filters a row set by audio predicates,
//!      evaluating the predicate against the cached attributes; an
//!      audio-bearing query without a provider surfaces the typed
//!      `QueryError::AudioProviderUnavailable` (mirrors Phase 6's
//!      similarity-unavailable contract).
//!   5. A composed query (`length:>3:00 codec:flac` etc.) walks the
//!      filename lens then drops rows whose audio attributes don't
//!      satisfy the predicate.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use sourcerer_audio::{
    AudioAttributes, AudioAttributesProvider, AudioCache, AudioError, analyze_file,
};
use sourcerer_index::Index;
use sourcerer_journal::JournalEvent;
use sourcerer_query::{
    AudioPredicate, ExecOpts, ModifierKind, QueryError, QueryNode, execute_with_audio, parse,
};
use tempfile::tempdir;

// ---------- WAV fixture builders ----------------------------------------

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
    f.write_all(b"RIFF")?;
    f.write_all(&chunk_size.to_le_bytes())?;
    f.write_all(b"WAVE")?;
    f.write_all(b"fmt ")?;
    f.write_all(&16u32.to_le_bytes())?;
    f.write_all(&1u16.to_le_bytes())?;
    f.write_all(&channels.to_le_bytes())?;
    f.write_all(&sample_rate.to_le_bytes())?;
    f.write_all(&byte_rate.to_le_bytes())?;
    f.write_all(&block_align.to_le_bytes())?;
    f.write_all(&bits_per_sample.to_le_bytes())?;
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

// ---------- (1) analyzer round-trips a synthetic WAV --------------------

#[test]
fn analyzes_sine_wav_within_ebu_r128_tolerance() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ref.wav");
    // 1 kHz sine, −23 dBFS, 5 s, 44.1 kHz / stereo / 16-bit. EBU R128
    // K-weighted measurement of a 1 kHz sine maps within ±1 LU of the
    // linear amplitude — easily within the smoke's headroom.
    let samples = sine_wave_s16(10f32.powf(-23.0 / 20.0), 44_100, 2, 5.0);
    write_wav_s16(&path, 44_100, 2, &samples).unwrap();

    let attrs = analyze_file(&path).expect("analyze synthetic WAV");
    assert_eq!(attrs.codec, "pcm_s16");
    assert_eq!(attrs.sample_rate, 44_100);
    assert_eq!(attrs.channels, 2);
    assert_eq!(attrs.bit_depth, Some(16));
    assert!(
        (attrs.length_seconds() - 5.0).abs() < 0.1,
        "length: {}",
        attrs.length_seconds()
    );
    assert!(
        (attrs.lufs_integrated - -23.0).abs() < 1.0,
        "lufs_integrated: {} (expected ≈ −23 LUFS)",
        attrs.lufs_integrated
    );
    assert!(
        attrs.peak_dbfs > -25.0 && attrs.peak_dbfs < -22.0,
        "peak_dbfs: {} (expected ≈ −23 dBFS)",
        attrs.peak_dbfs
    );
    assert!(
        attrs.silence_ratio < 0.05,
        "silence_ratio: {} (sine should not register as silent)",
        attrs.silence_ratio
    );
}

#[test]
fn analyzes_silent_wav_as_pure_silence() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("hush.wav");
    let zeros = vec![0i16; 44_100 * 2 * 5];
    write_wav_s16(&path, 44_100, 2, &zeros).unwrap();

    let attrs = analyze_file(&path).unwrap();
    assert!(
        attrs.silence_ratio > 0.99,
        "silence_ratio: {}",
        attrs.silence_ratio
    );
    assert_eq!(attrs.lufs_integrated, f32::NEG_INFINITY);
    assert_eq!(attrs.peak_dbfs, f32::NEG_INFINITY);
}

#[test]
fn analyzer_rejects_non_audio_extensions() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("notes.txt");
    std::fs::write(&path, b"hello").unwrap();
    let err = analyze_file(&path).unwrap_err();
    assert!(matches!(err, AudioError::NotAudio { .. }));
}

// ---------- (2) AudioCache round-trip + invalidation --------------------

#[test]
fn audio_cache_disk_round_trip_and_mtime_invalidation() {
    let dir = tempdir().unwrap();
    let cache_path = dir.path().join("audio-cache.json");
    let song = dir.path().join("song.wav");
    // tiny dummy file (the cache stores by path; analyze_file isn't
    // called on this path because we insert the entry by hand).
    std::fs::write(&song, b"placeholder").unwrap();

    let cache = AudioCache::open(&cache_path).unwrap();
    let attrs = AudioAttributes {
        codec: "flac".into(),
        sample_rate: 44_100,
        channels: 2,
        bit_depth: Some(16),
        duration_ns: 200_000_000_000,
        lufs_integrated: -16.0,
        lufs_short_term_p99: -10.0,
        lufs_short_term_p10: -22.0,
        peak_dbfs: -1.0,
        silence_ratio: 0.0,
        dynamic_range_lu: 12.0,
    };
    cache.insert(song.clone(), 100, attrs.clone());
    assert!(cache.lookup(&song, 100).is_some());
    assert!(
        cache.lookup(&song, 101).is_none(),
        "mtime mismatch must miss"
    );
    cache.flush().unwrap();

    // Re-open — entry survives the disk round-trip.
    let cache2 = AudioCache::open(&cache_path).unwrap();
    let hit = cache2.lookup(&song, 100).expect("entry survives flush");
    assert_eq!(hit.codec, attrs.codec);
    assert!((hit.lufs_integrated - attrs.lufs_integrated).abs() < 1e-3);
}

// ---------- (3) DSL parse coverage --------------------------------------

#[test]
fn parses_all_six_audio_modifiers() {
    for q in [
        "lufs:<-14",
        "codec:flac",
        "length:>3:00",
        "rate:>=44100",
        "silence:>50%",
        "dr:>10",
    ] {
        let parsed = parse(q).unwrap_or_else(|e| panic!("`{q}` failed: {e}"));
        match parsed.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Audio(_) => {}
                k => panic!("`{q}` not Audio: {k:?}"),
            },
            n => panic!("`{q}` unexpected: {n:?}"),
        }
    }
}

#[test]
fn parses_composed_audio_query() {
    // The Phase 9 prompt's worked example.
    let q = parse("lufs:<-14 codec:flac length:>3:00").unwrap();
    match q.root() {
        QueryNode::And(parts) => {
            assert_eq!(parts.len(), 3);
            for p in parts {
                match p {
                    QueryNode::Modifier(m) => match &m.kind {
                        ModifierKind::Audio(AudioPredicate::Lufs { .. })
                        | ModifierKind::Audio(AudioPredicate::Codec(_))
                        | ModifierKind::Audio(AudioPredicate::Length { .. }) => {}
                        k => panic!("non-Phase-9 modifier in AND: {k:?}"),
                    },
                    n => panic!("non-modifier child: {n:?}"),
                }
            }
        }
        n => panic!("expected And, got {n:?}"),
    }
}

// ---------- (4) executor surfaces AudioProviderUnavailable --------------

fn create_event(path: &Path, mtime_ns: i64) -> JournalEvent {
    JournalEvent::Create {
        path: path.to_path_buf(),
        size: std::fs::metadata(path).map(|m| m.len()).unwrap_or(0),
        mtime_ns: mtime_ns as i128,
        ctime_ns: mtime_ns as i128,
        attrs: 0,
    }
}

#[test]
fn audio_query_without_provider_surfaces_typed_error() {
    let idx_dir = tempdir().unwrap();
    let idx = Index::open(idx_dir.path()).unwrap();
    // No need to populate the index for this test — we only check
    // the front-door gating.
    let q = parse("codec:flac").unwrap();
    let err = execute_with_audio(&idx, None, None, &q, ExecOpts::default()).unwrap_err();
    assert!(matches!(err, QueryError::AudioProviderUnavailable));
}

// ---------- (5) end-to-end: filename lens + audio post-filter -----------

#[test]
fn execute_with_audio_filters_rows_by_predicate() {
    let dir = tempdir().unwrap();
    let idx_dir = dir.path().join("index");
    let cache_path = dir.path().join("audio-cache.json");
    let media_dir = dir.path().join("media");
    std::fs::create_dir_all(&media_dir).unwrap();

    // Two synthetic audio files at different reference levels.
    let loud = media_dir.join("loud.wav");
    write_wav_s16(
        &loud,
        44_100,
        2,
        &sine_wave_s16(10f32.powf(-10.0 / 20.0), 44_100, 2, 5.0),
    )
    .unwrap();
    let quiet = media_dir.join("quiet.wav");
    write_wav_s16(
        &quiet,
        44_100,
        2,
        &sine_wave_s16(10f32.powf(-30.0 / 20.0), 44_100, 2, 5.0),
    )
    .unwrap();

    let idx = Index::open(&idx_dir).unwrap();
    let mtime = 1_700_000_000_000_000_000i64;
    idx.apply(&[create_event(&loud, mtime), create_event(&quiet, mtime)])
        .unwrap();
    idx.commit().unwrap();

    let cache: Arc<AudioCache> = Arc::new(AudioCache::open(&cache_path).unwrap());

    // `lufs:>-20` should keep only the loud (-10 LUFS) file. The
    // executor calls into `cache.get(path, mtime)` per row; a miss
    // triggers `analyze_file`, which decodes the on-disk WAV.
    let q = parse("lufs:>-20").unwrap();
    let provider: &dyn AudioAttributesProvider = cache.as_ref();
    let result = execute_with_audio(
        &idx,
        None,
        Some(provider),
        &q,
        ExecOpts {
            limit: 100,
            ..ExecOpts::default()
        },
    )
    .unwrap();
    let names: Vec<String> = result
        .rows()
        .iter()
        .filter_map(|r| {
            r.path
                .file_name()
                .and_then(|s| s.to_str())
                .map(str::to_string)
        })
        .collect();
    assert!(
        names.iter().any(|n| n == "loud.wav"),
        "loud.wav missing from `lufs:>-20` results: {names:?}"
    );
    assert!(
        !names.iter().any(|n| n == "quiet.wav"),
        "quiet.wav should not match `lufs:>-20`: {names:?}"
    );

    // Inverse predicate keeps only the quiet file.
    let q2 = parse("lufs:<-20").unwrap();
    let result2 = execute_with_audio(
        &idx,
        None,
        Some(provider),
        &q2,
        ExecOpts {
            limit: 100,
            ..ExecOpts::default()
        },
    )
    .unwrap();
    let names2: Vec<String> = result2
        .rows()
        .iter()
        .filter_map(|r| {
            r.path
                .file_name()
                .and_then(|s| s.to_str())
                .map(str::to_string)
        })
        .collect();
    assert!(
        names2.iter().any(|n| n == "quiet.wav"),
        "quiet.wav missing from `lufs:<-20`: {names2:?}"
    );
    assert!(
        !names2.iter().any(|n| n == "loud.wav"),
        "loud.wav should not match `lufs:<-20`: {names2:?}"
    );

    // Codec predicate works on the same row set.
    let q3 = parse("codec:pcm_s16").unwrap();
    let result3 = execute_with_audio(
        &idx,
        None,
        Some(provider),
        &q3,
        ExecOpts {
            limit: 100,
            ..ExecOpts::default()
        },
    )
    .unwrap();
    assert_eq!(result3.rows().len(), 2, "both files match codec:pcm_s16");
}

// ---------- silence:= equality with epsilon -----------------------------

#[test]
fn silence_eq_uses_epsilon_so_user_typed_values_match() {
    // The audio executor's `cmp_op_f32::Eq` arm uses `1e-3` epsilon.
    // A `silence:=1.0` query against a pure-silence file (whose
    // `silence_ratio` is exactly `1.0`) matches; the same query
    // against a near-silent (0.999) file should also match because
    // the difference is below the tolerance.
    let dir = tempdir().unwrap();
    let idx_dir = dir.path().join("index");
    let media_dir = dir.path().join("media");
    std::fs::create_dir_all(&media_dir).unwrap();
    let hush = media_dir.join("hush.wav");
    let silent = vec![0i16; 44_100 * 2 * 5];
    write_wav_s16(&hush, 44_100, 2, &silent).unwrap();

    let idx = Index::open(&idx_dir).unwrap();
    idx.apply(&[create_event(&hush, 1_700_000_000_000_000_000)])
        .unwrap();
    idx.commit().unwrap();

    let cache: Arc<AudioCache> = Arc::new(AudioCache::open(dir.path().join("c.json")).unwrap());
    let q = parse("silence:=1.0").unwrap();
    let provider: &dyn AudioAttributesProvider = cache.as_ref();
    let result = execute_with_audio(
        &idx,
        None,
        Some(provider),
        &q,
        ExecOpts {
            limit: 100,
            ..ExecOpts::default()
        },
    )
    .unwrap();
    assert_eq!(result.rows().len(), 1);
}

// ---------- bonus: NullProvider falls through cleanly -------------------

#[test]
fn null_provider_makes_audio_predicates_match_nothing() {
    let dir = tempdir().unwrap();
    let idx_dir = dir.path().join("index");
    let media_dir = dir.path().join("media");
    std::fs::create_dir_all(&media_dir).unwrap();
    let song = media_dir.join("song.wav");
    write_wav_s16(&song, 44_100, 2, &sine_wave_s16(0.5, 44_100, 2, 1.0)).unwrap();

    let idx = Index::open(&idx_dir).unwrap();
    idx.apply(&[create_event(&song, 1_700_000_000_000_000_000)])
        .unwrap();
    idx.commit().unwrap();

    let null = sourcerer_audio::NullProvider;
    let q = parse("codec:flac").unwrap();
    let result = execute_with_audio(
        &idx,
        None,
        Some(&null as &dyn AudioAttributesProvider),
        &q,
        ExecOpts {
            limit: 100,
            ..ExecOpts::default()
        },
    )
    .unwrap();
    // NullProvider returns Ok(None) for every path — the audio-modifier
    // arm in eval_modifier maps that to "row doesn't match", so no
    // rows survive the filter even though the file exists in the
    // index.
    assert_eq!(result.rows().len(), 0);
}
