#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use crate::repository::BareRepository;

#[cfg(test)]
use crate::worktree::WorktreeListItem;

#[test]
fn test_worktree_can_be_created_from_a_worktree_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/origin/some-work     f9e08b4 [some-work]".to_string());
    let worktree = super::Worktree::try_from(item).expect("Couldn't create a worktree");

    assert_eq!("/a/repo/origin/some-work", worktree.path);
    assert_eq!("some-work", worktree.name);
}

#[test]
fn test_worktree_cannot_be_created_from_a_bare_worktree_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = super::WorktreeListItem::new(&repo, "/a/repo  (bare)".to_string());
    super::Worktree::try_from(item).expect_err("Shouldn't have created a worktree, but did");
}

#[test]
fn test_worktree_cannot_be_created_from_a_detached_worktree_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = super::WorktreeListItem::new(&repo, "/a/repo  (detached HEAD)".to_string());
    super::Worktree::try_from(item).expect_err("Shouldn't have created a worktree, but did");
}
