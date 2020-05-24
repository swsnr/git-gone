# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
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

[GH-10]: https://github.com/lunaryorn/git-gone/pull/10

## [0.3.0] – 2019-12-18
### Changed
* Shell out to `git fetch --prune --all` for `-f/--fetch`, and thus use Git's
  mechanisms for authentication (see [GH-5]) and the standard pretty output.
  Hence `git gone -f` now requires `git` in `$PATH`.

[GH-5]: https://github.com/lunaryorn/git-gone/issues/5

## [0.2.0] – 2019-11-30
### Fixed
* Properly detect gone branches by looking at branches that used to have an
  upstream (see [GH-8]).

[GH-8]: https://github.com/lunaryorn/git-gone/pull/8

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

[0.1.0]: https://github.com/lunaryorn/git-gone/releases/tag/v0.1.0
[0.1.1]: https://github.com/lunaryorn/git-gone/compare/v0.1.0...v0.1.1
[0.1.2]: https://github.com/lunaryorn/git-gone/compare/v0.1.1...v0.1.2
[0.2.0]: https://github.com/lunaryorn/git-gone/compare/v0.1.2...v0.2.0
[0.3.0]: https://github.com/lunaryorn/git-gone/compare/vv0.2.0...v0.3.0
[0.3.1]: https://github.com/lunaryorn/git-gone/compare/vv0.3.0...v0.3.1
[0.3.2]: https://github.com/lunaryorn/git-gone/compare/vv0.3.1...v0.3.2
[Unreleased]: https://github.com/lunaryorn/git-gone/compare/v0.3.2...HEAD
