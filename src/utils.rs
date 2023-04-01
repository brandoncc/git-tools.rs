use std::{env, path::PathBuf};

use crate::{commands::git_command, worktree::Worktree, CommandWorkingDirectory, Context, worktree_list_item::WorktreeListItem};

const MAIN_BRANCH_NAMES: [&str; 2] = ["main", "master"];

pub fn expand_path(path: String) -> String {
    PathBuf::from(
        path.replace("~", &env::var("HOME").unwrap())
            .replace("$USER", &env::var("USER").unwrap()),
    )
    .to_str()
    .expect("Path is not a valid &str")
    .to_string()
}

fn clean_branch_name(branch: &String) -> String {
    branch.split_whitespace().last().unwrap().to_string()
}

pub fn get_all_branch_names(repo_path: &PathBuf) -> Vec<String> {
    git_command(vec!["branch"], PathBuf::from(repo_path))
        .expect("Couldn't get branch names")
        .output
        .iter()
        .map(clean_branch_name)
        .collect::<Vec<String>>()
}

pub fn get_all_worktrees(context: &Context) -> Result<Vec<Worktree>, String> {
    let worktrees = git_command(vec!["worktree", "list"], PathBuf::from(&context.repo_path))
        .expect("Couldn't get worktree names")
        .output
        .iter()
        .map(|line| WorktreeListItem::new(&(context.repo_path), line))
        .filter_map(|list_item| match list_item.is_bare() || list_item.is_detached() {
            true => None,
            false => Some(Worktree::try_from(&list_item).expect("Couldn't create Worktree from WorktreeListItem"))
        })
        .collect::<Vec<Worktree>>();

    Ok(worktrees)
}

pub fn get_main_branch_name(repo_path: &PathBuf) -> String {
    get_all_branch_names(repo_path)
        .into_iter()
        .filter_map(|branch| {
            if MAIN_BRANCH_NAMES.contains(&branch.as_str()) {
                Some(branch)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .first()
        .expect("No main branch found")
        .to_owned()
}

pub fn get_current_branch_name(repo_path: &PathBuf) -> String {
    git_command(vec!["branch", "--show-current"], repo_path.clone())
        .expect("Couldn't get current branch")
        .output
        .iter()
        .next()
        .expect("No output found")
        .to_string()
}

pub fn is_bare_repo(cwd: &CommandWorkingDirectory) -> bool {
    get_bare_root(cwd).is_ok()
}

pub fn get_bare_root(cwd: &CommandWorkingDirectory) -> Result<PathBuf, String> {
    let mut node = Some(cwd.clone());

    while let Some(n) = node {
        if n.to_str().expect("Expected to find a path string") == "/" {
            return Err(format!("{:?} is not a bare repository", cwd.clone()));
        }

        if is_bare_root(&n) {
            return Ok(n.clone());
        }

        match n.parent() {
            Some(path) => {
                node = Some(PathBuf::from(path));
            }
            None => {
                return Err(format!("{:?} is not a bare repository", cwd.clone()));
            }
        }
    }

    match node {
        Some(n) => Ok(n),
        None => Err(format!("{:?} is not a bare repository", cwd.clone())),
    }
}

fn is_bare_root(path: &PathBuf) -> bool {
    match git_command(vec!["rev-parse", "--is-bare-repository"], path.clone()) {
        Ok(result) => result.output.join("") == "true",
        Err(_) => false,
    }
}

pub fn get_normal_root(cwd: &CommandWorkingDirectory) -> Result<PathBuf, String> {
    match git_command(vec!["rev-parse", "--show-toplevel"], cwd.clone()) {
        Ok(result) => Ok(PathBuf::from(result.output.join(""))),
        Err(_) => Err(format!(
            "Error: command must be run within a normal repository (it was run from {:?})",
            cwd
        )),
    }
}

pub fn merged_branches(
    main_branch_name: &String,
    cwd: &CommandWorkingDirectory,
) -> Result<Vec<String>, String> {
    match git_command(
        vec!["branch", "--merged", main_branch_name.as_str()],
        cwd.clone(),
    ) {
        Ok(result) => Ok(result
            .output
            .iter()
            .map(clean_branch_name)
            .filter_map(|branch| {
                if branch == *main_branch_name {
                    None
                } else {
                    Some(branch)
                }
            })
            .collect::<Vec<String>>()),
        Err(res) => Err(format!(
            "An error occurred while getting merged branch list: {}",
            res.output.join("")
        )),
    }
}
