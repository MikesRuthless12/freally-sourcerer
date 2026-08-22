# The Linux half of the TASK-100 perf gate, and the Linux half of clippy.
#
# `ubuntu:22.04` on purpose, not whatever is newest: the release Linux leg
# builds on `ubuntu-22.04`, so matching it keeps the comparison against the
# Windows reference numbers from also carrying a glibc difference. SQLite
# itself is not a variable — `rusqlite` is `bundled`, so the container
# compiles the same amalgamation the Windows run does.
FROM ubuntu:22.04

# The second list is the Tauri Linux build set, copied from `ci.yml`. It is
# here so this container can also run clippy over
# `apps/freally-ui/src-tauri`, which is **excluded from the workspace** and
# is therefore the one crate a Windows-only local CI cannot check for the
# other two platforms.
#
# That gap is not hypothetical: a variable assigned unconditionally but
# read only inside a `#[cfg(windows)]` block is invisible on Windows and
# fails both other legs under `-D warnings`. It happened, and it is why
# these packages are in a bench image.
#
# `libdbus-1-dev` is not in `ci.yml`'s apt list, because the GitHub runner
# image already carries it. A bare ubuntu:22.04 does not, and
# `libdbus-sys` panics in its build script without it.
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential ca-certificates curl pkg-config \
      libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
      librsvg2-dev libssl-dev patchelf libdbus-1-dev \
 && rm -rf /var/lib/apt/lists/*

# Pinned to `rust-toolchain.toml`. `clippy` because of the note above;
# `rustfmt` because the same step in CI runs `cargo fmt -- --check`.
ARG RUST_VERSION=1.97.1
# `-c` once per component: `--component clippy rustfmt` swallows the second
# name as a positional and rustup-init exits 1.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain ${RUST_VERSION} \
        -c clippy -c rustfmt
ENV PATH=/root/.cargo/bin:$PATH

# Out of the bind-mounted source tree, so a Linux build never fights the
# Windows one over `target/`.
ENV CARGO_TARGET_DIR=/target
WORKDIR /src
