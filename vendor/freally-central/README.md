# `freally-central` — vendored panel source

This is the **view-only `CentralPanel`** React island that Freally
Sourcerer embeds under **Help → More Freally apps…**. `vite.config.ts`
and `vitest.config.ts` both alias `@freally/central-panel` to
`ui/src/panel`, which is the panel's only public entry point.

## Why this is a plain directory and not a submodule

It used to be a git submodule pointing at
`github.com/MikesRuthless12/freally-central`. That repository no longer
exists, so **every CI run failed at checkout** — all three OS legs, on
`ci.yml` and `release.yml` alike, before a single build step ran.

Vendoring the source directly removes the external dependency: this
repository now builds from a clone with no other repository required.

## What was kept

Only `ui/src/panel` (63 files, ~450 KB) plus `LICENSE` and `EULA.md`.
The rest of the original tree — that project's own `src-tauri`, `docs`,
`images`, `crates`, and Cargo manifests — was a whole separate
application and nothing here referenced it.

The panel is self-contained: every relative import inside it resolves
within this directory.

## Updating it

There is no upstream to pull from any more. Edit these files directly,
and keep the `@freally/central-panel` entry point (`ui/src/panel/index.ts`)
stable — that is the contract the alias depends on.
