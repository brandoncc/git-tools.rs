use std::path::PathBuf;

use crate::{
    bare_repo,
    commands::git_command,
    normal_repo,
    utils::{get_bare_root, get_main_branch_name, is_bare_repo},
    Context,
};

#[cfg(test)]
mod tests;

pub struct Repository;

struct BareRepository {
    main_branch_name: String,
    root: PathBuf,
}

struct NormalRepository {}

pub trait RepositoryInterface {
    fn clean_merged(self: &Self, context: &Context) -> Result<(), String>;
    fn is_bare(self: &Self) -> bool;
}

impl<'a> RepositoryInterface for BareRepository {
    fn clean_merged(self: &Self, context: &Context) -> Result<(), String> {
        bare_repo::clean_merged_worktrees(context)
    }

    fn is_bare(self: &Self) -> bool {
        true
    }
}

impl RepositoryInterface for NormalRepository {
    fn clean_merged(self: &Self, context: &Context) -> Result<(), String> {
        normal_repo::clean_merged_branches(context)
    }

    fn is_bare(self: &Self) -> bool {
        false
    }
}

impl BareRepository {
    pub fn at(path: &PathBuf) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        if !is_repo(path) {
            return None;
        }

        Some(Self {
            main_branch_name: get_main_branch_name(path),
            root: get_bare_root(path).expect("Expected to find a bare repo root, but didn't"),
        })
    }
}

impl Repository {
    pub fn at(path: &PathBuf) -> Option<Box<dyn RepositoryInterface>> {
        if !path.exists() {
            return None;
        }

        if !is_repo(path) {
            return None;
        }

        match is_bare_repo(path) {
            true => Some(Box::new(BareRepository::at(path).expect(
                format!("{:#?} is not a valid git repository", path).as_str(),
            ))),
            false => Some(Box::new(NormalRepository {})),
        }
    }
}

fn is_repo(path: &PathBuf) -> bool {
    git_command(vec!["branch"], path.clone()).is_ok()
}
