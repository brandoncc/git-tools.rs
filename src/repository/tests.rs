#[cfg(test)]
use std::env::temp_dir;

#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use crate::test_helpers::run_test;

#[cfg(test)]
use crate::{Context, RepoType};

#[cfg(test)]
use crate::test_setup::BARE_REPO_NAME;

#[cfg(test)]
use crate::test_setup::CLEAN_NORMAL_REPO_NAME;

#[cfg(test)]
use super::Repository;

#[test]
fn test_at_returns_a_bare_repository_for_the_bare_root_path() {
    run_test(
        "test_at_returns_a_bare_repository_for_the_bare_root_path",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| match Repository::at(context.repo_path) {
            Some(repo) => assert!(repo.is_bare()),
            _ => panic!("Should have returned a BareRepository, but didn't"),
        },
    );
}

#[test]
fn test_at_returns_a_bare_repository_for_a_valid_repo_subdirectory_path() {
    run_test(
        "test_at_returns_a_bare_repository_for_a_valid_repo_subdirectory_path",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| match Repository::at(context.repo_path.join("merged")) {
            Some(repo) => assert!(repo.is_bare()),
            _ => panic!("Should have returned a BareRepository, but didn't"),
        },
    );
}

#[test]
fn test_at_returns_a_normal_repository() {
    run_test(
        "test_at_returns_a_normal_repository",
        CLEAN_NORMAL_REPO_NAME,
        &RepoType::Normal,
        |context: Context| match Repository::at(context.repo_path) {
            Some(repo) => assert!(!repo.is_bare()),
            _ => panic!("Should have returned a BareRepository, but didn't"),
        },
    );
}

#[test]
fn test_at_returns_none_for_an_invalid_path() {
    let path = PathBuf::from("/tmp/invalid-repo-path");
    let repo = Repository::at(path);

    assert!(repo.is_none());
}

#[test]
fn test_at_returns_none_for_a_non_repo_path() {
    // use /tmp or equivalent because it is guaranteed to exist and also will not be a repo path
    let path = PathBuf::from(temp_dir());
    let repo = Repository::at(path);

    assert!(repo.is_none());
}
