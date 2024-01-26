use std::{collections::HashMap, fs::File, io, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub document: Document,
    pub plugins: Option<HashMap<String, Plugin>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub version: String,
    pub backend: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub url: String,
    pub version: String,
}


fn check_existence_brack_toml() -> bool {
    let path = Path::new("Brack.toml");
    path.exists()
}

pub async fn add_plugin(url: &str, version: &str) -> Result<()> {
    if !check_existence_brack_toml() {
        return Err(anyhow::anyhow!("Brack.toml is not found."));
    }

    let mut config: Config = toml::from_str(&std::fs::read_to_string("Brack.toml")?)?;

    let repository_name = url.trim_end_matches('/').split('/').last().ok_or_else(|| anyhow::anyhow!("Last element of URL is not found."))?;
    let plugin_name = repository_name.split('.').next().ok_or_else(|| anyhow::anyhow!("First element of repository name is not found."))?;

    if let Some(_) = config.plugins.as_ref().and_then(|plugins| plugins.get(plugin_name)) {
        return Err(anyhow::anyhow!("Plugin already exists."));
    }

    let download_url = format!("{}/releases/download/{}/{}.wasm", url, version, repository_name);

    let response = reqwest::get(&download_url).await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download plugin from {}.\nStatus: {} - {}",
            download_url,
            response.status().as_str(),
            response.status().canonical_reason().unwrap_or("Unknown error")
        ));
    }

    let bytes = response.bytes().await?;
    std::fs::create_dir_all("plugins")?;
    let mut out = File::create(format!("plugins/{}.wasm", repository_name))?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    config.plugins.get_or_insert_with(|| HashMap::new()).insert(
        plugin_name.to_string(),
        Plugin {
            url: url.to_string(),
            version: version.to_string(),
        },
    );
    let toml = toml::to_string(&config)?;
    std::fs::write("Brack.toml", toml)?;

    Ok(())
}
