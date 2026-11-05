# Changelog

All notable changes to lib-games. Versions follow [Semantic Versioning](https://semver.org/)
(in 0.x, only a new minor version may break the interface; CI's `semver` job enforces it).

## [0.2.0]

### Changed
- **Breaking:** `Found` is `#[non_exhaustive]`; outside this crate make it with `Found::new(game)` and set the rest on
  the result (`base`, `prefix`, `engine`, `aliases`). A field added later is then not a breaking change
  (Droidtop/tracker#867). To move: replace `Found { game, base, prefix, engine }` with `Found::new(game)` and assign
  the other fields.
- Epic games are keyed by Epic's catalog id, `epic:<CatalogItemId>`, droidtop's key for the same game
  (Droidtop/tracker#871). The launcher's `.item` manifests carry it; for legendary and Heroic it comes from legendary's
  `metadata/<AppName>.json` (`metadata.id`). Without one, the key stays `epic:<AppName>`.

### Added
- `Found::aliases`: a game's other keys. An Epic game keyed by its catalog id has `epic:<AppName>`, the key this
  scanner gave it before, so a program can link the old key to the new one.

## [0.1.3]

### Added
- CI's `semver` job: `cargo-semver-checks` against the last release, so a breaking change fails unless its version and
  this CHANGELOG say so (Droidtop/tracker#867).

### Changed
- Built on lib-sync v0.1.2 (cross-store links, Droidtop/tracker#798), so droidtop-agent can take lib-sync v0.1.2 and
  link one sync-core. Nothing else changed.

## [0.1.2]

### Changed
- Built on lib-sync v0.1.1, as 0.1.1 meant to be. 0.1.1 was released with its pins unchanged (still lib-sync
  v0.1.0), so it is the same as 0.1.0; use 0.1.2.

## [0.1.1]

### Changed
- Built on lib-sync v0.1.1 (sync-core 0.1.1: a forgotten device is told it is not paired), so a program
  using this library and lib-sync links one copy of sync-core.

## [0.1.0]

### Added
- `games-scan`: the computer's scanner: stores, launchers, game and ROM folders, installed applications. Moved from droidtop-agent `0ec4110` `crates/scan` unchanged but for its package name (Droidtop/tracker#867 step 1).
