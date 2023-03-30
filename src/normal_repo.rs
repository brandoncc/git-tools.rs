use crate::{
    commands::git_command,
    utils::{get_current_branch_name, merged_branches},
    Context,
};

pub fn clean_merged_branches(context: &Context) -> Result<(), String> {
    validate_clean_repo(context)?;

    let branches = merged_branches(&context.main_branch_name, &context.repo_path)
        .expect("couldn't get list of merged branches");
    let current_branch = get_current_branch_name(&context.repo_path);

    let mut deleted_current_branch = false;

    git_command(
        vec!["checkout", context.main_branch_name.as_str()],
        context.repo_path.clone(),
    )
    .expect(
        format!(
            "Failed to checkout the '{}' branch",
            context.main_branch_name,
        )
        .as_str(),
    );

    for branch in branches {
        git_command(
            vec!["branch", "-d", branch.as_str()],
            context.repo_path.clone(),
        )
        .expect(format!("An error occurred while deleting the '{}' branch", branch).as_str());

        if branch == current_branch {
            deleted_current_branch = true;
        }

        println!("Deleted branch: {}", branch);
    }

    if !deleted_current_branch {
        git_command(
            vec!["checkout", current_branch.as_str()],
            context.repo_path.clone(),
        )
        .expect(
            format!(
                "Failed to checkout the original branch ({})",
                current_branch
            )
            .as_str(),
        );
    }

    Ok(())
}

fn validate_clean_repo(context: &Context) -> Result<bool, String> {
    match repo_is_clean(&context) {
        Ok(true) => Ok(true),
        Ok(false) => Err("Repository has uncommitted changes, please commit, stash, or delete these changes and then try again".to_string()),
        Err(_) => Err("Repository has uncommitted changes, please commit, stash, or delete these changes and then try again".to_string())
    }
}

fn repo_is_clean(context: &Context) -> Result<bool, String> {
    let result = git_command(vec!["status", "--short"], context.repo_path.clone());

    match result {
        Ok(res) => Ok(res.output.len() == 0),
        Err(res) => Err(format!(
            "An error occurred while checking if the repo was clean: {}",
            res.output.join("")
        )),
    }
}
