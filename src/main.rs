use std::{env, path::PathBuf, process::exit};

use utils::{get_bare_root, get_main_branch_name, get_normal_root, is_bare_repo};

use crate::utils::expand_path;

mod bare_repo;
mod commands;
mod normal_repo;
mod utils;
mod worktree;
mod worktree_list_item;

#[cfg(test)]
mod tests;

pub type CommandWorkingDirectory = PathBuf;

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

#[derive(Debug)]
pub struct Context {
    repo_type: RepoType,
    repo_path: PathBuf,
    main_branch_name: String,
}

impl Context {
    fn create() -> Self {
        let cwd = get_cwd();
        let repo_type = get_repo_type(&cwd);
        let repo_path = get_repo_path(&cwd, &repo_type);
        let main_branch_name = get_main_branch_name(&repo_path);

        Self {
            main_branch_name,
            repo_path,
            repo_type,
        }
    }
}

fn get_cwd() -> CommandWorkingDirectory {
    match env::var("REPO") {
        Ok(repo) => PathBuf::from(expand_path(repo)),
        Err(_) => env::current_dir().expect("Couldn't get the current working directory"),
    }
}

fn get_command() -> AvailableCommands {
    let args: Vec<String> = env::args().collect();
    AvailableCommands::from(args[1].clone())
}

fn get_repo_path(cwd: &CommandWorkingDirectory, repo_type: &RepoType) -> PathBuf {
    match repo_type {
        RepoType::Bare => {
            get_bare_root(cwd).expect("Expected to find a bare repo root, but didn't")
        }
        RepoType::Normal => {
            get_normal_root(cwd).expect("Expected to find a normal repo root, but didn't")
        }
    }
}

fn get_repo_type(repo_path: &PathBuf) -> RepoType {
    match is_bare_repo(&repo_path) {
        true => RepoType::Bare,
        false => RepoType::Normal,
    }
}

fn main() {
    let context = Context::create();

    match get_command() {
        AvailableCommands::CleanMergedBranches => match context.repo_type {
            RepoType::Bare => match bare_repo::clean_merged_worktrees(&context) {
                Ok(_) => (),
                Err(msg) => {
                    println!("Error: {}", msg);
                    exit(1);
                }
            },
            RepoType::Normal => match normal_repo::clean_merged_branches(&context) {
                Ok(_) => (),
                Err(msg) => {
                    println!("Error: {}", msg);
                    exit(1);
                }
            },
        },
        _ => {
            println!("Available commands: clean-merged-branches");
            println!("repo path: {:?}", context.repo_path);
            exit(1);
        }
    };
}
