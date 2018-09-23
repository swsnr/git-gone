# git-gone

[![Current release]( https://img.shields.io/crates/v/git-gone.svg)](https://crates.io/crates/git-gone)
[![Build status](https://img.shields.io/travis/com/lunaryorn/git-gone/master.svg)](https://travis-ci.com/lunaryorn/git-gone)
[![Windows build status](https://img.shields.io/appveyor/ci/lunaryorn/git-gone/master.svg)](https://ci.appveyor.com/project/lunaryorn/git-gone)

`git-gone` lists and prunes “gone” branches.

A “gone” branch is a local branch whose upstream branch no longer exists. This
frequently occurs in a pull request workflow:

1. You create a local branch, push it and create a pull request.
2. A reviewer merges the pull request and deletes the branch on the server.
3. Your local branch still lingers in your clone.

Over time and after many pull request you accumulate many of these branches
which reference long-merged pull requests and serve no further purpose.

git gone can list these branches and automatically prune them from your clone to
cleanup the debris of pull requests.

## Usage

To fetch and prune and then delete all gone branches:

```console
$ git gone -fv prune
```

The command prints undo information, so you can always restore a branch if you’d
like to keep it.

Alternatively fetch first and then just prune gone branches:

```console
$ git fetch --all --prune
$ git gone prune
```

## Install

Install from [crates.io](https://crates.io/crates/git-gone)

```console
$ cargo install git-gone
```

## License

Copyright 2018 Sebastian Wiesner <sebastian@swsnr.de>

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at <http://www.apache.org/licenses/LICENSE-2.0>.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
