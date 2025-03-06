use crate::config::Config;
use core::fmt;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone)]
pub struct Hook {
    pub expr: Option<bool>,
    pub stmt: Option<bool>,
    pub document: Option<bool>,
    pub text: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum PluginSchema {
    GitHub {
        owner: String,
        repo: String,
        version: String,
        hook: Hook,
    },
    Local {
        path: String,
        hook: Hook,
    },
}

trait SerializeHookFields: SerializeStruct {
    fn serialize_hook_fields(&mut self, hook: &Hook) -> Result<(), Self::Error>;
}

impl<T> SerializeHookFields for T
where
    T: SerializeStruct,
{
    fn serialize_hook_fields(&mut self, hook: &Hook) -> Result<(), T::Error> {
        if let Some(expr_hook) = hook.expr {
            self.serialize_field("expr_hook", &expr_hook)?;
        }
        if let Some(stmt_hook) = hook.stmt {
            self.serialize_field("stmt_hook", &stmt_hook)?;
        }
        if let Some(document_hook) = hook.document {
            self.serialize_field("document_hook", &document_hook)?;
        }
        if let Some(text_hook) = hook.text {
            self.serialize_field("text_hook", &text_hook)?;
        }
        Ok(())
    }
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
                ref hook,
            } => {
                s.serialize_field("schema", "github")?;
                s.serialize_field("owner", owner)?;
                s.serialize_field("repo", repo)?;
                s.serialize_field("version", version)?;
                s.serialize_hook_fields(hook)?;
            }
            PluginSchema::Local { ref path, ref hook } => {
                s.serialize_field("schema", "local")?;
                s.serialize_field("path", path)?;
                s.serialize_hook_fields(hook)?;
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
                let mut path = None;
                let mut expr_hook = None;
                let mut stmt_hook = None;
                let mut document_hook = None;
                let mut text_hook = None;

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
                        "path" => {
                            if path.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            path = Some(map.next_value()?);
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
                        "text_hook" => {
                            if text_hook.is_some() {
                                return Err(de::Error::duplicate_field("text_hook"));
                            }
                            text_hook = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(&key, FIELDS)),
                    }
                }

                let schema: String = schema.ok_or_else(|| de::Error::missing_field("schema"))?;
                let owner: String = match schema.as_str() {
                    "github" => owner.ok_or_else(|| de::Error::missing_field("owner"))?,
                    _ => "".to_string(),
                };
                let repo: String = match schema.as_str() {
                    "github" => repo.ok_or_else(|| de::Error::missing_field("repo"))?,
                    _ => "".to_string(),
                };
                let version: String = match schema.as_str() {
                    "github" => version.ok_or_else(|| de::Error::missing_field("version"))?,
                    _ => "".to_string(),
                };
                let path: String = match schema.as_str() {
                    "local" => path.ok_or_else(|| de::Error::missing_field("path"))?,
                    _ => "".to_string(),
                };
                let hook = Hook {
                    expr: expr_hook,
                    stmt: stmt_hook,
                    document: document_hook,
                    text: text_hook,
                };

                match schema.as_str() {
                    "github" => Ok(PluginSchema::GitHub {
                        owner,
                        repo,
                        version,
                        hook,
                    }),
                    "local" => Ok(PluginSchema::Local { path, hook }),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Str(&schema),
                        &"github or local",
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &[
            "schema",
            "owner",
            "repo",
            "version",
            "path",
            "expr_hook",
            "stmt_hook",
            "document_hook",
            "text_hook",
        ];
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

async fn add_plugin_github(schema: &str) -> Result<Config> {
    // github:owner/repo@version
    let owner = schema
        .split('/')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Owner is not found."))?
        .split(':')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Owner is not found."))?;
    let repo = schema
        .split('/')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Repository is not found."))?
        .split('@')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Repository is not found."))?;
    let version = schema
        .split('@')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Version is not found."))?;
    let url = format!("https://github.com/{}/{}", owner, repo);

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

    if config
        .plugins
        .as_ref()
        .and_then(|plugins| plugins.get(plugin_name))
        .is_some()
    {
        return Err(anyhow::anyhow!("Plugin already exists."));
    }

    config.plugins.get_or_insert_with(HashMap::new).insert(
        plugin_name.to_string(),
        PluginSchema::GitHub {
            owner: owner.to_string(),
            repo: repo.to_string(),
            version: version.to_string(),
            hook: Hook {
                expr: None,
                stmt: None,
                document: None,
                text: None,
            },
        },
    );
    Ok(config)
}

fn add_plugin_local(schema: &str) -> Result<Config> {
    // local:path/to/plugin.wasm
    let path = schema
        .split(':')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Path is not found."))?;
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow::anyhow!("Plugin file is not found."));
    }

    let mut config: Config = toml::from_str(&std::fs::read_to_string("Brack.toml")?)?;
    config.plugins.get_or_insert_with(HashMap::new).insert(
        path.file_stem()
            .ok_or_else(|| anyhow::anyhow!("File stem is not found."))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("File stem is not found."))?
            .split('.')
            .next()
            .ok_or_else(|| anyhow::anyhow!("First element of file stem is not found."))?
            .to_string(),
        PluginSchema::Local {
            path: path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Path is not found."))?
                .to_string(),
            hook: Hook {
                expr: None,
                stmt: None,
                document: None,
                text: None,
            },
        },
    );
    Ok(config)
}

pub async fn add_plugin(schema: &str) -> Result<()> {
    if !check_existence_brack_toml() {
        return Err(anyhow::anyhow!("Brack.toml is not found."));
    }

    let schema_type = schema
        .split(':')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Repository type is not found."))?;

    let config = match schema_type {
        "github" => add_plugin_github(schema).await?,
        "local" => add_plugin_local(schema)?,
        _ => return Err(anyhow::anyhow!("Unknown repository type.")),
    };

    let toml = toml::to_string(&config)?;
    std::fs::write("Brack.toml", toml)?;

    Ok(())
}
