use crate::{commands::git_command, test_setup::DEFAULT_BRANCH_NAME, worktree::Worktree};

use self::test_helpers::run_test;
use super::*;

#[test]
fn test_merged_branches_are_not_deleted_if_working_tree_is_not_clean() {
    run_test(
        "test_merged_branches_are_not_deleted_if_working_tree_is_not_clean",
        "dirty_repo",
        |repo| {
            let result = repo.clean_merged();

            assert!(result.is_err());

            test_helpers::assert_branches(
                &repo,
                vec![
                    DEFAULT_BRANCH_NAME.to_string(),
                    "merged".to_string(),
                    "unmerged".to_string(),
                ],
            );
        },
    );
}

#[test]
fn test_merged_branches_are_deleted_if_working_tree_is_clean() {
    run_test(
        "test_merged_branches_are_deleted_if_working_tree_is_clean",
        "clean_repo",
        |repo| {
            let result = repo.clean_merged();

            assert!(result.is_ok());

            test_helpers::assert_branch_does_not_exist(&repo, "merged".to_string());
        },
    );
}

#[test]
fn test_unmerged_branches_are_not_deleted() {
    run_test(
        "test_unmerged_branches_are_not_deleted",
        "clean_repo",
        |repo| {
            let result = repo.clean_merged();

            assert!(result.is_ok());

            test_helpers::assert_branch_exists(&repo, "unmerged".to_string());
        },
    );
}

#[test]
fn test_deleting_current_head_branch_leaves_repo_with_main_branch_checked_out() {
    run_test(
        "test_deleting_current_head_branch_leaves_repo_with_main_branch_checked_out",
        "clean_repo",
        |repo| {
            git_command(vec!["checkout", "merged"], repo.root())
                .expect("Failed to checkout merged branch");

            test_helpers::assert_current_branch(&repo, "merged".to_string());

            repo.clean_merged()
                .expect("failed to clean merged branches");

            test_helpers::assert_current_branch(&repo, DEFAULT_BRANCH_NAME.to_string());
        },
    );
}

#[test]
fn test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out() {
    run_test(
        "test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out",
        "clean_repo",
        |repo| {
            git_command(vec!["checkout", "unmerged"], repo.root())
                .expect("Failed to checkout unmerged branch");

            test_helpers::assert_current_branch(&repo, "unmerged".to_string());

            repo.clean_merged()
                .expect("failed to clean merged branches");

            test_helpers::assert_current_branch(&repo, "unmerged".to_string());
        },
    );
}

#[test]
fn test_dirty_worktrees_are_not_removed() {
    run_test(
        "test_dirty_worktrees_are_not_removed",
        test_setup::BARE_REPO_NAME,
        |repo| {
            repo.clean_merged()
                .expect("failed to clean merged worktrees");

            test_helpers::assert_worktree_exists(&repo, "dirty".to_string());
        },
    );
}

#[test]
fn test_unmerged_worktrees_are_not_removed() {
    run_test(
        "test_unmerged_worktrees_are_not_removed",
        test_setup::BARE_REPO_NAME,
        |repo| {
            repo.clean_merged()
                .expect("failed to clean merged worktrees");

            test_helpers::assert_worktree_exists(&repo, "unmerged".to_string());
        },
    );
}

#[test]
fn test_merged_worktrees_are_removed() {
    run_test(
        "test_merged_worktrees_are_removed",
        test_setup::BARE_REPO_NAME,
        |repo| {
            repo.clean_merged()
                .expect("failed to clean merged worktrees");

            test_helpers::assert_worktree_does_not_exist(&repo, "merged".to_string());
        },
    );
}

#[test]
fn test_branches_are_removed_for_worktrees_that_are_removed() {
    run_test(
        "test_branches_are_removed_for_worktrees_that_are_removed",
        test_setup::BARE_REPO_NAME,
        |repo| {
            test_helpers::assert_branch_exists(&repo, "merged".to_string());

            repo.clean_merged()
                .expect("failed to clean merged worktrees");

            test_helpers::assert_branch_does_not_exist(&repo, "merged".to_string());
        },
    );
}

#[test]
fn test_branches_are_removed_for_worktrees_that_are_removed_when_path_doesnt_match_worktree() {
    run_test(
        "test_branches_are_removed_for_worktrees_that_are_removed_when_path_doesnt_match_worktree",
        test_setup::BARE_REPO_NAME,
        |repo| {
            test_helpers::assert_worktree_exists(&repo, "wont-match-path".to_string());
            test_helpers::assert_branch_exists(&repo, "wont-match-path".to_string());

            repo.clean_merged()
                .expect("failed to clean merged worktrees");

            test_helpers::assert_worktree_does_not_exist(&repo, "wont-match-path".to_string());
            test_helpers::assert_branch_does_not_exist(&repo, "wont-match-path".to_string());
        },
    );
}

#[test]
fn test_main_worktree_is_not_removed() {
    run_test(
        "test_main_worktree_is_not_removed",
        test_setup::BARE_REPO_NAME,
        |repo| {
            repo.clean_merged()
                .expect("failed to clean merged worktrees");

            test_helpers::assert_worktree_exists(&repo, DEFAULT_BRANCH_NAME.to_string());
        },
    );
}

#[test]
fn test_worktree_list_is_parsed_correctly() {
    run_test(
        "test_worktree_list_is_parsed_correctly",
        test_setup::BARE_REPO_NAME,
        |repo| {
            let bare_repo = match repo {
                repository::Repository::Bare(bare_repo) => bare_repo,
                _ => panic!("repo is not bare"),
            };

            let worktrees = bare_repo
                .all_worktrees()
                .expect("Couldn't get all worktrees");
            let library_dir =
                env::current_dir().unwrap_or_else(|_| panic!("Couldn't get library directory"));

            let expected = vec![
                Worktree {
                    name: "dirty".to_string(),
                    path: format!("{}/dummy_repos/test_worktree_list_is_parsed_correctly/bare repo  -_^^ with symbols and spaces/dirty", library_dir.to_str().unwrap()),
                    repository: &bare_repo,
                },
                Worktree {
                    name: DEFAULT_BRANCH_NAME.to_string(),
                    path: format!("{}/dummy_repos/test_worktree_list_is_parsed_correctly/bare repo  -_^^ with symbols and spaces/main", library_dir.to_str().unwrap()),
                    repository: &bare_repo,
                },
                Worktree {
                    name: "merged".to_string(),
                    path: format!("{}/dummy_repos/test_worktree_list_is_parsed_correctly/bare repo  -_^^ with symbols and spaces/merged", library_dir.to_str().unwrap()),
                    repository: &bare_repo,
                },
                Worktree {
                    name: "wont-match-path".to_string(),
                    path: format!("{}/dummy_repos/test_worktree_list_is_parsed_correctly/bare repo  -_^^ with symbols and spaces/origin/doesnt-match-name", library_dir.to_str().unwrap()),
                    repository: &bare_repo,
                },
                Worktree {
                    name: "other-branch".to_string(),
                    path: format!("{}/dummy_repos/test_worktree_list_is_parsed_correctly/bare repo  -_^^ with symbols and spaces/origin/other-branch", library_dir.to_str().unwrap()),
                    repository: &bare_repo,
                },
                Worktree {
                    name: "unmerged".to_string(),
                    path: format!("{}/dummy_repos/test_worktree_list_is_parsed_correctly/bare repo  -_^^ with symbols and spaces/unmerged", library_dir.to_str().unwrap()),
                    repository: &bare_repo,
                },
            ];

            assert_eq!(expected, worktrees);
        },
    );
}
