# Changelog

All notable changes to xeli are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning is [SemVer](https://semver.org/).

## [Unreleased]

## [0.1.3] — 2026-05-21

### Changed
- Republishing release after the old GitHub repo was deleted and recreated. `@josharsh/xeli@0.1.2` and `josharsh/homebrew-tap` Formula/xeli.rb both pointed to release artifacts that no longer existed. 0.1.3 is byte-identical code, fresh artifacts.
- `dist-workspace.toml`: added `npm` to `publish-jobs` so the npm registry is updated as part of every tagged release.

## [0.1.2] — 2026-05-20

### Added
- **File picker on no-args**: running `xeli` with no file argument (and no piped stdin) now opens a fuzzy-searchable picker listing supported data files in the current directory, sorted by mtime.
- **Inline AI key entry**: pressing `Ctrl+K` with no API key configured opens a two-step inline flow — pick provider (Anthropic / OpenAI), paste key (masked except last 4 chars), Enter saves to `~/.config/xeli/config.toml` and drops you straight into the AI prompt. No restart, no separate `xeli config` command.
- **Persistent key hints**: the status bar is now two lines — mode + transient message on top, always-visible key hints on the bottom (`Ctrl+K AI · Ctrl+Q SQL · / find · f filter …`). Hints are mode-aware.

### Changed
- `dist-workspace.toml`: added `npm-scope = "@josharsh"` so future cargo-dist runs produce a correctly-scoped npm tarball.

## [0.1.1] — 2026-05-20

### Fixed
- **DuckDB panic on group-by, SQL mode, formula bar, AI/join queries**: `engine::execute_query` called `stmt.column_count()` before the prepared statement had been executed, which panics in duckdb-rs 1.10501 with *"The statement was not executed yet."* Switched to the documented pattern (`query()` first, then read column info via `rows.as_ref().column_count()`). Affected every TUI feature that ran ad-hoc SQL.

## [0.1.0] — 2026-04-10

### Added
- Initial release.
- Multi-format loader: CSV, TSV, JSON, JSONL, Parquet, Excel (xlsx).
- Interactive TUI table with vim-style navigation and mouse support.
- Regex search (`/`), visual filter builder (`f`), click-to-sort (`s`).
- AI natural language → SQL via OpenAI or Anthropic (`Ctrl+K`).
- Direct DuckDB SQL mode (`Ctrl+Q`).
- Column statistics (`Ctrl+I`), histograms (`v`), formula bar (`=`), computed columns (`c`).
- Group-by + pivot wizard (`g`), join wizard (`J`).
- Export filtered data to CSV / JSON / Parquet (`e`).
- 5 themes: Dracula, Nord, Catppuccin, Tokyo Night, Solarized.
- Command palette (`Ctrl+P`), full undo stack (`u`).
- Per-platform binaries via cargo-dist: macOS arm64/x86_64, Linux arm64/x86_64.

[Unreleased]: https://github.com/josharsh/xeli/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/josharsh/xeli/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/josharsh/xeli/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/josharsh/xeli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/josharsh/xeli/releases/tag/v0.1.0
