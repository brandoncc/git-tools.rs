use crate::{commands::git_command, utils::merged_branches, Context};

pub fn clean_merged_branches(context: &Context) -> Result<(), String> {
    let branches = merged_branches(&context.main_branch_name, &context.repo_path)
        .expect("Couldn't get the list of merged branches");

    for branch in branches {
        if worktree_is_clean(context, &branch) {
            match delete_worktree(context, &branch) {
                Ok(_) => (),
                Err(msg) => println!("Couldn't delete worktree '{}', error: {}", branch, msg)
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
