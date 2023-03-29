
use std::{env::current_dir, path::PathBuf, process::Command};

use crate::{
    tests::test_setup::{setup, teardown},
    utils::{get_all_branch_names, get_all_worktree_names},
    Context, RepoType,
};

pub fn assert_branches(context: Context, branches: Vec<String>) {
    assert_eq!(get_all_branch_names(&context.repo_path), branches);
}

pub fn assert_branch_exists(context: Context, branch: String) {
    assert!(get_all_branch_names(&context.repo_path).contains(&branch));
}

pub fn assert_branch_does_not_exist(context: Context, branch: String) {
    assert!(!get_all_branch_names(&context.repo_path).contains(&branch));
}

pub fn assert_worktree_does_not_exist(context: Context, worktree: String) {
    assert!(!get_all_worktree_names(&context.repo_path).contains(&worktree));
}

pub fn assert_current_branch(context: &Context, branch: String) {
    let result = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("Couldn't get the current branch");
    assert_eq!(
        branch,
        String::from_utf8(result.stdout).expect("Couldn't get the stdout")
    );
}

pub fn assert_worktree_exists(context: Context, worktree: String) {
    assert!(get_all_branch_names(&context.repo_path).contains(&worktree));
}

pub fn create_context(repo_name: String, repo_type: RepoType) -> Context {
    Context {
        main_branch_name: repo_name.to_string(),
        repo_path: PathBuf::from(current_dir().unwrap()).join(format!("dummy_repos/{}", repo_name)),
        repo_type,
    }
}

pub fn run_setup(test_name: &str) {
    match setup(test_name) {
        Ok(_) => (),
        Err(msg) => {
            assert!(false, "Test setup failed with error: {}", msg)
        }
    }
}

pub fn run_teardown(test_name: &str) {
    match teardown(test_name) {
        Ok(_) => (),
        Err(msg) => {
            assert!(false, "Test teardown failed with error: {}", msg)
        }
    }
}

pub fn run_test(test_name: &str, repo_name: &str, repo_type: RepoType, test: fn(Context)) {
    run_setup(test_name);
    let context = create_context(format!("{}/{}", test_name, repo_name), repo_type);
    test(context);
    run_teardown(test_name);
}
