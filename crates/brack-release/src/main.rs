use std::fs::read_to_string;
use std::path::Path;
use tokio::process::Command;
use toml_edit::{value, DocumentMut};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

mod semver;
use crate::semver::SemVer;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Update { semver_kind: SemVerKind },
    DebugUpdate { version: String },
    Release,
}

#[derive(Debug, Clone, ValueEnum)]
enum SemVerKind {
    Major,
    Minor,
    Patch,
    RC,
}

fn get_current_version<P: AsRef<Path>>(path: P) -> Result<SemVer> {
    let file = read_to_string(path)?;
    let mut lines = file.lines();
    let version = SemVer::new_with_string(
        lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("No version found"))?,
    )?;
    Ok(version)
}

fn rewrite_version<P: AsRef<Path> + Copy>(path: P, version: &SemVer) -> Result<()> {
    let version = version.to_string();
    let toml_str = read_to_string(path)?;
    let mut toml = toml_str.parse::<DocumentMut>()?;
    toml["package"]["version"] = value(version);
    let toml_str = toml.to_string();
    std::fs::write(path, toml_str)?;
    Ok(())
}

fn rewrite_all_cargo_toml(next_version: &SemVer) -> Result<()> {
    let cargo_toml_paths = [
        "Cargo.toml",
        "crates/brack-codegen/Cargo.toml",
        "crates/brack-expander/Cargo.toml",
        "crates/brack-language-server/Cargo.toml",
        "crates/brack-parser/Cargo.toml",
        "crates/brack-plugin/Cargo.toml",
        "crates/brack-project-manager/Cargo.toml",
        "crates/brack-tokenizer/Cargo.toml",
        "crates/brack-transformer/Cargo.toml",
    ];
    for path in cargo_toml_paths.iter() {
        rewrite_version(path, &next_version)?;
    }
    Ok(())
}

fn rewrite_version_file<P: AsRef<Path> + Copy>(path: P, version: &SemVer) -> Result<()> {
    let version = version.to_string();
    std::fs::write(path, version)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.sub_commands {
        SubCommands::Update { semver_kind } => {
            let status = Command::new("git").arg("status").arg("--porcelain").output().await?;
            if !status.stdout.is_empty() {
                println!("Please commit all changes before updating version");
                return Ok(());
            }
            let current_version = get_current_version("VERSION")?;
            let next_version = match semver_kind {
                SemVerKind::Major => current_version.next_major(),
                SemVerKind::Minor => current_version.next_minor(),
                SemVerKind::Patch => current_version.next_patch(),
                SemVerKind::RC => current_version.next_rc(),
            }?;
            match semver_kind {
                SemVerKind::Major | SemVerKind::Minor | SemVerKind::Patch => {
                    let except_rc_version = next_version.release()?;
                    Command::new("git")
                        .arg("switch")
                        .arg("-c")
                        .arg(format!("release/v{}", except_rc_version))
                        .status()
                        .await?;
                    rewrite_all_cargo_toml(&next_version)?;
                    rewrite_version_file("VERSION", &next_version)?;
                    Command::new("git")
                        .arg("commit")
                        .arg("-am")
                        .arg(format!("update: prepare for next version: {}", next_version))
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("tag")
                        .arg(format!("v{}", next_version))
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("push")
                        .arg("origin")
                        .arg(format!("release/v{}", except_rc_version))
                        .status()
                        .await?;
                    println!("🎉 Successfully updated version: {} and pre-released", next_version);
                }
                SemVerKind::RC => {
                    let except_rc_version = next_version.release()?;
                    Command::new("git")
                        .arg("switch")
                        .arg("develop")
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("pull")
                        .arg("origin")
                        .arg("develop")
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("switch")
                        .arg(format!("release/v{}", except_rc_version))
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("merge")
                        .arg("--no-ff")
                        .arg("develop")
                        .status()
                        .await?;
                    rewrite_all_cargo_toml(&next_version)?;
                    rewrite_version_file("VERSION", &next_version)?;
                    Command::new("git")
                        .arg("commit")
                        .arg("-am")
                        .arg(format!("update: prepare for next version: {}", next_version))
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("tag")
                        .arg(format!("v{}", next_version))
                        .status()
                        .await?;
                    Command::new("git")
                        .arg("push")
                        .arg("origin")
                        .arg(format!("release/v{}", except_rc_version))
                        .status()
                        .await?;
                    println!("🎉 Successfully updated version: {} and pre-released", next_version);
                }
            }
        }
        SubCommands::DebugUpdate { version } => {
            let next_version = SemVer::new_with_string(&version)?;
            rewrite_all_cargo_toml(&next_version)?;
            rewrite_version_file("VERSION", &next_version)?;
        }
        SubCommands::Release => {
            let status = Command::new("git").arg("status").arg("--porcelain").output().await?;
            if !status.stdout.is_empty() {
                println!("Please commit all changes before updating version");
                return Ok(());
            }
            let current_version = get_current_version("VERSION")?;
            let next_version = current_version.release()?;
            rewrite_all_cargo_toml(&next_version)?;
            rewrite_version_file("VERSION", &next_version)?;
            Command::new("git")
                .arg("commit")
                .arg("-am")
                .arg(format!("update: prepare for next version: {}", next_version))
                .status()
                .await?;
            Command::new("git")
                .arg("switch")
                .arg("main")
                .status()
                .await?;
            Command::new("git")
                .arg("pull")
                .arg("origin")
                .arg("main")
                .status()
                .await?;
            Command::new("git")
                .arg("merge")
                .arg("--no-ff")
                .arg(format!("release/v{}", next_version))
                .status()
                .await?;
            Command::new("git")
                .arg("tag")
                .arg(format!("v{}", next_version))
                .status()
                .await?;
            Command::new("git")
                .arg("push")
                .arg("origin")
                .arg("main")
                .status()
                .await?;
            Command::new("git")
                .arg("push")
                .arg("origin")
                .arg(format!("v{}", next_version))
                .status()
                .await?;
            println!("🎉 Successfully released version: {}", next_version);
        }
    }
    Ok(())
}
