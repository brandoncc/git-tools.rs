use std::{env, path::PathBuf, process::exit};

use crate::utils::expand_path;

mod commands;
mod repository;
mod utils;
mod worktree;
mod worktree_list_item;

#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod test_setup;

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum AvailableCommands {
    CleanMergedBranches,
    Invalid,
}

#[derive(Clone, Debug)]
pub enum RepoType {
    Bare,
    Normal,
}

impl From<String> for AvailableCommands {
    fn from(value: String) -> Self {
        match value.as_str() {
            "clean-merged-branches" => Self::CleanMergedBranches,
            _ => Self::Invalid,
        }
    }
}

fn get_cwd() -> PathBuf {
    match env::var("REPO") {
        Ok(repo) => PathBuf::from(expand_path(repo)),
        Err(_) => env::current_dir().expect("Couldn't get the current working directory"),
    }
}

fn get_command() -> AvailableCommands {
    let args: Vec<String> = env::args().collect();
    AvailableCommands::from(args[1].clone())
}

fn main() {
    let cwd = get_cwd();
    let repo = repository::Repository::at(&cwd)
        .unwrap_or_else(|| panic!("{:#?} is not a valid git repository", cwd));

    match get_command() {
        AvailableCommands::CleanMergedBranches => match repo.clean_merged() {
            Ok(_) => (),
            Err(msg) => {
                println!("Error: {}", msg);
                exit(1);
            }
        },
        _ => {
            println!("Available commands: clean-merged-branches");
            println!("repo path: {:?}", cwd);
            exit(1);
        }
    };
}
