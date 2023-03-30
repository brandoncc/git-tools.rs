use std::{env::current_dir, path::PathBuf, str::Split};

use crate::{
    tests::test_setup::{setup, teardown},
    utils::{get_all_branch_names, get_current_branch_name},
    Context, RepoType, commands::git_command,
};

fn clean_worktree_name(worktree: &String) -> Option<String> {
    let mut parts: Split<char> = worktree.as_str().split('[');

    if parts.clone().count() < 2 {
        return None;
    }

    let name = parts
        .next_back()
        .expect("Couldn't get second item")
        .strip_suffix(']')
        .expect("Couldn't strip suffix")
        .to_string();

    Some(name)
}

pub fn get_all_worktree_names(repo_path: &PathBuf) -> Vec<String> {
    git_command(vec!["worktree", "list"], PathBuf::from(repo_path))
        .expect("Couldn't get worktree names")
        .output
        .iter()
        .filter_map(clean_worktree_name)
        .collect::<Vec<String>>()
}

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
    let current_branch = get_current_branch_name(&context.repo_path);

    assert_eq!(branch, current_branch);
}

pub fn assert_worktree_exists(context: Context, worktree: String) {
    assert!(get_all_branch_names(&context.repo_path).contains(&worktree));
}

pub fn create_context(repo_name: String, repo_type: RepoType) -> Context {
    Context {
        main_branch_name: String::from("main"),
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
