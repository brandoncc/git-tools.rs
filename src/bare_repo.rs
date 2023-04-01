use crate::{
    commands::git_command,
    Context, utils::merged_worktrees, worktree::Worktree,
};

pub fn clean_merged_worktrees(context: &Context) -> Result<(), String> {
    let worktrees = merged_worktrees(&context)
        .expect("Couldn't get the list of merged worktrees");

    for worktree in worktrees {
        if (worktree.name != context.main_branch_name) && worktree_is_clean(context, &worktree) {
            match delete_worktree(context, &worktree) {
                Ok(_) => println!("Deleted worktree: {}", worktree.path),
                Err(msg) => println!("Couldn't delete worktree '{}', error: {}", worktree.path, msg),
            }
        }
    }

    Ok(())
}

fn delete_worktree(context: &Context, worktree: &Worktree) -> Result<(), String> {
    match git_command(
        vec!["worktree", "remove", &worktree.path],
        context.repo_path.clone(),
    ) {
        Ok(_) => Ok(()),
        Err(result) => Err(format!("{}", result.output.join(","))),
    }
}

fn worktree_is_clean(context: &Context, worktree: &Worktree) -> bool {
    let result = git_command(vec!["status", "--short"], context.repo_path.join(&worktree.path));

    match result {
        Ok(res) => res.output.len() == 0,
        Err(_) => false,
    }
}
