#[test]
fn test_worktree_can_be_created_from_a_worktree_list_item() {
    let item = super::WorktreeListItem::new(
        "/a/repo",
        "/a/repo/origin/some-work     f9e08b4 [some-work]",
    );
    let worktree = super::Worktree::try_from(&item).expect("Couldn't create a worktree");

    assert_eq!("origin/some-work", worktree.path);
    assert_eq!("some-work", worktree.name);
}

#[test]
fn test_worktree_cannot_be_created_from_a_bare_worktree_list_item() {
    let item = super::WorktreeListItem::new(
        "/a/repo",
        "/a/repo  (bare)",
    );
    super::Worktree::try_from(&item).expect_err("Shouldn't have created a worktree, but did");
}
