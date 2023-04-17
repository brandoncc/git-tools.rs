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

- [ ] Removing a worktree should also delete the related branch. This should work for worktrees that have paths which
  don't match their branch name as well.
- [ ] When parsing `git worktree --list`, use the whole path instead of just the relative path. This will remove logic
  that splits out the relative path, and should also support unconventional repository structures. The current
  implementation assumes that worktrees are stored as subdirectories of the repo root, but that doesn't have to be the
  case.
- [ ] Add a "dry run" mode so that you can see what will be deleted
- [ ] Remove as many object duplications (`clone()`/`to_owned()`) as possible
- [ ] Fix CI -- `git checkout`/`git commit` don't seem to work correctly, so the dummy repos don't get setup correctly
- [ ] Allow the tools to run on Windows. Right now, the `/` character is hard-coded as the directory separator.
