# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Break Versioning].

## [Unreleased]

## [v0.2.1] - 2026-03-26

### Fixed

- Install script now downloads the correct checksum file during installation
- Self-update no longer fails with a 404 when fetching the target release

## [v0.2.0] - 2026-03-26

### Added

- Shell completions and man-page generation commands (see [#1])
- Markdown rendering in the terminal with styled headings, code blocks, lists, blockquotes, and more (see [#2], [#3])
- Artifact and task detail views now render descriptions as styled markdown (see [#4], [#5])
- Verbose logging across commands, store operations, and config discovery

### Changed

- `artifact list --include-archived` renamed to `--all` (`-a`), matching task and search commands (**breaking**)
- Faster unique ID prefix computation and shorter ID encoding

### Fixed

- Log level semantics now correctly follow debug=why, trace=result convention

## v0.1.0 - 2026-03-26

Initial release

[Break Versioning]: https://www.taoensso.com/break-versioning
[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/

[#1]: https://github.com/aaronmallen/gest/issues/1
[#2]: https://github.com/aaronmallen/gest/issues/2
[#3]: https://github.com/aaronmallen/gest/issues/3
[#4]: https://github.com/aaronmallen/gest/issues/4
[#5]: https://github.com/aaronmallen/gest/issues/5

[Unreleased]: https://github.com/aaronmallen/gest/compare/0.2.1...main
[v0.2.0]: https://github.com/aaronmallen/gest/compare/0.1.0...0.2.0
[v0.2.1]: https://github.com/aaronmallen/gest/compare/0.2.0...0.2.1
