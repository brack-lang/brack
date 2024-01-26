use core::fmt;
use std::{collections::HashMap, fs::File, io, path::Path};

use anyhow::Result;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

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

#[derive(Debug)]
pub enum Plugin {
    GitHub {
        owner: String,
        repo: String,
        version: String,
    },
}

impl Serialize for Plugin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Plugin", 4)?;
        match *self {
            Plugin::GitHub {
                ref owner,
                ref repo,
                ref version,
            } => {
                s.serialize_field("schema", "github")?;
                s.serialize_field("owner", owner)?;
                s.serialize_field("repo", repo)?;
                s.serialize_field("version", version)?;
            }
        }
        s.end()
    }
}

impl<'de> Deserialize<'de> for Plugin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PluginVisitor;

        impl<'de> Visitor<'de> for PluginVisitor {
            type Value = Plugin;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Plugin")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Plugin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut schema = None;
                let mut owner = None;
                let mut repo = None;
                let mut version = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "schema" => {
                            if schema.is_some() {
                                return Err(de::Error::duplicate_field("schema"));
                            }
                            schema = Some(map.next_value()?);
                        }
                        "owner" => {
                            if owner.is_some() {
                                return Err(de::Error::duplicate_field("owner"));
                            }
                            owner = Some(map.next_value()?);
                        }
                        "repo" => {
                            if repo.is_some() {
                                return Err(de::Error::duplicate_field("repo"));
                            }
                            repo = Some(map.next_value()?);
                        }
                        "version" => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("version"));
                            }
                            version = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(&key, FIELDS)),
                    }
                }

                let schema: String = schema.ok_or_else(|| de::Error::missing_field("schema"))?;
                let owner: String = owner.ok_or_else(|| de::Error::missing_field("owner"))?;
                let repo: String = repo.ok_or_else(|| de::Error::missing_field("repo"))?;
                let version: String = version.ok_or_else(|| de::Error::missing_field("version"))?;

                match schema.as_str() {
                    "github" => Ok(Plugin::GitHub {
                        owner,
                        repo,
                        version,
                    }),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Str(&schema),
                        &"github",
                    )),
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["schema", "owner", "repo", "version"];
        deserializer.deserialize_struct("Plugin", FIELDS, PluginVisitor)
    }
}

fn check_existence_brack_toml() -> bool {
    let path = Path::new("Brack.toml");
    path.exists()
}

pub async fn add_plugin(schema: &str) -> Result<()> {
    if !check_existence_brack_toml() {
        return Err(anyhow::anyhow!("Brack.toml is not found."));
    }

    let repository_type = schema
        .split(':')
        .nth(0)
        .ok_or_else(|| anyhow::anyhow!("Repository type is not found."))?;
    let owner = schema
        .split('/')
        .nth(0)
        .ok_or_else(|| anyhow::anyhow!("Owner is not found."))?
        .split(':')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Owner is not found."))?;
    let repo = schema
        .split('/')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Repository is not found."))?
        .split('@')
        .nth(0)
        .ok_or_else(|| anyhow::anyhow!("Repository is not found."))?;
    let version = schema
        .split('@')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Version is not found."))?;
    let url = match repository_type {
        "github" => format!("https://github.com/{}/{}", owner, repo),
        _ => {
            return Err(anyhow::anyhow!(
                "Repository type '{}' is not supported.",
                repository_type
            ))
        }
    };

    let mut config: Config = toml::from_str(&std::fs::read_to_string("Brack.toml")?)?;

    let repository_name = url
        .trim_end_matches('/')
        .split('/')
        .last()
        .ok_or_else(|| anyhow::anyhow!("Last element of URL is not found."))?;
    let plugin_name = repository_name
        .split('.')
        .next()
        .ok_or_else(|| anyhow::anyhow!("First element of repository name is not found."))?;

    if let Some(_) = config
        .plugins
        .as_ref()
        .and_then(|plugins| plugins.get(plugin_name))
    {
        return Err(anyhow::anyhow!("Plugin already exists."));
    }

    let download_url = format!(
        "{}/releases/download/{}/{}.wasm",
        url, version, repository_name
    );

    let response = reqwest::get(&download_url).await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download plugin from {}.\nStatus: {} - {}",
            download_url,
            response.status().as_str(),
            response
                .status()
                .canonical_reason()
                .unwrap_or("Unknown error")
        ));
    }

    let bytes = response.bytes().await?;
    std::fs::create_dir_all("plugins")?;
    let mut out = File::create(format!("plugins/{}.wasm", repository_name))?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    config.plugins.get_or_insert_with(|| HashMap::new()).insert(
        plugin_name.to_string(),
        Plugin::GitHub {
            owner: owner.to_string(),
            repo: repo.to_string(),
            version: version.to_string(),
        },
    );
    let toml = toml::to_string(&config)?;
    std::fs::write("Brack.toml", toml)?;

    Ok(())
}
