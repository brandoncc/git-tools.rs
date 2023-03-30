use std::{error::Error, process::Command};

pub fn setup(test_name: &str) -> Result<(), Box<dyn Error>> {
    // make sure we start with a clean slate even of a previous test failed
    teardown(test_name)?;

    create_bare_repo(test_name)?;
    setup_worktrees(test_name)?;
    create_normal_repos(test_name)?;

    Ok(())
}

fn create_bare_repo(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("mkdir")
        .arg("-p")
        .arg(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("mkdir")
        .arg("-p")
        .arg(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .output()?;

    Command::new("git")
        .arg("init")
        .current_dir(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .output()?;

    Command::new("touch")
        .arg("README.md")
        .current_dir(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit readme")
        .current_dir(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .output()?;

    Command::new("git")
        .arg("clone")
        .arg("--bare")
        .arg(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .arg(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("remote")
        .arg("rm")
        .arg("origin")
        .current_dir(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("rm")
        .arg("-rf")
        .arg(format!("dummy_repos/{}/bare_repo.tmp", test_name))
        .output()?;

    Ok(())
}

fn setup_worktrees(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("main")
        .current_dir(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("dirty")
        .current_dir(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("uncommitted-file")
        .current_dir(format!("dummy_repos/{}/bare_repo/dirty", test_name))
        .output()?;

    Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("unmerged")
        .current_dir(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("unmerged-file")
        .current_dir(format!("dummy_repos/{}/bare_repo/unmerged", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("unmerged-file")
        .current_dir(format!("dummy_repos/{}/bare_repo/unmerged", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("file that won't be merged")
        .current_dir(format!("dummy_repos/{}/bare_repo/unmerged", test_name))
        .output()?;

    Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("merged")
        .current_dir(format!("dummy_repos/{}/bare_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("merged-file")
        .current_dir(format!("dummy_repos/{}/bare_repo/merged", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged-file")
        .current_dir(format!("dummy_repos/{}/bare_repo/merged", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("file that will be merged")
        .current_dir(format!("dummy_repos/{}/bare_repo/merged", test_name))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("merged")
        .current_dir(format!("dummy_repos/{}/bare_repo/main", test_name))
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
        .arg(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("init")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("README.md")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit readme")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("unmerged")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("unmerged-file")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("unmerged-file")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit unmerged-file")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("merged")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("merged-file")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged-file")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit merged-file")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("merged")
        .current_dir(format!("dummy_repos/{}/clean_repo", test_name))
        .output()?;

    Ok(())
}

fn create_dirty_repo(test_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("mkdir")
        .arg("-p")
        .arg(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("init")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("README.md")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit readme")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("unmerged")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("unmerged-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("unmerged-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit unmerged-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("merged")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("merged-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("add")
        .arg("merged-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit merged-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("checkout")
        .arg("main")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("git")
        .arg("merge")
        .arg("merged")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
        .output()?;

    Command::new("touch")
        .arg("dirty-file")
        .current_dir(format!("dummy_repos/{}/dirty_repo", test_name))
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
        .arg(format!("dummy_repos/{}", test_name))
        .output()?;

    Ok(())
}
