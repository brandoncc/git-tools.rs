use crate::{
    commands::git_command,
    utils::get_all_worktree_names,
    Context,
};

pub fn clean_merged_worktrees(context: &Context) -> Result<(), String> {
    let worktrees = get_all_worktree_names(&context.repo_path)
        .expect("Couldn't get the list of merged worktrees");

    for worktree in worktrees {
        if (worktree != context.main_branch_name) && worktree_is_clean(context, &worktree) {
            match delete_worktree(context, &worktree) {
                Ok(_) => println!("Deleted worktree: {}", worktree),
                Err(msg) => println!("Couldn't delete worktree '{}', error: {}", worktree, msg),
            }
        }
    }

    Ok(())
}

fn delete_worktree(context: &Context, branch: &String) -> Result<(), String> {
    match git_command(
        vec!["worktree", "remove", branch.as_str()],
        context.repo_path.clone(),
    ) {
        Ok(_) => Ok(()),
        Err(result) => Err(format!("{}", result.output.join(","))),
    }
}

fn worktree_is_clean(context: &Context, branch: &String) -> bool {
    let result = git_command(vec!["status", "--short"], context.repo_path.join(branch));

    match result {
        Ok(res) => res.output.len() == 0,
        Err(_) => false,
    }
}
