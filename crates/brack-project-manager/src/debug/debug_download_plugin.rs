use anyhow::Result;
use brack_project_manager::project::Project;

#[tokio::main]
async fn main() -> Result<()> {
    let mut project = Project::new();
    project.load_brack_toml()?;
    project.clear_plugins()?;
    project.download_plugins_using_config().await?;
    dbg!(&project.plugins_metadata);
    Ok(())
}
