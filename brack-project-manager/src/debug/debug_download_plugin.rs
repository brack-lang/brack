use brack_project_manager::project::Project;

#[tokio::main]
async fn main() {
    let config = toml::from_str(&std::fs::read_to_string("Brack.toml").unwrap()).unwrap();
    let mut project = Project::new(config);
    let _ = project.download_plugins_using_config().await;
}
