# Changelog

All notable changes to lib-games. Versions follow [Semantic Versioning](https://semver.org/)
(0.x releases may change anything).

## [0.1.3]

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
