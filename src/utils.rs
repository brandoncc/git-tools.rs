use std::{env, path::{PathBuf, Path}};

use crate::commands::git_command;

pub fn expand_path(path: String) -> String {
    PathBuf::from(
        path.replace('~', &env::var("HOME").unwrap())
            .replace("$USER", &env::var("USER").unwrap()),
    )
    .to_str()
    .expect("Path is not a valid &str")
    .to_string()
}

pub fn get_current_branch_name(repo_path: &Path) -> String {
    git_command(vec!["branch", "--show-current"], repo_path)
        .expect("Couldn't get current branch")
        .output.get(0)
        .expect("No output found")
        .to_string()
}

pub fn is_bare_repo(cwd: &Path) -> bool {
    get_bare_root(cwd).is_ok()
}

pub fn get_bare_root(cwd: &Path) -> Result<PathBuf, String> {
    let mut node = Some(cwd.to_path_buf());

    while let Some(n) = node {
        if n.to_str().expect("Expected to find a path string") == "/" {
            return Err(format!("{:?} is not a bare repository", cwd.to_path_buf()));
        }

        if is_bare_root(&n) {
            return Ok(n);
        }

        match n.parent() {
            Some(path) => {
                node = Some(path.to_path_buf());
            }
            None => {
                return Err(format!("{:?} is not a bare repository", cwd.to_path_buf()));
            }
        }
    }

    match node {
        Some(n) => Ok(n),
        None => Err(format!("{:?} is not a bare repository", cwd.to_path_buf())),
    }
}

fn is_bare_root(path: &Path) -> bool {
    match git_command(vec!["rev-parse", "--is-bare-repository"], path) {
        Ok(result) => result.output.join("") == "true",
        Err(_) => false,
    }
}

pub fn get_normal_root(cwd: &Path) -> Result<PathBuf, String> {
    match git_command(vec!["rev-parse", "--show-toplevel"], cwd) {
        Ok(result) => Ok(PathBuf::from(result.output.join(""))),
        Err(_) => Err(format!(
            "Error: command must be run within a normal repository (it was run from {:?})",
            cwd
        )),
    }
}
