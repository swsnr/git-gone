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

use clap::SubCommand;
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

fn main() -> Result<(), Box<std::error::Error>> {
    let app = app_from_crate!().subcommand(
        SubCommand::with_name("list")
            .display_order(1)
            .about("Lists gone branches"),
    );

    let matches = app.get_matches();

    let repo = Repository::open_from_env()?;
    match matches.subcommand_name() {
        Some("list") => list_gone_branches(&repo)?,
        None => list_gone_branches(&repo)?,
        Some(name) => panic!("Unexpected subcommand: {}", name),
    }

    Ok(())
}
