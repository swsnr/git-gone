// Copyright 2018-2025 Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

#![deny(warnings, clippy::all, clippy::pedantic,
    // Guard against left-over debugging output
    clippy::dbg_macro,
    clippy::unimplemented,
    clippy::use_debug,
    clippy::todo,
    // Do not carelessly ignore errors
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
)]

use anyhow::Result;
use gix::remote::Direction;

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
Licensed under the European Union Public Licence
Report issues to <https://codeberg.org/swsnr/git-gone>."
}

#[derive(Debug, clap::Parser)]
#[command(author, version, about, after_help=after_help())]
struct Args {
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
    let repo =
        gix::ThreadSafeRepository::discover_with_environment_overrides(".")?.to_thread_local();

    let references = repo.references()?;
    let gone_branches = references
        .local_branches()?
        // Discard invalid refs
        .filter_map(std::result::Result::ok)
        // Find all refs that have a configured tracking branch on the remote
        .filter(|reference| {
            if let Some(Ok(refname)) = reference.remote_tracking_ref_name(Direction::Fetch)
                && let Ok(None) = repo.try_find_reference(refname.as_ref())
            {
                return true;
            }
            false
        });

    match args.command.unwrap_or(Command::List) {
        Command::List => {
            for branch in gone_branches {
                println!("{}", branch.name().shorten());
            }
        }
        Command::Prune => {
            for mut branch in gone_branches {
                let oid = branch.peel_to_commit()?.id();
                let name = branch.name().shorten().to_owned();
                match branch.delete() {
                    Ok(()) => {
                        println!("Deleted {name} (restore with `git switch -c {name} {oid}`)");
                    }
                    Err(error) => {
                        eprintln!("Skipped deleting {name} due to {error}");
                    }
                }
            }
        }
    }

    Ok(())
}
