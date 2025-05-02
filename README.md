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

Copyright Sebastian Wiesner <sebastian@swsnr.de>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
