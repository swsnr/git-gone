// Copyright 2018 Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

#![deny(warnings)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]

#[macro_use]
extern crate clap;
extern crate git2;

use clap::{Arg, SubCommand};
use git2::*;

/// Iterate over "gone" branches.
///
/// "gone" branches are branches with an upstream branch that does not exist
/// anymore.
fn find_gone_branches(repo: &Repository) -> Result<impl Iterator<Item = Branch>, Error> {
    let local_branches = repo
        .branches(Some(BranchType::Local))?
        .collect::<Result<Vec<(Branch, BranchType)>, Error>>()?;
    Ok(local_branches
        .into_iter()
        .map(|item| item.0)
        .filter(|branch| {
            branch
                .upstream()
                .err()
                .iter()
                .any(|error| error.code() == ErrorCode::NotFound)
        }))
}

/// List all gone branches on standard output.
fn list_gone_branches(repo: &Repository) -> Result<(), Error> {
    for branch in find_gone_branches(repo)? {
        let name = String::from_utf8_lossy(branch.name_bytes()?);
        println!("{}", name);
    }
    Ok(())
}

fn delete_gone_branches(repo: &Repository) -> Result<(), Error> {
    for mut branch in find_gone_branches(repo)? {
        let oid = branch.get().peel_to_commit()?.id();
        // Take a copy of the name cow because "delete()" borrows mutable
        let name = String::from_utf8_lossy(branch.name_bytes()?).to_string();
        branch.delete()?;
        println!(
            "Deleted {0} (restore with `git checkout -b {0} {1}`)",
            name, oid
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Prints detailed progress when fetching"),
        ).arg(
            Arg::with_name("fetch")
                .short("f")
                .long("fetch")
                .help("Fetches and prunes all remotes first"),
        ).subcommand(
            SubCommand::with_name("list")
                .display_order(1)
                .about("Lists gone branches"),
        ).subcommand(
            SubCommand::with_name("delete")
                .display_order(2)
                .about("Deletes gone branches"),
        );

    let matches = app.get_matches();
    let verbose = matches.is_present("verbose");

    let repo = Repository::open_from_env()?;

    if matches.is_present("fetch") {
        for remote_name in repo.remotes()?.iter() {
            let mut options = FetchOptions::new();
            options.prune(FetchPrune::On);
            // Print progress if the user asked for this
            if verbose {
                let mut callbacks = RemoteCallbacks::new();
                callbacks.update_tips(|name, _, new_object| {
                    if new_object.is_zero() {
                        println!("{} gone", name);
                    } else {
                        println!("{} updated to {}", name, new_object);
                    }
                    true
                });
                options.remote_callbacks(callbacks);
            }
            repo.find_remote(remote_name.expect("Non-utf8 remote name found"))?
                .fetch(
                    &[],
                    Some(&mut options),
                    Some("git-gone auto fetch and prune"),
                )?;
        }
    }

    match matches.subcommand_name().unwrap_or("list") {
        "list" => list_gone_branches(&repo)?,
        "delete" => delete_gone_branches(&repo)?,
        name => panic!("Unexpected subcommand: {}", name),
    }

    Ok(())
}
