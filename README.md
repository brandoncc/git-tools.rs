# git-tools.rs

A collection of tools that interact with git repositories.

## Commands

### clean-merged-branches

Running `git-tools clean-merged-branches` in a git repository directory will delete any branches or worktrees that are
merged into the main or master branch.

## Liability

If you are going to use this tool, please make sure you understand how it works. I do not assume any resposibility if an
unexpected branch or worktree are deleted.

## TODO

- [ ] Add a "dry run" mode so that you can see what will be deleted
- [ ] Remove as many object duplications (`clone()`/`to_owned()`) as possible
