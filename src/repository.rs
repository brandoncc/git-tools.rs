use std::path::PathBuf;

use crate::{commands::git_command, utils::is_bare_repo, Context, bare_repo, normal_repo};

#[cfg(test)]
mod tests;

pub struct Repository;

struct BareRepository {}

struct NormalRepository {}

pub trait RepositoryInterface {
    fn clean_merged(self: &Self, context: &Context) -> Result<(), String>;
    fn is_bare(self: &Self) -> bool;
}

impl RepositoryInterface for BareRepository {
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

impl Repository {
    pub fn at(path: &PathBuf) -> Option<Box<dyn RepositoryInterface>> {
        if !path.exists() {
            return None;
        }

        if !is_repo(path) {
            return None;
        }

        match is_bare_repo(path) {
            true => Some(Box::new(BareRepository {})),
            false => Some(Box::new(NormalRepository {})),
        }
    }
}

fn is_repo(path: &PathBuf) -> bool {
    git_command(vec!["branch"], path.clone()).is_ok()
}
