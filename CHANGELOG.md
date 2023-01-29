# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Update dependencies.

## [0.4.3] – 2022-12-01

### Changed
- Update Github repository URL to <https://github.com/swsnr/git-gone>.
- Update dependencies.

## [0.4.2] – 2022-10-09

### Changed

- Use Github again and provide release binaries.
- Update dependencies.

## [0.4.1] – 2022-09-28

### Fixed
- Correctly parse command line arguments (see [CB-14]).

[CB-14]: https://codeberg.org/flausch/git-gone/issues/14

## [0.4.0] – 2022-09-18

### Changed
- Update git2 dependency.

## [0.3.8] – 2022-01-14

### Changed
- Migrate code to <https://codeberg.org/flausch/git-gone/tags>

## [0.3.7] – 2020-07-09
### Changed
- Update dependencies to remove yanked `small_vec` version from build.

### Fixed
- Include source files in crate distributions; fixes building from source with
  `cargo install` (see [GH-12]).

[GH-12]: https://github.com/swsnr/git-gone/issues/12

## [0.3.6] – 2020-06-14
### Changed
- Update dependencies.

## [0.3.5] – 2020-06-14
### Fixed
- Fix checksum calculation for releases.

## [0.3.4] – 2020-06-14
### Changed
- Upload release artifacts in an extra step.
- Include release changelog in release description.
- Add checksums to release workflow.

## [0.3.3] – 2020-05-24
### Added
- Support statically linked builds against musl, and release in this
  configuration.

## [0.3.2] – 2020-04-15
### Fixed
- Include manpage in release packages.

## [0.3.1] – 2020-04-15
### Added
* Add `git-gone(1)` man page (see [GH-10])

### Changed
* Update git2 dependency to 0.13.

[GH-10]: https://github.com/swsnr/git-gone/pull/10

## [0.3.0] – 2019-12-18
### Changed
* Shell out to `git fetch --prune --all` for `-f/--fetch`, and thus use Git's
  mechanisms for authentication (see [GH-5]) and the standard pretty output.
  Hence `git gone -f` now requires `git` in `$PATH`.

[GH-5]: https://github.com/swsnr/git-gone/issues/5

## [0.2.0] – 2019-11-30
### Fixed
* Properly detect gone branches by looking at branches that used to have an
  upstream (see [GH-8]).

[GH-8]: https://github.com/swsnr/git-gone/pull/8

## [0.1.2] – 2019-01-13
### Added
* Build binaries on Travis CI

## [0.1.1] – 2018-09-23
### Added
* Add README to cargo metadata
* Add categories and keywords to cargo metadata

## [0.1.0] – 2018-09-23

Initial release.

### Added

* Add `list` subcommand to list gone branches.
* Add `prune` subcommand to prune gone branches.
* Add `--fetch` flag to fetch all remotes before looking for gone branches.

[Unreleased]: https://github.com/swsnr/git-gone/compare/v0.4.3...HEAD
[0.4.3]: https://github.com/swsnr/git-gone/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/swsnr/git-gone/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/swsnr/git-gone/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/swsnr/git-gone/compare/v0.3.8...v0.4.0
[0.3.8]: https://github.com/swsnr/git-gone/compare/v0.3.7...v0.3.8
[0.3.7]: https://github.com/swsnr/git-gone/compare/v0.3.6...v0.3.7
[0.3.6]: https://github.com/swsnr/git-gone/compare/v0.3.5...v0.3.6
[0.3.5]: https://github.com/swsnr/git-gone/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/swsnr/git-gone/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/swsnr/git-gone/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/swsnr/git-gone/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/swsnr/git-gone/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/swsnr/git-gone/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/swsnr/git-gone/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/swsnr/git-gone/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/swsnr/git-gone/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/swsnr/git-gone/releases/tag/v0.1.0
