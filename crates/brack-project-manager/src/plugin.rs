use crate::config::Config;
use core::fmt;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs::File, io, path::Path};

use anyhow::Result;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone)]
pub enum PluginSchema {
    GitHub {
        owner: String,
        repo: String,
        version: String,
        expr_hook: Option<bool>,
        stmt_hook: Option<bool>,
        document_hook: Option<bool>,
    },
}

impl Serialize for PluginSchema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Plugin", 4)?;
        match *self {
            PluginSchema::GitHub {
                ref owner,
                ref repo,
                ref version,
                ref expr_hook,
                ref stmt_hook,
                ref document_hook,
            } => {
                s.serialize_field("schema", "github")?;
                s.serialize_field("owner", owner)?;
                s.serialize_field("repo", repo)?;
                s.serialize_field("version", version)?;
                if let Some(expr_hook) = expr_hook {
                    s.serialize_field("expr_hook", expr_hook)?;
                }
                if let Some(stmt_hook) = stmt_hook {
                    s.serialize_field("stmt_hook", stmt_hook)?;
                }
                if let Some(document_hook) = document_hook {
                    s.serialize_field("document_hook", document_hook)?;
                }
            }
        }
        s.end()
    }
}

impl<'de> Deserialize<'de> for PluginSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PluginVisitor;

        impl<'de> Visitor<'de> for PluginVisitor {
            type Value = PluginSchema;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Plugin")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PluginSchema, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut schema = None;
                let mut owner = None;
                let mut repo = None;
                let mut version = None;
                let mut expr_hook = None;
                let mut stmt_hook = None;
                let mut document_hook = None;

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
                        "expr_hook" => {
                            if expr_hook.is_some() {
                                return Err(de::Error::duplicate_field("expr_hook"));
                            }
                            expr_hook = Some(map.next_value()?);
                        }
                        "stmt_hook" => {
                            if stmt_hook.is_some() {
                                return Err(de::Error::duplicate_field("stmt_hook"));
                            }
                            stmt_hook = Some(map.next_value()?);
                        }
                        "document_hook" => {
                            if document_hook.is_some() {
                                return Err(de::Error::duplicate_field("document_hook"));
                            }
                            document_hook = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(&key, FIELDS)),
                    }
                }

                let schema: String = schema.ok_or_else(|| de::Error::missing_field("schema"))?;
                let owner: String = owner.ok_or_else(|| de::Error::missing_field("owner"))?;
                let repo: String = repo.ok_or_else(|| de::Error::missing_field("repo"))?;
                let version: String = version.ok_or_else(|| de::Error::missing_field("version"))?;

                match schema.as_str() {
                    "github" => Ok(PluginSchema::GitHub {
                        owner,
                        repo,
                        version,
                        expr_hook,
                        stmt_hook,
                        document_hook,
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

impl PluginSchema {
    pub fn hash_sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", self));
        format!("{:x}", hasher.finalize())
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
        PluginSchema::GitHub {
            owner: owner.to_string(),
            repo: repo.to_string(),
            version: version.to_string(),
            expr_hook: None,
            stmt_hook: None,
            document_hook: None,
        },
    );
    let toml = toml::to_string(&config)?;
    std::fs::write("Brack.toml", toml)?;

    Ok(())
}
