use anyhow::Result;
use brack_project_manager::project::{self, Project};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let mut project = Project::new();
    project.load_brack_toml()?;
    project.download_plugins_using_config().await?;
    project.build()?;
    Ok(())
}
