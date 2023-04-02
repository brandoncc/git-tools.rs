#[cfg(test)]
use std::env::temp_dir;

#[cfg(test)]
use std::fs::create_dir;

#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use crate::repository::{BareRepository, NormalRepository};

#[cfg(test)]
use crate::test_helpers::run_test;

#[cfg(test)]
use crate::test_setup::{BARE_REPO_NAME, CLEAN_NORMAL_REPO_NAME};

#[cfg(test)]
use crate::{Context, RepoType};

#[cfg(test)]
use super::Repository;

#[test]
fn test_repository_at_returns_a_bare_repository_for_the_bare_root_path() {
    run_test(
        "test_repository_at_returns_a_bare_repository_for_the_bare_root_path",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| match Repository::at(&context.repo_path) {
            Some(repo) => assert!(repo.is_bare()),
            _ => panic!("Should have returned a BareRepository, but didn't"),
        },
    );
}

#[test]
fn test_repository_at_returns_a_bare_repository_for_a_valid_repo_subdirectory_path() {
    run_test(
        "test_repository_at_returns_a_bare_repository_for_a_valid_repo_subdirectory_path",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| match Repository::at(&context.repo_path.join("merged")) {
            Some(repo) => assert!(repo.is_bare()),
            _ => panic!("Should have returned a BareRepository, but didn't"),
        },
    );
}

#[test]
fn test_repository_at_returns_a_normal_repository() {
    run_test(
        "test_repository_at_returns_a_normal_repository",
        CLEAN_NORMAL_REPO_NAME,
        &RepoType::Normal,
        |context: Context| match Repository::at(&context.repo_path) {
            Some(repo) => assert!(!repo.is_bare()),
            _ => panic!("Should have returned a BareRepository, but didn't"),
        },
    );
}

#[test]
fn test_repository_at_returns_none_for_an_invalid_path() {
    let path = PathBuf::from("/tmp/invalid-repo-path");
    let repo = Repository::at(&path);

    assert!(repo.is_none());
}

#[test]
fn test_repository_at_returns_none_for_a_non_repo_path() {
    // use /tmp or equivalent because it is guaranteed to exist and also will not be a repo path
    let path = temp_dir();
    let repo = Repository::at(&path);

    assert!(repo.is_none());
}

#[test]
fn test_bare_repository_at_with_subdirectory_has_correct_root() {
    run_test(
        "test_bare_repository_at_with_subdirectory_has_correct_root",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| {
            let path = context.repo_path.join("merged");
            let repo = BareRepository::at(&path)
                .expect(format!("{:#?} is not a valid git repository", &path).as_str());

            assert_eq!(context.repo_path, repo.root);
        },
    );
}

#[test]
fn test_bare_repository_at_with_root_has_correct_root() {
    run_test(
        "test_bare_repository_at_with_root_has_correct_root",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| {
            let repo = BareRepository::at(&context.repo_path).expect(
                format!("{:#?} is not a valid git repository", &context.repo_path).as_str(),
            );

            assert_eq!(context.repo_path, repo.root);
        },
    );
}

#[test]
fn test_bare_repository_at_with_subdirectory_has_correct_main_branch_name() {
    run_test(
        "test_bare_repository_at_with_subdirectory_has_correct_main_branch_name",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| {
            let path = context.repo_path.join("merged");
            let repo = BareRepository::at(&path)
                .expect(format!("{:#?} is not a valid git repository", &path).as_str());

            assert_eq!("main", repo.main_branch_name);
        },
    );
}

#[test]
fn test_bare_repository_at_with_root_has_correct_main_branch_name() {
    run_test(
        "test_bare_repository_at_with_root_has_correct_main_branch_name",
        BARE_REPO_NAME,
        &RepoType::Bare,
        |context: Context| {
            let repo = BareRepository::at(&context.repo_path).expect(
                format!("{:#?} is not a valid git repository", &context.repo_path).as_str(),
            );

            assert_eq!("main", repo.main_branch_name);
        },
    );
}

#[test]
fn test_bare_repository_at_returns_none_for_an_invalid_path() {
    let path = PathBuf::from("/tmp/invalid-repo-path");
    let repo = BareRepository::at(&path);

    assert!(repo.is_none());
}

#[test]
fn test_bare_repository_at_returns_none_for_a_non_repo_path() {
    // use /tmp or equivalent because it is guaranteed to exist and also will not be a repo path
    let path = temp_dir();
    let repo = BareRepository::at(&path);

    assert!(repo.is_none());
}

#[test]
fn test_normal_repository_at_with_subdirectory_has_correct_root() {
    run_test(
        "test_normal_repository_at_with_subdirectory_has_correct_root",
        CLEAN_NORMAL_REPO_NAME,
        &RepoType::Normal,
        |context: Context| {
            let path = context.repo_path.join("subdirectory");
            create_dir(&path).expect("Couldn't create subdirectory");
            let repo = NormalRepository::at(&path)
                .expect(format!("{:#?} is not a valid git repository", &path).as_str());

            assert_eq!(context.repo_path, repo.root);
        },
    );
}

#[test]
fn test_normal_repository_at_with_root_has_correct_root() {
    run_test(
        "test_normal_repository_at_with_root_has_correct_root",
        CLEAN_NORMAL_REPO_NAME,
        &RepoType::Normal,
        |context: Context| {
            let repo = NormalRepository::at(&context.repo_path).expect(
                format!("{:#?} is not a valid git repository", &context.repo_path).as_str(),
            );

            assert_eq!(context.repo_path, repo.root);
        },
    );
}

#[test]
fn test_normal_repository_at_with_subdirectory_has_correct_main_branch_name() {
    run_test(
        "test_normal_repository_at_with_subdirectory_has_correct_main_branch_name",
        CLEAN_NORMAL_REPO_NAME,
        &RepoType::Normal,
        |context: Context| {
            let path = context.repo_path.join("subdirectory");
            create_dir(&path).expect("Couldn't create subdirectory");
            let repo = NormalRepository::at(&path)
                .expect(format!("{:#?} is not a valid git repository", &path).as_str());

            assert_eq!("main", repo.main_branch_name);
        },
    );
}

#[test]
fn test_normal_repository_at_with_root_has_correct_main_branch_name() {
    run_test(
        "test_normal_repository_at_with_root_has_correct_main_branch_name",
        CLEAN_NORMAL_REPO_NAME,
        &RepoType::Normal,
        |context: Context| {
            let repo = NormalRepository::at(&context.repo_path).expect(
                format!("{:#?} is not a valid git repository", &context.repo_path).as_str(),
            );

            assert_eq!("main", repo.main_branch_name);
        },
    );
}

#[test]
fn test_normal_repository_at_returns_none_for_an_invalid_path() {
    let path = PathBuf::from("/tmp/invalid-repo-path");
    let repo = NormalRepository::at(&path);

    assert!(repo.is_none());
}

#[test]
fn test_normal_repository_at_returns_none_for_a_non_repo_path() {
    // use /tmp or equivalent because it is guaranteed to exist and also will not be a repo path
    let path = temp_dir();
    let repo = NormalRepository::at(&path);

    assert!(repo.is_none());
}
