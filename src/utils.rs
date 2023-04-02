use std::{env, path::PathBuf};

use crate::commands::git_command;

pub fn expand_path(path: String) -> String {
    PathBuf::from(
        path.replace("~", &env::var("HOME").unwrap())
            .replace("$USER", &env::var("USER").unwrap()),
    )
    .to_str()
    .expect("Path is not a valid &str")
    .to_string()
}

pub fn get_current_branch_name(repo_path: &PathBuf) -> String {
    git_command(vec!["branch", "--show-current"], repo_path)
        .expect("Couldn't get current branch")
        .output
        .iter()
        .next()
        .expect("No output found")
        .to_string()
}

pub fn is_bare_repo(cwd: &PathBuf) -> bool {
    get_bare_root(cwd).is_ok()
}

pub fn get_bare_root(cwd: &PathBuf) -> Result<PathBuf, String> {
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
    match git_command(vec!["rev-parse", "--is-bare-repository"], path) {
        Ok(result) => result.output.join("") == "true",
        Err(_) => false,
    }
}

pub fn get_normal_root(cwd: &PathBuf) -> Result<PathBuf, String> {
    match git_command(vec!["rev-parse", "--show-toplevel"], cwd) {
        Ok(result) => Ok(PathBuf::from(result.output.join(""))),
        Err(_) => Err(format!(
            "Error: command must be run within a normal repository (it was run from {:?})",
            cwd
        )),
    }
}
