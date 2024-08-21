use anyhow::Result;
use brack_project_manager::project::Project;

#[tokio::main]
async fn main() -> Result<()> {
    let config = toml::from_str(&std::fs::read_to_string("Brack.toml").unwrap()).unwrap();
    let mut project = Project::new(config);
    project.download_plugins_using_config().await?;
    dbg!(&project.plugins_metadata);
    Ok(())
}
