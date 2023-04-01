use crate::worktree_list_item::WorktreeListItem;

#[derive(Debug, PartialEq, Eq)]
pub struct Worktree {
    pub path: String,
    pub name: String,
}

impl<'a> TryFrom<&WorktreeListItem<'a>> for Worktree {
    type Error = &'static str;

    fn try_from(list_item: &WorktreeListItem) -> Result<Self, Self::Error> {
        if list_item.is_bare() {
            return Err("Can't create a Worktree from a bare WorktreeListItem");
        }

        if list_item.is_detached() {
            return Err("Can't create a Worktree from a detached WorktreeListItem");
        }

        Ok(Self {
            name: list_item.name().expect("Couldn't get list item name"),
            path: list_item.path().expect("Couldn't get list item path"),
        })
    }
}

mod tests;
