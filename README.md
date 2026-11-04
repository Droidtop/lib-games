# lib-games

The library: what a game is, what is installed, and the scanners that find it. One of the shared library families of Droidtop/tracker#867: the libraries belong to no program;
each program has its own scaffold and pins a tagged release of each library it uses (#867 D2).

| Crate | Where | Holds |
|---|---|---|
| `games-scan` | [`crates/scan`](crates/scan) | the computer's scanner: stores, launchers, game and ROM folders, installed applications |

Used by: droidtop-agent (and through it droidtop); Pythia once its native module moves on.

Licence: GPL-3.0-only (see `LICENSE`).

## Where the code came from

#867 step 1 moves existing code before changing it ("reuse and contract first, rewrites second"). These
crates are droidtop-agent's, moved as they were:
  - `games-scan` from droidtop-agent `0ec4110` `crates/scan`

Only the package names changed. Code still imports each crate by its old name (`droidtop_agent_*`), so
nothing that uses it had to be edited; renaming the imports is a later, mechanical change.

## Build and test

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

CI (`.github/workflows/ci.yml`) runs those on Linux, Windows and macOS. It then builds and tests every
program that uses this library against the commit (#867 D2). Work lands on `main` from `ci/*` branches
once CI is green.

## Versions

A release is a tag `vX.Y.Z`. CI creates it on a green `main` whose root `Cargo.toml` names a version not
yet tagged, so to release, raise that version (and the CHANGELOG) in the change that needs it. Programs,
and the other libraries, depend on a tag:

```toml
droidtop-agent-core = { package = "sync-core", git = "https://github.com/Droidtop/lib-sync", tag = "v0.1.0" }
```
