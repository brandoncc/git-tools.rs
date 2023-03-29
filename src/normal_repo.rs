use std::process::exit;

use crate::{commands::git_command, utils::merged_branches, Context};

pub fn clean_merged_branches(context: &Context) -> Result<String, String> {
    // main_branch=$(main_branch_name)
    // current_branch=$(current_branch_name)
    //
    // git diff --quiet
    // repo_is_clean=$?
    // if [ $repo_is_clean -ne "0" ]; then
    //   echo "Your working tree must be clean to use this comment. Stash or commit your changes."
    //   return 1
    // fi
    //
    // [ "$current_branch" != "$main_branch" ] && git checkout "$main_branch"
    // git pull
    //
    // deleted_current_branch=0
    //
    // for branch in $(merged_branches); do
    //   if [ "$branch" = "*" ] || [ "$branch" = "main" ] || [ "$branch" = "master" ]; then
    //     echo "Not deleting $branch"
    //   else
    //     if [ "$branch" = "$current_branch" ]; then
    //       deleted_current_branch=1
    //     fi
    //
    //     git branch -d "$branch"
    //   fi
    // done
    //
    // if [ "$current_branch" != "$main_branch" ] && [ $deleted_current_branch -eq "0" ]; then
    //   git checkout "$current_branch"
    // fi

    validate_clean_repo(context)?;

    let branches = merged_branches(&context.main_branch_name, &context.repo_path);
    println!("branches: {:?}", branches);

    Ok("".to_string())
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
