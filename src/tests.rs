mod test_helpers;
mod test_setup;

use crate::commands::git_command;

use self::test_helpers::run_test;
use super::*;

#[test]
fn test_merged_branches_are_not_deleted_if_working_tree_is_not_clean() {
    run_test(
        "test_merged_branches_are_not_deleted_if_working_tree_is_not_clean",
        "dirty_repo",
        RepoType::Normal,
        |context| {
            let result = normal_repo::clean_merged_branches(&context);

            assert!(result.is_err());

            test_helpers::assert_branches(
                context,
                vec![
                    "main".to_string(),
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
        RepoType::Normal,
        |context| {
            let result = normal_repo::clean_merged_branches(&context);

            assert!(result.is_ok());

            test_helpers::assert_branch_does_not_exist(context, "merged".to_string());
        },
    );
}

#[test]
fn test_unmerged_branches_are_not_deleted() {
    run_test(
        "test_unmerged_branches_are_not_deleted",
        "clean_repo",
        RepoType::Normal,
        |context| {
            let result = normal_repo::clean_merged_branches(&context);

            assert!(result.is_ok());

            test_helpers::assert_branch_exists(context, "unmerged".to_string());
        },
    );
}

#[test]
fn test_deleting_current_head_branch_leaves_repo_with_main_branch_checked_out() {
    run_test(
        "test_deleting_current_head_branch_leaves_repo_with_main_branch_checked_out",
        "clean_repo",
        RepoType::Normal,
        |context| {
            git_command(vec!["checkout", "merged"], context.repo_path.clone())
                .expect("Failed to checkout merged branch");

            test_helpers::assert_current_branch(&context, "merged".to_string());

            normal_repo::clean_merged_branches(&context).expect("failed to clean merged branches");

            test_helpers::assert_current_branch(&context, "main".to_string());
        },
    );
}

#[test]
fn test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out() {
    run_test(
        "test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out",
        "clean_repo",
        RepoType::Normal,
        |context| {
            git_command(vec!["checkout", "unmerged"], context.repo_path.clone())
                .expect("Failed to checkout unmerged branch");

            test_helpers::assert_current_branch(&context, "unmerged".to_string());

            normal_repo::clean_merged_branches(&context).expect("failed to clean merged branches");

            test_helpers::assert_current_branch(&context, "unmerged".to_string());
        },
    );
}

#[test]
fn test_dirty_worktrees_are_not_removed() {
    run_test(
        "test_dirty_worktrees_are_not_removed",
        test_setup::BARE_REPO_NAME,
        RepoType::Bare,
        |context| {
            bare_repo::clean_merged_worktrees(&context).expect("failed to clean merged branches");

            test_helpers::assert_worktree_exists(context, "dirty".to_string());
        },
    );
}

#[test]
fn test_unmerged_worktrees_are_not_removed() {
    run_test(
        "test_unmerged_worktrees_are_not_removed",
        test_setup::BARE_REPO_NAME,
        RepoType::Bare,
        |context| {
            bare_repo::clean_merged_worktrees(&context).expect("failed to clean merged branches");

            test_helpers::assert_worktree_exists(context, "unmerged".to_string());
        },
    );
}

#[test]
fn test_merged_worktrees_are_removed() {
    run_test(
        "test_merged_worktrees_are_removed",
        test_setup::BARE_REPO_NAME,
        RepoType::Bare,
        |context| {
            bare_repo::clean_merged_worktrees(&context).expect("failed to clean merged branches");

            test_helpers::assert_worktree_does_not_exist(context, "merged".to_string());
        },
    );
}

#[test]
fn test_main_worktree_is_not_removed() {
    run_test(
        "test_main_worktree_is_not_removed",
        test_setup::BARE_REPO_NAME,
        RepoType::Bare,
        |context| {
            bare_repo::clean_merged_worktrees(&context).expect("failed to clean merged branches");

            test_helpers::assert_worktree_exists(context, "main".to_string());
        },
    );
}
