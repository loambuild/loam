# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0](https://github.com/loambuild/loam-sdk/compare/loam-cli-v0.6.6...loam-cli-v0.7.0) - 2024-03-22

### Added
- Add new update-env command ([#39](https://github.com/loambuild/loam-sdk/pull/39))
- [**breaking**] rename Owner to Admin ([#37](https://github.com/loambuild/loam-sdk/pull/37))

### Fixed
- create .env if it doesn't exist ([#41](https://github.com/loambuild/loam-sdk/pull/41))

## [0.6.6](https://github.com/loambuild/loam-sdk/compare/loam-cli-v0.6.5...loam-cli-v0.6.6) - 2024-01-09

### Other
- update Cargo.lock dependencies

## [0.6.5](https://github.com/loambuild/loam-sdk/compare/loam-cli-v0.6.4...loam-cli-v0.6.5) - 2023-09-24

### Fixed
- ensure that deps of --package are built before specified package ([#31](https://github.com/loambuild/loam-sdk/pull/31))

## [0.6.4](https://github.com/loambuild/loam-sdk/compare/loam-cli-v0.6.3...loam-cli-v0.6.4) - 2023-09-01

### Fixed
- use versions directly so they get bumped by release ([#24](https://github.com/loambuild/loam-sdk/pull/24))

## [0.6.3](https://github.com/loambuild/loam-sdk/compare/loam-cli-v0.6.2...loam-cli-v0.6.3) - 2023-09-01

### Other
- release ([#15](https://github.com/loambuild/loam-sdk/pull/15))

## [0.6.2](https://github.com/loambuild/loam-sdk/releases/tag/loam-cli-v0.6.2) - 2023-08-31

### Added
- add CLI with build command and update soroban-sdk ([#11](https://github.com/loambuild/loam-sdk/pull/11))

### Fixed
- add versions to workspace deps in root Cargo.toml ([#19](https://github.com/loambuild/loam-sdk/pull/19))
- use actual versions ([#18](https://github.com/loambuild/loam-sdk/pull/18))
- use correct versions ([#17](https://github.com/loambuild/loam-sdk/pull/17))
- added description and license for crates

### Other
- Prepare for publishing ([#12](https://github.com/loambuild/loam-sdk/pull/12))
