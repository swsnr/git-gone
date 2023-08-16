// Copyright 2018-2020 Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

#![deny(warnings, clippy::all)]

use git2::*;

/// Whether a branch has an upstream.
///
/// Try to get the name of the upstream branch of the given branch.  If it
/// exists the branch has an upstream, otherwise it hasn't.
fn has_upstream(repo: &Repository, branch: &Branch) -> bool {
    // branch_upstream_name expects the full refname (e.g. refs/heads/…) so
    // we move the underlying reference with .get().
    branch
        .get()
        .name()
        .map_or(false, |refname| repo.branch_upstream_name(refname).is_ok())
}

/// Iterate over gone branches.
///
/// Find all branches which still have an upstream branch configured, but
/// whose upstream doesn't exist anymore.  These branches had an upstream once
/// probably because they were pushed, but the upstream is gone, e.g. was
/// deleted on the remote.
fn find_gone_branches(repo: &Repository) -> Result<impl Iterator<Item = Branch<'_>>, Error> {
    let local_branches = repo
        .branches(Some(BranchType::Local))?
        .collect::<Result<Vec<(Branch<'_>, BranchType)>, Error>>()?;
    Ok(local_branches
        .into_iter()
        .map(|item| item.0)
        // Look at branches which have an upstream branch…
        .filter(move |branch| has_upstream(repo, branch))
        // …and if that upstream doesn't exist anymore, we have a branch which
        // had an upstream once, but no more, so the upstream’s gone.
        .filter(|branch| {
            branch
                .upstream()
                .err()
                .iter()
                .any(|error| error.code() == ErrorCode::NotFound)
        }))
}

/// List all gone branches from the given repo on standard output.
fn list_gone_branches(repo: &Repository) -> Result<(), Error> {
    for branch in find_gone_branches(repo)? {
        let name = String::from_utf8_lossy(branch.name_bytes()?);
        println!("{name}");
    }
    Ok(())
}

/// Prune gone branches from the given repository.
fn prune_gone_branches(repo: &Repository) -> Result<(), Error> {
    for mut branch in find_gone_branches(repo)? {
        let oid = branch.get().peel_to_commit()?.id();
        // Take a copy of the name cow because "delete()" borrows mutable
        let name = String::from_utf8_lossy(branch.name_bytes()?).to_string();
        match branch.delete() {
            Ok(()) => {
                println!("Deleted {name} (restore with `git checkout -b {name} {oid}`)");
            }
            Err(e) => {
                eprintln!("Skipped deleting {name} due to {e:?}");
            }
        };
    }
    Ok(())
}

fn fetch_branches(verbose: bool) -> Result<(), std::io::Error> {
    let mut command = std::process::Command::new("git");
    command.arg("fetch").arg("--prune").arg("--all");
    if !verbose {
        command.arg("--quiet");
    }
    command
        .spawn()
        .and_then(|mut c| c.wait())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                use std::io::{Error, ErrorKind};
                Err(Error::new(
                    ErrorKind::Other,
                    "git fetch --prune --all failed",
                ))
            }
        })
}

fn after_help() -> &'static str {
    "\
A \"gone\" branch is a local Git branch whose upstream branch no longer exist.
This frequently occurs in a pull request workflow:

  1. You create a local branch, push it and create a pull request.
  2. A reviewer merges the pull request and deletes the branch on the server.
  3. Your local branch still lingers in your clone.

Over time and after many pull request you accumulate many of these branches
which reference long-merged pull requests and serve no further purpose.

git gone can list these branches and also prune them from your clone.

Copyright (C) 2018–2020 Sebastian Wiesner
Licensed under the Apache License, Version 2.0
Report issues to <https://github.com/swsnr/git-gone>."
}

#[derive(Debug, clap::Parser)]
#[command(author, version, about, after_help=after_help())]
struct Args {
    /// Fetch and prune all remotes first.
    #[arg(short, long, global = true)]
    fetch: bool,
    /// Print detailed progress when fetching
    #[arg(short, long, global = true)]
    verbose: bool,
    /// The command.  Defaults to "list".
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// List all gone branches.  Default.
    List,
    /// Delete all gone branches.
    Prune,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let args = Args::parse();
    let repo = Repository::open_from_env()?;

    if args.fetch {
        fetch_branches(args.verbose)?;
    }

    match args.command.unwrap_or(Command::List) {
        Command::List => list_gone_branches(&repo)?,
        Command::Prune => prune_gone_branches(&repo)?,
    }

    Ok(())
}
