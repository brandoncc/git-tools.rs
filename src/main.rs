use std::{env, path::PathBuf, process::exit};

use utils::{get_bare_root, get_main_branch_name, get_normal_root, is_bare_repo};

use crate::utils::expand_path;

mod bare_repo;
mod commands;
mod normal_repo;
mod utils;

pub type CommandWorkingDirectory = PathBuf;

#[derive(Debug)]
enum AvailableCommands {
    CleanMergedBranches,
    Invalid,
}

#[derive(Debug)]
pub enum RepoType {
    Bare,
    Normal,
}

impl From<String> for AvailableCommands {
    fn from(value: String) -> Self {
        match value.as_str() {
            "clean-merged-branches" => Self::CleanMergedBranches,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    repo_type: RepoType,
    repo_path: PathBuf,
    main_branch_name: String,
}

impl Context {
    fn create() -> Self {
        let cwd = get_cwd();
        let repo_type = get_repo_type(&cwd);
        let repo_path = get_repo_path(&cwd, &repo_type);
        let main_branch_name = get_main_branch_name(&repo_path);

        Self {
            main_branch_name,
            repo_path,
            repo_type,
        }
    }
}

fn get_cwd() -> CommandWorkingDirectory {
    match env::var("REPO") {
        Ok(repo) => PathBuf::from(expand_path(repo)),
        Err(_) => env::current_dir().expect("Couldn't get the current working directory"),
    }
}

fn get_command() -> AvailableCommands {
    let args: Vec<String> = env::args().collect();
    AvailableCommands::from(args[1].clone())
}

fn get_repo_path(cwd: &CommandWorkingDirectory, repo_type: &RepoType) -> PathBuf {
    // let result = run_command(CommandConfiguration::new(
    //     "git",
    //     Some(vec!["rev-parse", "--is-bare-repository"]),
    //     cwd,
    // ));
    //
    // let output = res
    //     .expect("Error: command must be run from within a git repository")
    //     .output
    //     .first()
    //     .unwrap()
    //     .clone();
    //
    // match output {
    //     "true" => false,
    //     Err(res) => {
    //         println!("res: {:?}", res);
    //         println!("");
    //         exit(1);
    //     }
    // }

    match repo_type {
        RepoType::Bare => {
            get_bare_root(cwd).expect("Expected to find a bare repo root, but didn't")
        }
        RepoType::Normal => {
            get_normal_root(cwd).expect("Expected to find a normal repo root, but didn't")
        }
    }
}

fn get_repo_type(repo_path: &PathBuf) -> RepoType {
    match is_bare_repo(&repo_path) {
        true => RepoType::Bare,
        false => RepoType::Normal,
    }
}

fn main() {
    let context = Context::create();
    println!("context: {:?}", context);

    match get_command() {
        AvailableCommands::CleanMergedBranches => match context.repo_type {
            RepoType::Bare => panic!("Not implemented"),
            RepoType::Normal => match normal_repo::clean_merged_branches(&context) {
                Ok(_) => {}
                Err(msg) => {
                    println!("Error: {}", msg);
                    exit(1);
                }
            },
        },
        _ => {
            println!("Available commands: clean-merged-branches");
            println!("repo path: {:?}", context.repo_path);
            exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    mod test_helpers;
    mod test_setup;

    use super::*;
    use self::test_helpers::run_test;
    use std::process::Command;

    #[test]
    fn test_merged_branches_are_not_deleted_if_working_tree_is_not_clean() {
        run_test(
            "test_merged_branches_are_not_deleted_if_working_tree_is_not_clean",
            "dirty_repo",
            RepoType::Normal,
            |context| {
                let result = normal_repo::clean_merged_branches(&context);

                assert!(result.is_err());

                test_helpers::assert_branches(
                    context,
                    vec![
                        "main".to_string(),
                        "merged".to_string(),
                        "unmerged".to_string(),
                    ],
                );
            },
        );
    }

    #[test]
    fn test_merged_branches_are_deleted_if_working_tree_is_clean() {
        run_test(
            "test_merged_branches_are_deleted_if_working_tree_is_clean",
            "clean_repo",
            RepoType::Normal,
            |context| {
                let result = normal_repo::clean_merged_branches(&context);

                assert!(result.is_ok());

                test_helpers::assert_branch_does_not_exist(context, "merged".to_string());
            },
        );
    }

    #[test]
    fn test_unmerged_branches_are_not_deleted() {
        run_test(
            "test_unmerged_branches_are_not_deleted",
            "clean_repo",
            RepoType::Normal,
            |context| {
                let result = normal_repo::clean_merged_branches(&context);

                assert!(result.is_ok());

                test_helpers::assert_branch_exists(context, "unmerged".to_string());
            },
        );
    }

    #[test]
    fn test_deleting_current_head_branch_leaves_repo_with_main_branch_checked_out() {
        run_test(
            "test_deleting_current_head_branch_leaves_repo_with_main_branch_checked_out",
            "clean_repo",
            RepoType::Normal,
            |context| {
                Command::new("git")
                    .arg("checkout")
                    .arg("merged")
                    .status()
                    .expect("failed to checkout merged branch");

                test_helpers::assert_current_branch(&context, "merged".to_string());

                normal_repo::clean_merged_branches(&context)
                    .expect("failed to clean merged branches");

                test_helpers::assert_current_branch(&context, "main".to_string());
            },
        );
    }

    #[test]
    fn test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out() {
        run_test(
            "test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out",
            "clean_repo",
            RepoType::Normal,
            |context| {
                Command::new("git")
                    .arg("checkout")
                    .arg("unmerged")
                    .status()
                    .expect("failed to checkout merged branch");

                test_helpers::assert_current_branch(&context, "unmerged".to_string());

                normal_repo::clean_merged_branches(&context)
                    .expect("failed to clean merged branches");

                test_helpers::assert_current_branch(&context, "unmerged".to_string());
            },
        );

        test_helpers::run_setup(
            "test_not_deleting_current_head_branch_leaves_repo_with_the_same_branch_checked_out",
        );
    }

    #[test]
    fn test_dirty_worktrees_are_not_removed() {
        run_test(
            "test_dirty_worktrees_are_not_removed",
            "bare_repo",
            RepoType::Bare,
            |context| {
                bare_repo::clean_merged_branches(&context)
                    .expect("failed to clean merged branches");

                test_helpers::assert_worktree_exists(context, "dirty".to_string());
            },
        );
    }

    #[test]
    fn test_unmerged_worktrees_are_not_removed() {
        run_test(
            "test_unmerged_worktrees_are_not_removed",
            "bare_repo",
            RepoType::Bare,
            |context| {
                bare_repo::clean_merged_branches(&context)
                    .expect("failed to clean merged branches");

                test_helpers::assert_worktree_exists(context, "unmerged".to_string());
            },
        );
    }

    #[test]
    fn test_merged_worktrees_are_removed() {
        run_test(
            "test_merged_worktrees_are_removed",
            "bare_repo",
            RepoType::Bare,
            |context| {
                bare_repo::clean_merged_branches(&context)
                    .expect("failed to clean merged branches");

                test_helpers::assert_worktree_does_not_exist(context, "merged".to_string());
            },
        );
    }

    #[test]
    fn test_main_worktree_is_not_removed() {
        run_test(
            "test_main_worktree_is_not_removed",
            "bare_repo",
            RepoType::Bare,
            |context| {
                bare_repo::clean_merged_branches(&context)
                    .expect("failed to clean merged branches");

                test_helpers::assert_worktree_exists(context, "main".to_string());
            },
        );
    }
}
