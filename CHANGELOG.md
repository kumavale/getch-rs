# Change Log

## [0.2.0] - 2024-02-03
### Added
- Document for docs (#15)
### Changed
- Reception of 0 bytes is now considered as `EOF` and `Ctrl+Z` is returned (#10)

## [0.1.4] - 2023-08-26
### Fixed
- `Ctrl+Z` now works on windows (#9)

## [0.1.3] - 2023-04-27
### Fixed
- Only set `VIRTUAL_TERINAL_INPUT` as console mode upon init (#7)

## [0.1.2] - 2023-04-26
### Fixed
- Remove `enable_line_input` flag when setting console (#5)

## [0.1.1] - 2023-04-24
### Added
- Supported CSI sequence for Windows (#4)

## [0.1.0] - 2023-02-01
- Initial release
