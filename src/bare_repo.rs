use crate::{Context, commands::git_command, utils::merged_branches};

pub fn clean_merged_branches(context: &Context) {
    println!("Cleaning merged branches from bare repo");
    let branches = merged_branches(&context.main_branch_name, &context.repo_path);
    // get all merged branches
    //
    // for each branch:
    //   if it is clean:
    //     log that it is being deleted
}

fn branch_is_clean(context: Context, branch: String) -> bool {
    let result = git_command(vec!["status", "--short"], context.repo_path.join(branch));

    match result {
        Ok(res) => res.output.len() == 0,
        Err(res) => false
    }
}
