use crate::{
    commands::git_command, repository::BareRepository, worktree_list_item::WorktreeListItem,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree<'a> {
    pub path: String,
    pub name: String,
    pub repository: &'a BareRepository,
}

impl<'a> TryFrom<WorktreeListItem<'a>> for Worktree<'a> {
    type Error = &'static str;

    fn try_from(list_item: WorktreeListItem<'a>) -> Result<Self, Self::Error> {
        if list_item.is_bare() {
            return Err("Can't create a Worktree from a bare WorktreeListItem");
        }

        if list_item.is_detached() {
            return Err("Can't create a Worktree from a detached WorktreeListItem");
        }

        Ok(Self {
            name: list_item.name().expect("Couldn't get list item name"),
            path: list_item.path().expect("Couldn't get list item path"),
            repository: list_item.repository,
        })
    }
}

impl<'a> Worktree<'a> {
    pub fn delete(&self) -> Result<(), String> {
        match git_command(
            vec!["worktree", "remove", &self.path],
            self.repository.root(),
        ) {
            Ok(_) => Ok(()),
            Err(result) => Err(result.output.join(",")),
        }?;

        match git_command(vec!["branch", "-d", &self.name], self.repository.root()) {
            Ok(_) => Ok(()),
            Err(result) => Err(result.output.join(",")),
        }
    }

    pub fn is_clean(&self) -> bool {
        let result = git_command(
            vec!["status", "--short"],
            &self.repository.root().join(&self.path),
        );

        match result {
            Ok(res) => res.output.is_empty(),
            Err(_) => false,
        }
    }
}

mod tests;
