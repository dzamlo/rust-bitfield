# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0] - 2024-07-22

### Added
 - Easily Derive Bitwise Operations, thanks to @PokeJofeJr4th

### Breaking Changes
 - The minimum rustc version is now 1.79.0

## [0.15.0] - 2024-04-09

### Added
 - Allow to generate mask for a field

### Breaking Changes
 - The minimum rustc version is now 1.46.0

## [0.14.0] - 2022-07-11

### Added
 - Getters work with immutable Data

### Breaking Changes
 - The minimum rustc version is now 1.31.0
 - The setters of the `BitRange` and `Bit` has been separated in the `BitRangeMut` and `BitMut` traits.

## [0.13.2] - 2019-05-28

### Added
- `from into` can be used in place of `from` to change the input type of the setter. Thanks to @roblabla

[Unreleased]: https://github.com/dzamlo/rust-bitfield/compare/v0.16.0...HEAD
[0.16.0]: https://github.com/dzamlo/rust-bitfield/compare/v0.15.0...v0.16.0
[0.15.0]: https://github.com/dzamlo/rust-bitfield/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/dzamlo/rust-bitfield/compare/v0.13.2...v0.14.0
[0.13.2]: https://github.com/dzamlo/rust-bitfield/compare/v0.13.1...v0.13.2

