use std::{env::current_dir, path::PathBuf};

use crate::{
    test_setup::{setup, teardown},
    utils::{get_all_branch_names, get_current_branch_name, get_all_worktrees},
    Context, RepoType
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

pub fn assert_worktree_does_not_exist(context: Context, worktree_name: String) {
    let worktrees = get_all_worktrees(&context).expect("Couldn't get list of worktrees");

    assert!(!worktrees.iter().any(|w| w.name == worktree_name));
}

pub fn assert_current_branch(context: &Context, branch: String) {
    let current_branch = get_current_branch_name(&context.repo_path);

    assert_eq!(branch, current_branch);
}

pub fn assert_worktree_exists(context: Context, worktree_name: String) {
    let worktrees = get_all_worktrees(&context).expect("Couldn't get list of worktrees");

    assert!(worktrees.iter().any(|w| w.name == worktree_name));
}

pub fn create_context(repo_name: String, repo_type: &RepoType) -> Context {
    Context {
        main_branch_name: String::from("main"),
        repo_path: PathBuf::from(current_dir().unwrap()).join(format!("dummy_repos/{}", repo_name)),
        repo_type: repo_type.to_owned(),
    }
}

pub fn run_setup(test_name: &str, repo_type: &RepoType) {
    match setup(test_name, repo_type) {
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

pub fn run_test(test_name: &str, repo_name: &str, repo_type: &RepoType, test: fn(Context)) {
    run_setup(test_name, repo_type);
    let context = create_context(format!("{}/{}", test_name, repo_name), repo_type);
    test(context);
    run_teardown(test_name);
}
