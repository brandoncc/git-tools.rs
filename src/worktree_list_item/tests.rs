use super::WorktreeListItem;

#[test]
fn test_path_output_does_not_include_repo_path() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo/some-work f9e08b4 [some-work]")
    };

    println!("item.path(): {:?}", item.path());
    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_repo_path_can_include_spaces() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo with  spaces"),
        list_item_output: &String::from("/a/repo with  spaces/some-work f9e08b4 [some-work]")
    };

    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_git_hash_can_be_separated_by_multiple_spaces() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo with  spaces"),
        list_item_output: &String::from("/a/repo with  spaces/some-work     f9e08b4 [some-work]")
    };

    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_worktree_path_and_repo_path_can_include_symbols() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo with-_ ^symbols"),
        list_item_output: &String::from("/a/repo with-_ ^symbols/some-work     f9e08b4 [some-work]")
    };

    assert_eq!(Some("some-work".to_string()), item.path());
}

#[test]
fn test_path_is_none_for_the_root_list_item() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo with-_ ^symbols"),
        list_item_output: &String::from("/a/repo with-_ ^symbols    (bare)")
    };

    assert_eq!(None, item.path());
}

#[test]
fn test_name_can_include_symbols() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo/some-work     f9e08b4 [some-_^wo[rk]]")
    };

    assert_eq!(Some("some-_^wo[rk]".to_string()), item.name());
}

#[test]
fn test_name_can_include_spaces() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo/some-work     f9e08b4 [some work]")
    };

    assert_eq!(Some("some work".to_string()), item.name());
}

#[test]
fn test_name_can_include_forward_slashes() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo/some-work     f9e08b4 [some/work]")
    };

    assert_eq!(Some("some/work".to_string()), item.name());
}

#[test]
fn test_name_can_include_backslashes() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo/some-work     f9e08b4 [some\\work]")
    };

    assert_eq!(Some("some\\work".to_string()), item.name());
}

#[test]
fn test_name_is_none_for_the_root_list_item() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo    (bare)")
    };

    assert_eq!(None, item.name());
}

#[test]
fn test_is_bare_is_true_for_the_root_list_item() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo    (bare)")
    };

    assert!(item.is_bare());
}

#[test]
fn test_is_bare_is_false_for_a_non_root_list_item() {
    let item = WorktreeListItem {
        repo_path: &String::from("/a/repo"),
        list_item_output: &String::from("/a/repo/some-work     f9e08b4 [some-work]")
    };

    assert!(!item.is_bare());
}
