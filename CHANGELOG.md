# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
* Properly detect gone branches by looking at branches that used to have an
  upstream.

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
[Unreleased]: https://github.com/lunaryorn/git-gone/compare/v0.1.2...HEAD
