use std::path::Path;

// This is a way to store one line from `git worktree list` so that it can be easily coerced into a
// Worktree
pub struct WorktreeListItem<'a> {
    repo_path: &'a str,
    list_item_output: &'a str,
}

impl<'a> WorktreeListItem<'a> {
    fn path(&self) -> Option<String> {
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
            .strip_prefix(self.repo_path)
            .expect("Couldn't strip repo path from full path");

        Some(
            relative_path
                .to_str()
                .expect("Couldn't convert path to str")
                .to_string(),
        )
    }

    fn name(&self) -> Option<String> {
        if self.is_bare() {
            return None;
        }

        let name_portion = self.output_split_on_left_square_bracket().1;
        let worktree_name = name_portion
            .strip_suffix(']')
            .expect("Couldn't strip suffix");

        Some(worktree_name.to_string())
    }

    fn is_bare(&self) -> bool {
        self.list_item_output.ends_with("(bare)")
    }

    fn output_split_on_left_square_bracket(&self) -> (&str, &str) {
        self.list_item_output
            .split_once('[')
            .expect("Couldn't split '{}' on '['")
    }
}

mod tests;
