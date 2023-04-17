#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use crate::repository::BareRepository;

#[cfg(test)]
use super::WorktreeListItem;

#[test]
fn test_path_output_does_not_include_repo_path() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work f9e08b4 [some-work]".to_string());

    assert_eq!(Some("/a/repo/some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_repo_path_can_include_spaces() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo with  spaces"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo with  spaces/some-work f9e08b4 [some-work]".to_string());

    assert_eq!(Some("/a/repo with  spaces/some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_git_hash_can_be_separated_by_multiple_spaces() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo with  spaces"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(
        &repo,
        "/a/repo with  spaces/some-work     f9e08b4 [some-work]".to_string(),
    );

    assert_eq!(Some("/a/repo with  spaces/some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_repo_path_can_include_symbols() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo with-_ ^symbols"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(
        &repo,
        "/a/repo with-_ ^symbols/some-work     f9e08b4 [some-work]".to_string(),
    );

    assert_eq!(Some("/a/repo with-_ ^symbols/some-work".to_string()), item.path());
}

#[test]
fn test_path_can_have_a_subdirectory() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/origin/some-work     f9e08b4 [some-work]".to_string());

    assert_eq!(Some("/a/repo/origin/some-work".to_string()), item.path());
}

#[test]
fn test_path_is_none_for_the_root_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo with-_ ^symbols"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo with-_ ^symbols    (bare)".to_string());

    assert_eq!(None, item.path());
}

#[test]
fn test_name_can_include_symbols() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work     f9e08b4 [some-_^wo[rk]]".to_string());

    assert_eq!(Some("some-_^wo[rk]".to_string()), item.name());
}

#[test]
fn test_name_can_include_spaces() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work     f9e08b4 [some work]".to_string());

    assert_eq!(Some("some work".to_string()), item.name());
}

#[test]
fn test_name_can_include_forward_slashes() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work     f9e08b4 [some/work]".to_string());

    assert_eq!(Some("some/work".to_string()), item.name());
}

#[test]
fn test_name_can_include_backslashes() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work     f9e08b4 [some\\work]".to_string());

    assert_eq!(Some("some\\work".to_string()), item.name());
}

#[test]
fn test_name_is_none_for_the_root_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo    (bare)".to_string());

    assert_eq!(None, item.name());
}

#[test]
fn test_is_bare_is_true_for_the_root_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo    (bare)".to_string());

    assert!(item.is_bare());
}

#[test]
fn test_is_bare_is_false_for_a_non_root_list_item() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work     f9e08b4 [some-work]".to_string());

    assert!(!item.is_bare());
}

#[test]
fn test_is_detached_is_true_if_list_item_is_detached() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(
        &repo,
        "/a/repo/dirty                0000000 (detached HEAD)".to_string(),
    );

    assert!(item.is_detached());
}

#[test]
fn test_is_detached_is_false_if_list_item_is_not_detached() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(&repo, "/a/repo/some-work     f9e08b4 [some-work]".to_string());

    assert!(!item.is_detached());
}

#[test]
fn test_prunable_worktree_is_parsed_properly() {
    let repo = BareRepository {
        root: PathBuf::from("/a/repo"),
        main_branch_name: "main".to_string(),
    };
    let item = WorktreeListItem::new(
        &repo,
        "/a/repo/some  work in a prunable worktree    f9e08b4 [some-work] prunable".to_string(),
    );

    assert_eq!("some-work", item.name().expect("Couldn't parse name"));
    assert_eq!(
        "/a/repo/some  work in a prunable worktree",
        item.path().expect("Couldn't parse path")
    );
}
