use std::{any::Any, path::{PathBuf, Path}};

use crate::{
    commands::git_command,
    utils::{get_bare_root, get_current_branch_name, get_normal_root, is_bare_repo},
    worktree::Worktree,
    worktree_list_item::WorktreeListItem,
};

#[cfg(test)]
mod tests;

const MAIN_BRANCH_NAMES: [&str; 2] = ["main", "master"];

pub struct Repository;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BareRepository {
    pub main_branch_name: String,
    pub root: PathBuf,
}

struct NormalRepository {
    main_branch_name: String,
    root: PathBuf,
}

pub trait RepositoryInterface {
    fn as_any(&self) -> &dyn Any;
    fn clean_merged(&self) -> Result<(), String>;
    fn is_bare(&self) -> bool;
    fn root(&self) -> &PathBuf;
    fn main_branch_name(&self) -> &String;
}

impl<'a> RepositoryInterface for BareRepository {
    fn clean_merged(&self) -> Result<(), String> {
        let worktrees = self
            .merged_worktrees()
            .expect("Couldn't get the list of merged worktrees");

        for worktree in worktrees {
            if worktree.is_clean() {
                if worktree.name != self.main_branch_name {
                    match worktree.delete() {
                        Ok(_) => println!("Deleted worktree: {}", worktree.path),
                        Err(msg) => println!(
                            "Couldn't delete worktree '{}', error: {}",
                            worktree.path, msg
                        ),
                    }
                }
            } else {
                println!(
                    "Couldn't delete worktree '{}' ({}) because it contains unstaged changes",
                    worktree.name, worktree.path
                );
            }
        }

        Ok(())
    }

    fn is_bare(&self) -> bool {
        true
    }

    fn root(&self) -> &PathBuf {
        &self.root
    }

    fn main_branch_name(&self) -> &String {
        &self.main_branch_name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RepositoryInterface for NormalRepository {
    fn clean_merged(&self) -> Result<(), String> {
        self.validate_cleanliness()?;

        let branches = merged_branches(&self.main_branch_name, &self.root)
            .expect("couldn't get list of merged branches");
        let current_branch = get_current_branch_name(&self.root);

        let mut deleted_current_branch = false;

        git_command(vec!["checkout", &self.main_branch_name], &self.root)
            .unwrap_or_else(|_| panic!("Failed to checkout the '{}' branch", self.main_branch_name));

        for branch in branches {
            git_command(vec!["branch", "-d", branch.as_str()], &self.root).unwrap_or_else(|_| panic!("An error occurred while deleting the '{}' branch", branch));

            if branch == current_branch {
                deleted_current_branch = true;
            }

            println!("Deleted branch: {}", branch);
        }

        if !deleted_current_branch {
            git_command(vec!["checkout", current_branch.as_str()], &self.root).unwrap_or_else(|_| panic!("Failed to checkout the original branch ({})",
                    current_branch));
        }

        Ok(())
    }

    fn is_bare(&self) -> bool {
        false
    }

    fn root(&self) -> &PathBuf {
        &self.root
    }

    fn main_branch_name(&self) -> &String {
        &self.main_branch_name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BareRepository {
    pub fn at(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        if !is_repo(path) {
            return None;
        }

        Some(Self {
            main_branch_name: find_main_branch_name(path),
            root: get_bare_root(path).expect("Expected to find a bare repo root, but didn't"),
        })
    }

    pub fn all_worktrees(&self) -> Result<Vec<Worktree>, String> {
        let worktrees = git_command(vec!["worktree", "list"], &self.root)
            .expect("Couldn't get worktree names")
            .output
            .into_iter()
            .map(|line| WorktreeListItem::new(self, line))
            .filter_map(
                |list_item| match list_item.is_bare() || list_item.is_detached() {
                    true => None,
                    false => Some(
                        Worktree::try_from(list_item)
                            .expect("Couldn't create Worktree from WorktreeListItem"),
                    ),
                },
            )
            .collect::<Vec<Worktree>>();

        Ok(worktrees)
    }

    fn merged_worktrees(&self) -> Result<Vec<Worktree>, String> {
        let merged = merged_branches(&self.main_branch_name, &self.root)
            .expect("Couldn't get merged branches");
        let all = self.all_worktrees().expect("Couldn't get all worktrees");
        let not_merged = all
            .into_iter()
            .filter(|w| merged.contains(&w.name))
            .collect::<Vec<Worktree>>();

        Ok(not_merged)
    }
}

impl NormalRepository {
    pub fn at(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        if !is_repo(path) {
            return None;
        }

        Some(Self {
            main_branch_name: find_main_branch_name(path),
            root: get_normal_root(path).expect("Expected to find a repo root, but didn't"),
        })
    }

    fn validate_cleanliness(&self) -> Result<bool, String> {
        match self.is_clean() {
            Ok(true) => Ok(true),
            Ok(false) => Err("Repository has uncommitted changes, please commit, stash, or delete these changes and then try again".to_string()),
            Err(_) => Err("Repository has uncommitted changes, please commit, stash, or delete these changes and then try again".to_string())
        }
    }

    fn is_clean(&self) -> Result<bool, String> {
        let result = git_command(vec!["status", "--short"], &self.root);

        match result {
            Ok(res) => Ok(res.output.is_empty()),
            Err(res) => Err(format!(
                "An error occurred while checking if the repo was clean: {}",
                res.output.join("")
            )),
        }
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
            true => Some(Box::new(BareRepository::at(path).unwrap_or_else(|| panic!("{:#?} is not a valid git repository", path)))),
            false => Some(Box::new(NormalRepository::at(path).unwrap_or_else(|| panic!("{:#?} is not a valid git repository", path)))),
        }
    }
}

fn is_repo(path: &Path) -> bool {
    git_command(vec!["branch"], path).is_ok()
}

fn clean_branch_name(branch: &String) -> String {
    branch.split_whitespace().last().unwrap().to_string()
}

fn merged_branches(main_branch_name: &String, repo_path: &Path) -> Result<Vec<String>, String> {
    match git_command(
        vec!["branch", "--merged", main_branch_name.as_str()],
        repo_path,
    ) {
        Ok(result) => Ok(result
            .output
            .iter()
            .map(clean_branch_name)
            .filter(|branch| branch.to_string() != *main_branch_name)
            .collect::<Vec<String>>()),
        Err(res) => Err(format!(
            "An error occurred while getting merged branch list: {}",
            res.output.join("")
        )),
    }
}

fn find_main_branch_name(repo_path: &Path) -> String {
    all_branch_names(repo_path)
        .into_iter()
        .filter(|branch| MAIN_BRANCH_NAMES.contains(&branch.as_str()))
        .collect::<Vec<String>>()
        .first()
        .expect("No main branch found")
        .to_owned()
}

pub fn all_branch_names(repo_path: &Path) -> Vec<String> {
    git_command(vec!["branch"], repo_path)
        .expect("Couldn't get branch names")
        .output
        .iter()
        .map(clean_branch_name)
        .collect::<Vec<String>>()
}
