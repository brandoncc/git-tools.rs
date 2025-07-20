use std::path::PathBuf;

use crate::{
    get_cwd,
    repository::{all_branch_names, Repository},
    test_setup::{setup, teardown, BARE_REPO_NAME, DUMMY_REPOS_DIRECTORY},
    utils::get_current_branch_name,
};

pub fn assert_branches(repo: &Repository, branches: Vec<String>) {
    assert_eq!(all_branch_names(repo.root()), branches);
}

pub fn assert_branch_exists(repo: &Repository, branch: String) {
    assert!(all_branch_names(repo.root()).contains(&branch));
}

pub fn assert_branch_does_not_exist(repo: &Repository, branch: String) {
    assert!(!all_branch_names(repo.root()).contains(&branch));
}

pub fn assert_worktree_does_not_exist(repo: &Repository, worktree_name: String) {
    assert!(!worktree_exists(repo, worktree_name))
}

pub fn assert_current_branch(repo: &Repository, branch: String) {
    let current_branch = get_current_branch_name(repo.root());

    assert_eq!(branch, current_branch);
}

pub fn assert_worktree_exists(repo: &Repository, worktree_name: String) {
    assert!(worktree_exists(repo, worktree_name))
}

pub fn run_setup(test_name: &str, bare_repo: bool) {
    match setup(test_name, bare_repo) {
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

pub fn run_test(test_name: &str, repo_directory: &str, test: fn(Repository)) {
    // setup must be run before we create the Repository struct or else the repo doesn't exist
    run_setup(test_name, repo_directory == BARE_REPO_NAME);

    let cwd = get_cwd();
    let cwd_str = cwd.to_str().expect("Couldn't convert cwd to str");
    let repo_path = PathBuf::from(format!(
        "{}/{}/{}/{}",
        cwd_str, DUMMY_REPOS_DIRECTORY, test_name, repo_directory
    ));
    let repository = Repository::at(&repo_path)
        .unwrap_or_else(|| panic!("{:#?} is not a valid git repository", repo_path));

    test(repository);
    run_teardown(test_name);
}

fn worktree_exists(repo: &Repository, worktree_name: String) -> bool {
    match repo {
        Repository::Bare(repo) => {
            let worktrees = repo
                .all_worktrees()
                .expect("Couldn't get list of worktrees");

            worktrees.iter().any(|w| w.name == worktree_name)
        }
        _ => {
            panic!("repo is not a bare repo")
        }
    }
}
