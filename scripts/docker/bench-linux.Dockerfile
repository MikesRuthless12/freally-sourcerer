# The Linux half of the TASK-100 perf gate.
#
# `ubuntu:22.04` on purpose, not whatever is newest: the release Linux leg
# builds on `ubuntu-22.04`, so matching it keeps the comparison against the
# Windows reference numbers from also carrying a glibc difference. SQLite
# itself is not a variable — `rusqlite` is `bundled`, so the container
# compiles the same amalgamation the Windows run does.
FROM ubuntu:22.04

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential ca-certificates curl pkg-config \
 && rm -rf /var/lib/apt/lists/*

# Pinned to `rust-toolchain.toml`. `--profile minimal`: the bench needs
# neither rustfmt nor clippy.
ARG RUST_VERSION=1.97.1
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain ${RUST_VERSION}
ENV PATH=/root/.cargo/bin:$PATH

# Out of the bind-mounted source tree, so a Linux build never fights the
# Windows one over `target/`.
ENV CARGO_TARGET_DIR=/target
WORKDIR /src
