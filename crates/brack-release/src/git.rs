use crate::semver::SemVer;
use anyhow::{bail, Result};
use tokio::process::Command;

pub async fn git_switch(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("switch")
        .arg(branch)
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to switch branch to '{}'", branch);
    }
    Ok(())
}

pub async fn git_switch_new(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("switch")
        .arg("-c")
        .arg(branch)
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to create and switch branch to '{}'", branch);
    }
    Ok(())
}

pub async fn git_pull(remote: &str, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("pull")
        .arg(remote)
        .arg(branch)
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to pull {} {}", remote, branch);
    }
    Ok(())
}

pub async fn git_merge_no_ff(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("merge")
        .arg("--no-ff")
        .arg(branch)
        .status()
        .await?;
    if !status.success() {
        bail!(
            "Failed to merge branch '{}'. Possibly a merge conflict?",
            branch
        );
    }
    Ok(())
}

pub async fn git_commit_all(message: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["commit", "-am", message])
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to commit. message='{}'", message);
    }
    Ok(())
}

pub async fn git_push(remote: &str, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("push")
        .arg(remote)
        .arg(branch)
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to push to {}/{}", remote, branch);
    }
    Ok(())
}

pub async fn git_push_tags(remote: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("push")
        .arg(remote)
        .arg("--tags")
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to push tags to {}", remote);
    }
    Ok(())
}

pub async fn create_git_tag(version: &SemVer) -> Result<()> {
    let status = Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(format!("v{}", version))
        .arg("-m")
        .arg(format!("release version: {}", version))
        .status()
        .await?;
    if !status.success() {
        bail!("Failed to create git tag: {:?}", status);
    }
    Ok(())
}
