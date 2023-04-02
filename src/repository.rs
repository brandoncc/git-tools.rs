use std::path::PathBuf;

use crate::{commands::git_command, utils::is_bare_repo};

#[cfg(test)]
mod tests;

struct Repository;

struct BareRepository {}

struct NormalRepository {}

trait RepositoryInterface {
    fn is_bare(self: &Self) -> bool;
}

impl RepositoryInterface for BareRepository {
    fn is_bare(self: &Self) -> bool {
        true
    }
}

impl RepositoryInterface for NormalRepository {
    fn is_bare(self: &Self) -> bool {
        false
    }
}

impl Repository {
    pub fn at(path: PathBuf) -> Option<Box<dyn RepositoryInterface>> {
        if !path.exists() {
            return None;
        }

        if !is_repo(&path) {
            return None;
        }

        match is_bare_repo(&path) {
            true => Some(Box::new(BareRepository {})),
            false => Some(Box::new(NormalRepository {})),
        }
    }
}

fn is_repo(path: &PathBuf) -> bool {
    git_command(vec!["branch"], path.clone()).is_ok()
}
