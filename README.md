# git-gone

List and prune "gone" git branches.

A "gone" branch is a local Git branch whose upstream branch no longer exist.
This frequently occurs in a pull request workflow:

1. You create a local branch, push it and create a pull request.
2. A reviewer merges the pull request and deletes the branch on the server.
3. Your local branch still lingers in your clone.

Over time and after many pull request you accumulate many of these branches
which reference long-merged pull requests and serve no further purpose.

git gone can list these branches and also prune them from your clone.

See <https://eed3si9n.com/git-gone-cleaning-stale-local-branches/>.

## Usage

    git gone --help

## Installation

- `cargo install git-gone`  from [crates.io](https://crates.io/crates/git-gone)
- [Binary package for Arch](https://build.opensuse.org/package/show/home:swsnr/git-gone)
- 3rd party packages: <https://repology.org/project/git-gone>

## License

Copyright 2018-2025 Sebastian Wiesner <sebastian@swsnr.de>

Licensed under the EUPL, see <https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12>
