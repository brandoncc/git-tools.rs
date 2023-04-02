use std::path::Path;

use crate::repository::{BareRepository, RepositoryInterface};

// This is a way to store one line from `git worktree list` so that it can be easily coerced into a
// Worktree
#[derive(Clone)]
pub struct WorktreeListItem<'a> {
    list_item_output: String,
    pub repository: &'a BareRepository,
}

impl<'a> WorktreeListItem<'a> {
    pub fn new(repository: &'a BareRepository, list_item_output: String) -> Self {
        Self {
            repository,
            list_item_output,
        }
    }

    pub fn path(&self) -> Option<String> {
        if self.is_bare() {
            return None;
        }

        let path_portion = self.output_split_on_left_square_bracket().0;
        let worktree_path = path_portion.trim();
        let absolute_path = worktree_path
            .rsplit_once(' ')
            .expect(
                format!(
                    "Couldn't split on a space, does a space exist? (string: '{}')",
                    worktree_path
                )
                .as_str(),
            )
            .0
            .trim();

        let relative_path = Path::new(absolute_path)
            .strip_prefix(RepositoryInterface::root(self.repository))
            .expect("Couldn't strip repo path from full path");

        Some(
            relative_path
                .to_str()
                .expect("Couldn't convert path to str")
                .to_string(),
        )
    }

    pub fn name(&self) -> Option<String> {
        if self.is_bare() {
            return None;
        }

        let name_portion = self.output_split_on_left_square_bracket().1;
        let worktree_name = name_portion
            .rsplit_once(']')
            .expect("Couldn't split on ']'")
            .0;

        Some(worktree_name.to_string())
    }

    pub fn is_bare(&self) -> bool {
        self.list_item_output.ends_with("(bare)")
    }

    pub fn is_detached(&self) -> bool {
        self.list_item_output.ends_with("(detached HEAD)")
    }

    fn output_split_on_left_square_bracket(&self) -> (&str, &str) {
        self.list_item_output
            .split_once('[')
            .expect(format!("Couldn't split '{}' on '['", self.list_item_output).as_str())
    }
}

mod tests;
