//! TASK-105 smoke — the release pipeline's preconditions.
//!
//! The pipeline itself is four native legs off one tag, and it works. What
//! keeps going wrong is the state of the repository *at the moment the tag
//! is pushed*, and every one of those failures costs a full rebuild
//! because a tag-triggered workflow reads its own definition from the tag:
//! a fix on `main` cannot help a release already tagged.
//!
//! So these assert the things that must be true before tagging, and they
//! run in ordinary CI where a failure costs seconds instead of forty
//! minutes across four legs.
//!
//! The first one is not hypothetical. `release.yml` extracts its notes
//! from `## [<version>]` in `docs/CHANGELOG.md` and **refuses to publish
//! without them** — Build 3 lost four platform jobs to exactly that, after
//! the version was bumped and `## [Unreleased]` was not renamed.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !p.join("Cargo.toml").exists() || !p.join("docs").exists() {
        if !p.pop() {
            panic!("could not locate workspace root from CARGO_MANIFEST_DIR");
        }
    }
    p
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The version the next tag will carry.
fn shipping_version() -> String {
    let conf: serde_json::Value =
        serde_json::from_str(&read("apps/freally-ui/src-tauri/tauri.conf.json"))
            .expect("tauri.conf.json parses");
    conf["version"]
        .as_str()
        .expect("tauri.conf.json has a version")
        .to_string()
}

/// The release notes `release.yml` would extract for `version`.
///
/// Deliberately a re-implementation of the workflow's awk rather than a
/// call into it: the rule *is* the contract, and the point of the test is
/// to fail here rather than in a leg that has already spent eleven minutes
/// compiling. From the `## [<version>]` heading to the next `## `.
fn release_notes_for(version: &str) -> String {
    let changelog = read("docs/CHANGELOG.md");
    let needle = format!("## [{version}]");
    let mut out = Vec::new();
    let mut inside = false;
    for line in changelog.lines() {
        if line.starts_with(&needle) {
            inside = true;
            continue;
        }
        if inside && line.starts_with("## ") {
            break;
        }
        if inside {
            out.push(line);
        }
    }
    // The workflow also strips leading/trailing blanks and a trailing
    // `---` rule; for "is there anything here at all" the trim is enough.
    out.join("\n").trim().to_string()
}

#[test]
fn the_changelog_has_release_notes_for_the_version_being_shipped() {
    let version = shipping_version();
    let notes = release_notes_for(&version);
    assert!(
        !notes.is_empty(),
        "docs/CHANGELOG.md has no `## [{version}]` section with content in it.\n\
         `release.yml` refuses to publish without one, and it fails *after* \
         four legs have built. Before tagging: rename `## [Unreleased]` to \
         `## [{version}] — …` and open a fresh `## [Unreleased]` above it."
    );
}

#[test]
fn the_unreleased_section_is_not_what_gets_published() {
    // The extractor runs from the version heading to the next `## `, so
    // anything left under `[Unreleased]` *below* a version heading is
    // swallowed into that version's published notes and into the updater
    // dialog every user sees. Ordering the headings newest-first is what
    // keeps that from happening.
    let changelog = read("docs/CHANGELOG.md");
    let headings: Vec<&str> = changelog
        .lines()
        .filter(|l| l.starts_with("## ["))
        .collect();
    assert!(
        !headings.is_empty(),
        "docs/CHANGELOG.md has no version headings at all"
    );
    assert!(
        headings[0].starts_with("## [Unreleased]"),
        "`## [Unreleased]` must be the first version heading in the file; \
         found `{}` above it, which means everything under Unreleased is \
         published as part of that release's notes",
        headings[0]
    );
}

#[test]
fn one_tag_builds_every_platform_the_roadmap_promises() {
    // Four legs: Windows, both macOS architectures, Linux. The macOS Intel
    // leg cross-compiles on the arm64 runner rather than waiting for a
    // chronically-scarce macos-13 runner, which is what dropped the Intel
    // .dmg from v0.20.0 entirely.
    let workflow = read(".github/workflows/release.yml");
    for leg in [
        "windows-latest",
        "macos-14",
        "x86_64-apple-darwin",
        "ubuntu-22.04",
    ] {
        assert!(
            workflow.contains(leg),
            "release.yml no longer builds a `{leg}` leg — one tag must still \
             produce artifacts for all three OSes"
        );
    }
}

#[test]
fn the_bundler_is_still_asked_for_every_target() {
    // `"all"` is what produces msi + nsis + dmg + app + deb + rpm +
    // appimage. Narrowing it silently drops formats from the release, and
    // the download page keeps advertising them.
    let conf: serde_json::Value =
        serde_json::from_str(&read("apps/freally-ui/src-tauri/tauri.conf.json")).unwrap();
    assert_eq!(
        conf["bundle"]["targets"],
        serde_json::Value::String("all".into()),
        "bundle.targets must stay \"all\" — see TASK-101 for the seven \
         formats that depends on"
    );
}

#[test]
fn every_file_that_states_the_version_agrees() {
    // These four are read by different tools at different moments. When
    // they disagree the build still succeeds: the assets are named after
    // one of them and the updater manifest after another, so the app
    // downloads an update whose version it does not recognise and offers
    // it again on the next launch, forever.
    let version = shipping_version();
    let cases = [
        ("Cargo.toml", format!("version = \"{version}\"")),
        (
            "apps/freally-ui/src-tauri/Cargo.toml",
            format!("version = \"{version}\""),
        ),
        (
            "apps/freally-ui/package.json",
            format!("\"version\": \"{version}\""),
        ),
    ];
    for (file, needle) in cases {
        assert!(
            read(file).contains(&needle),
            "{file} does not carry version {version} \
             (tauri.conf.json is the one this test reads it from)"
        );
    }
}
