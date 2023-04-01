use std::path::PathBuf;

use super::WorktreeListItem;

#[test]
fn test_path_output_does_not_include_repo_path() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(&repo_path, "/a/repo/some-work f9e08b4 [some-work]");

    println!("item.path(): {:?}", item.path());
    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_repo_path_can_include_spaces() {
    let repo_path = PathBuf::from("/a/repo with  spaces");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo with  spaces/some-work f9e08b4 [some-work]",
    );

    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_git_hash_can_be_separated_by_multiple_spaces() {
    let repo_path = PathBuf::from("/a/repo with  spaces");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo with  spaces/some-work     f9e08b4 [some-work]",
    );

    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_repo_path_can_include_symbols() {
    let repo_path = PathBuf::from("/a/repo with-_ ^symbols");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo with-_ ^symbols/some-work     f9e08b4 [some-work]",
    );

    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_can_have_a_subdirectory() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/origin/some-work     f9e08b4 [some-work]",
    );

    assert_eq!(Some("origin/some-work".to_string()), item.path());
}

#[test]
fn test_path_is_none_for_the_root_list_item() {
    let repo_path = PathBuf::from("/a/repo with-_ ^symbols");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo with-_ ^symbols    (bare)",
    );

    assert_eq!(None, item.path());
}

#[test]
fn test_name_can_include_symbols() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some-work     f9e08b4 [some-_^wo[rk]]",
    );

    assert_eq!(Some("some-_^wo[rk]".to_string()), item.name());
}

#[test]
fn test_name_can_include_spaces() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some-work     f9e08b4 [some work]",
    );

    assert_eq!(Some("some work".to_string()), item.name());
}

#[test]
fn test_name_can_include_forward_slashes() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some-work     f9e08b4 [some/work]",
    );

    assert_eq!(Some("some/work".to_string()), item.name());
}

#[test]
fn test_name_can_include_backslashes() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some-work     f9e08b4 [some\\work]",
    );

    assert_eq!(Some("some\\work".to_string()), item.name());
}

#[test]
fn test_name_is_none_for_the_root_list_item() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(&repo_path, "/a/repo    (bare)");

    assert_eq!(None, item.name());
}

#[test]
fn test_is_bare_is_true_for_the_root_list_item() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(&repo_path, "/a/repo    (bare)");

    assert!(item.is_bare());
}

#[test]
fn test_is_bare_is_false_for_a_non_root_list_item() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some-work     f9e08b4 [some-work]",
    );

    assert!(!item.is_bare());
}

#[test]
fn test_is_detached_is_true_if_list_item_is_detached() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/dirty                0000000 (detached HEAD)",
    );

    assert!(item.is_detached());
}

#[test]
fn test_is_detached_is_false_if_list_item_is_not_detached() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some-work     f9e08b4 [some-work]",
    );

    assert!(!item.is_detached());
}

#[test]
fn test_prunable_worktree_is_parsed_properly() {
    let repo_path = PathBuf::from("/a/repo");
    let item = WorktreeListItem::new(
        &repo_path,
        "/a/repo/some  work in a prunable worktree    f9e08b4 [some-work] prunable",
    );

    assert_eq!("some-work", item.name().expect("Couldn't parse name"));
    assert_eq!("some  work in a prunable worktree", item.path().expect("Couldn't parse path"));
}
