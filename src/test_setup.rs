use std::{error::Error, process::Command};

pub const BARE_REPO_NAME: &str = "bare repo  -_^^ with symbols and spaces";
pub const CLEAN_NORMAL_REPO_NAME: &str = "clean_repo";
pub const DIRTY_NORMAL_REPO_NAME: &str = "dirty_repo";
pub const DUMMY_REPOS_DIRECTORY: &str = "dummy_repos";

pub fn setup(test_name: &str, bare_repo: bool) -> Result<(), Box<dyn Error>> {
    println!("test_name: {:?}", test_name);
    println!("bare_repo: {:?}", bare_repo);
    // make sure we start with a clean slate even of a previous test failed
    teardown(test_name)?;

    match bare_repo {
        true => {
            create_bare_repo(test_name)?;
            setup_worktrees(test_name)?;
        }
        false => create_normal_repos(test_name)?,
    }

    Ok(())
}

fn create_bare_repo(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("mkdir")
        .arg("-p")
        .arg(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("git")
        .arg("init")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("touch")
        .arg("README.md")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit readme")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("other-branch")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("wont-match-path")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    // Make sure we are on the main branch so that becomes the tracked main branch in the bare repo
    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .output()?;

    Command::new("git")
        .arg("clone")
        .arg("--bare")
        .arg(format!(
            "{}/{}/bare_repo_source",
            DUMMY_REPOS_DIRECTORY, test_name
        ))
        .arg(format!(
            "{}/{}/{}",
            DUMMY_REPOS_DIRECTORY, test_name, BARE_REPO_NAME
        ))
        .output()?;

    Ok(())
}

fn create_worktree(
    test_name: &str,
    worktree_name: &str,
    worktree_path: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("git");
    command.current_dir(format!(
        "{}/{}/{}",
        DUMMY_REPOS_DIRECTORY, test_name, BARE_REPO_NAME
    ));
    command.arg("worktree").arg("add");

    if let Some(path) = worktree_path {
        command.arg(path);
    }

    command.arg(worktree_name).output()?;

    Ok(())
}

fn setup_worktrees(test_name: &str) -> Result<(), Box<dyn Error>> {
    create_worktree(test_name, "main", None)?;
    create_worktree(test_name, "dirty", None)?;
    create_worktree(test_name, "unmerged", None)?;
    create_worktree(test_name, "merged", None)?;
    create_worktree(test_name, "other-branch", Some("origin/other-branch"))?;
    create_worktree(
        test_name,
        "wont-match-path",
        Some("origin/doesnt-match-name"),
    )?;

    Command::new("touch")
        .arg("uncommitted-file")
        .current_dir(format!(
            "dummy_repos/{}/{}/dirty",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}/unmerged",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}/unmerged",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("file that won't be merged")
        .current_dir(format!(
            "dummy_repos/{}/{}/unmerged",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}/merged",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}/merged",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("file that will merged")
        .current_dir(format!(
            "dummy_repos/{}/{}/merged",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("merged file 2")
        .current_dir(format!(
            "dummy_repos/{}/{}/origin/doesnt-match-name",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged file 2")
        .current_dir(format!(
            "dummy_repos/{}/{}/origin/doesnt-match-name",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("file that will be merged")
        .current_dir(format!(
            "dummy_repos/{}/{}/origin/doesnt-match-name",
            test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("merged")
        .current_dir(format!(
            "{}/{}/{}/main",
            DUMMY_REPOS_DIRECTORY, test_name, BARE_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("wont-match-path")
        .current_dir(format!(
            "{}/{}/{}/main",
            DUMMY_REPOS_DIRECTORY, test_name, BARE_REPO_NAME
        ))
        .output()?;

    Ok(())
}

fn create_normal_repos(test_name: &str) -> Result<(), Box<dyn Error>> {
    create_clean_repo(test_name)?;
    create_dirty_repo(test_name)?;

    Ok(())
}

fn create_clean_repo(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("mkdir")
        .arg("-p")
        .arg(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("init")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("README.md")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit readme")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("unmerged")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("merged")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("merged")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, CLEAN_NORMAL_REPO_NAME
        ))
        .output()?;

    Ok(())
}

fn create_dirty_repo(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("mkdir")
        .arg("-p")
        .arg(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("init")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("README.md")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit readme")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("unmerged")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit unmerged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("merged")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit merged-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("merged")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Command::new("touch")
        .arg("dirty-file")
        .current_dir(format!(
            "dummy_repos/{}/{}",
            test_name, DIRTY_NORMAL_REPO_NAME
        ))
        .output()?;

    Ok(())
}

pub fn teardown(test_name: &str) -> Result<(), Box<dyn Error>> {
    delete_dummy_repos(test_name)?;

    Ok(())
}

fn delete_dummy_repos(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("rm")
        .arg("-rf")
        .arg(format!("{}/{}", DUMMY_REPOS_DIRECTORY, test_name))
        .output()?;

    Ok(())
}
