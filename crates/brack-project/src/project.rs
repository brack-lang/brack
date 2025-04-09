use crate::manifest::{Author, DocumentSettings, License, Manifest};
use anyhow::Result;
use brack_common::{
    logger::Logger,
    project_errors::{ProjectDebug, ProjectError, ProjectInfo, ProjectWarning},
};
use brack_plugin::feature_flag::FeatureFlag;
// use brack_plugin::plugins::Plugins;
// use futures_util::StreamExt;
// use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, remove_dir_all, remove_file, write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Project {
    pub manifest: Manifest,
    pub project_root: Option<PathBuf>,
    pub channels: Option<Channels>,
    // todo: delete this field
    pub plugins_metadata: HashMap<String, (String, FeatureFlag)>,
}

const BRACK_TOML: &str = "Brack.toml";

pub type Channels = HashMap<String, Channel>;

pub type Channel = HashMap<String, PluginInChannel>;

#[derive(Serialize, Deserialize, Clone)]
pub enum PluginInChannelMethod {
    Git,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginInChannel {
    pub name: String,
    pub method: PluginInChannelMethod,
    pub url: String,
    pub licenses: Vec<License>,
    pub support_backends: Vec<String>,
}

// fn get_brack_progress_bar() -> ProgressBar {
//     let pb = ProgressBar::new(0);
//     pb.set_style(
//         ProgressStyle::default_bar()
//             .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>3}/{len:3} {msg}")
//             .unwrap()
//             .progress_chars("=>-"),
//     );
//     pb
// }

// async fn download_plugin_from_channel(channel: Channel, version: String) -> Result<()> {
//     Ok(())
// }

// async fn download_plugin_from_local(
//     path: String,
//     feature_flags: Option<Vec<String>>,
// ) -> Result<()> {
//     Ok(())
// }

// async fn download_plugin_from_github(owner: String, name: String, tag: String) -> Result<()> {
//     let url = format!(
//         "https://github.com/{}/{}/releases/download/{}/{}.tar.gz",
//         owner, name, tag, name
//     );
//     let pb = get_brack_progress_bar();
//     pb.set_message(format!("Downloading plugin {}", name));
//     let response = reqwest::get(&url).await?;
//     if !response.status().is_success() {
//         pb.finish_and_clear();
//         anyhow::bail!(
//             "Failed to download plugin from {}.\nStatus: {} - {}",
//             url,
//             response.status().as_str(),
//             response
//                 .status()
//                 .canonical_reason()
//                 .unwrap_or("Unknown error")
//         );
//     }
//     if let Some(size) = response.content_length() {
//         pb.set_length(size);
//     }
//     let mut stream = response.bytes_stream();
//     let mut downloaded_data = Vec::new();
//     while let Some(chunk) = stream.next().await {
//         let chunk = chunk?;
//         pb.inc(chunk.len() as u64);
//         downloaded_data.extend_from_slice(&chunk);
//     }
//     pb.finish_with_message(format!("Downloaded plugin {}", name));
//     let mut tar = tar::Archive::new(std::io::Cursor::new(downloaded_data));
//     tar.unpack("plugins")?;
//     Ok(())
// }

// async fn download_plugins(manifest: &Manifest, channels: &Channels) -> Result<Plugins> {
//     let mut dependencies = vec![];
//     if let Some(deps) = manifest.dependencies.clone() {
//         for (name, dep) in deps {
//             match dep {
//                 Dependency::Version(version) => {}
//                 Dependency::Channel {
//                     channel,
//                     version,
//                     feature_flags,
//                 } => {
//                     let channel_name = match channel {
//                         Some(channel) => channel,
//                         None => String::from("default"),
//                     };
//                     let channel = channels
//                         .get(&channel_name)
//                         .ok_or_else(|| anyhow::anyhow!("Channel {} not found", channel_name))?;
//                     let channel = channel.clone();
//                     download_plugin_from_channel(channel, version).await?
//                 }
//                 Dependency::GitHub {
//                     owner,
//                     name,
//                     tag,
//                     feature_flags,
//                 } => download_plugin_from_github(owner, name, tag).await?,
//                 Dependency::Local {
//                     path,
//                     feature_flags,
//                 } => download_plugin_from_local(path, feature_flags).await?,
//             }
//         }
//     }
//     Plugins::new(dependencies)
// }

// async fn download_channels(manifest: &Manifest) -> Result<Channels> {
//     let mut result: Channels = HashMap::new();
//     if let Some(channels) = manifest.channels.clone() {
//         for (name, url) in channels {
//             let pb = get_brack_progress_bar();
//             pb.set_message(format!("Downloading channel {}", name));
//             let response = reqwest::get(&url).await?;
//             if !response.status().is_success() {
//                 pb.finish_and_clear();
//                 anyhow::bail!(
//                     "Failed to download channel from {}.\nStatus: {} - {}",
//                     url,
//                     response.status().as_str(),
//                     response
//                         .status()
//                         .canonical_reason()
//                         .unwrap_or("Unknown error")
//                 );
//             }
//             if let Some(size) = response.content_length() {
//                 pb.set_length(size);
//             }
//             let mut stream = response.bytes_stream();
//             let mut downloaded_data = Vec::new();
//             while let Some(chunk) = stream.next().await {
//                 let chunk = chunk?;
//                 pb.inc(chunk.len() as u64);
//                 downloaded_data.extend_from_slice(&chunk);
//             }
//             pb.finish_with_message(format!("Downloaded channel {}", name));
//             let downloaded_data_str = std::str::from_utf8(&downloaded_data)?;
//             if let Ok(channel) = toml::from_str::<Channel>(downloaded_data_str) {
//                 result.insert(name, channel);
//             } else {
//                 anyhow::bail!("Failed to parse channel from {}", name);
//             }
//         }
//     }
//     Ok(result)
// }

const DEFAULT_PROJECT_SRC_PATH: &str = "docs";
const DEFAULT_PROJECT_TARGET_PATH: &str = "target";

fn try_create_dir<P: AsRef<Path>, L: Logger>(path: &P, logger: &L) -> Result<(), ProjectError> {
    let path = path.as_ref().to_path_buf();
    if path.exists() {
        // logger.debug(&ProjectDebug::DirectoryAlreadyExists { path });
        return Ok(());
    }
    match create_dir_all(&path) {
        Ok(_) => {
            logger.debug(&ProjectDebug::CreatingDirectory { path });
            Ok(())
        }
        Err(err) => {
            let err = ProjectError::FailedToCreateDirectory { path, source: err };
            logger.error(&err);
            Err(err)
        }
    }
}

fn try_write<P: AsRef<Path>, L: Logger>(
    path: &P,
    content: &str,
    logger: &L,
) -> Result<(), ProjectError> {
    let path = path.as_ref().to_path_buf();
    match write(&path, content) {
        Ok(_) => {
            logger.debug(&ProjectDebug::WritingFile {
                path,
                content: content.to_string(),
            });
            Ok(())
        }
        Err(err) => {
            let err = ProjectError::FailedToWriteFile { path, source: err };
            logger.error(&err);
            Err(err)
        }
    }
}

fn try_read_to_string<P: AsRef<Path>, L: Logger>(
    path: &P,
    logger: &L,
) -> Result<String, ProjectError> {
    let path = path.as_ref().to_path_buf();
    match read_to_string(&path) {
        Ok(content) => {
            // logger.log(ProjectDebug::ReadingFile(path_str, content.clone()).into());
            Ok(content)
        }
        Err(source) => {
            let err = ProjectError::FailedToReadFile { path, source };
            logger.error(&err);
            Err(err)
        }
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            manifest: Manifest::default(),
            project_root: None,
            plugins_metadata: HashMap::default(),
            channels: None,
        }
    }

    /// If the Brack.toml file exists, load it instead of `new`
    pub fn new_with_manifest<P: AsRef<Path>, L: Logger>(
        logger: &L,
        project_root: P,
    ) -> Result<Self, ProjectError> {
        let path = project_root.as_ref().join(BRACK_TOML);
        if !path.exists() {
            let err = ProjectError::ManifestNotFound { path };
            logger.error(&err);
            return Err(err);
        }
        let file = try_read_to_string(&path, logger)?;
        let manifest = toml::from_str(&file).map_err(|source| {
            let err = ProjectError::FailedToDeserializeManifest { source };
            logger.error(&err);
            err
        })?;
        Ok(Self {
            manifest,
            project_root: Some(project_root.as_ref().to_path_buf()),
            plugins_metadata: HashMap::default(),
            // fix: manifest.channels to Channels
            channels: None,
        })
    }

    /// Corresponds to `brack create` command
    pub fn create_document<P: AsRef<Path>, L: Logger>(
        &mut self,
        logger: &L,
        path: P,
    ) -> Result<(), ProjectError> {
        let path = path.as_ref();
        let name = path
            .file_name()
            .ok_or_else(|| {
                let err = ProjectError::FailedToGetFileNameFromPath {
                    path: path.to_path_buf(),
                };
                logger.error(&err);
                err
            })?
            .to_str()
            .ok_or_else(|| {
                let err = ProjectError::FailedToConvertPathToStr {
                    path: path.to_path_buf(),
                };
                logger.error(&err);
                err
            })?
            .to_string();

        logger.info(&ProjectInfo::CreatingProject { name: name.clone() });

        if let Some(project_root) = &self.project_root {
            let err = ProjectError::ProjectAlreadyExists {
                path: project_root.to_path_buf(),
            };
            logger.error(&err);
            return Err(err);
        }

        if path.exists() {
            let err = ProjectError::DirectoryAlreadyExists {
                path: path.to_path_buf(),
            };
            logger.error(&err);
            return Err(err);
        }

        try_create_dir(&path, logger)?;
        try_create_dir(&path.join(DEFAULT_PROJECT_SRC_PATH), logger)?;
        try_write(
            &path.join(format!("{}/main.[]", DEFAULT_PROJECT_SRC_PATH)),
            "Hello, Brack!",
            logger,
        )?;

        self.project_root = Some(path.to_path_buf());
        self.manifest.name = name.clone();
        self.manifest.version = String::from("0.1.0");
        self.manifest.authors = Some(vec![Author {
            name: String::from("your name"),
            email: String::from("you@example.com"),
        }]);
        self.manifest.dependencies = Some(HashMap::default());
        self.manifest.document = Some(DocumentSettings {
            target: String::from("html"),
            output_level: None,
            output_format: None,
            src: String::from(DEFAULT_PROJECT_SRC_PATH),
        });
        self.write_manifest(logger)?;
        logger.info(&ProjectInfo::FinishedCreatingProject { name });
        Ok(())
    }

    pub fn write_manifest<L: Logger>(&self, logger: &L) -> Result<(), ProjectError> {
        match &self.project_root {
            Some(project_root) => {
                let manifest_path = project_root.join(BRACK_TOML);
                let file = match toml::to_string(&self.manifest) {
                    Ok(file) => file,
                    Err(err) => {
                        let err = ProjectError::FailedToSerializeManifest { source: err };
                        logger.error(&err);
                        return Err(err);
                    }
                };
                try_write(&manifest_path, &file, logger)?;
            }
            _ => {
                let err = ProjectError::UninitializedProject;
                logger.error(&err);
                return Err(err);
            }
        };
        Ok(())
    }

    pub async fn build<L: Logger>(&self, logger: &mut L) -> Result<(), ProjectError> {
        // let channels = download_channels(&self.manifest).await?;
        // let _plugins = download_plugins(&self.manifest, &channels).await?;
        let mut plugins = match brack_plugin::plugins::Plugins::new(vec![]) {
            Ok(plugins) => plugins,
            Err(_) => {
                let err = ProjectError::FailedToCreatePlugin;
                logger.error(&err);
                return Err(err);
            }
        };
        let name = self.manifest.name.clone();
        logger.info(&ProjectInfo::BuildingProject { name: name.clone() });
        if self.project_root.is_none() {
            let err = ProjectError::UninitializedProject;
            logger.error(&err);
            return Err(err);
        }
        let project_root = self.project_root.as_ref().unwrap();
        let docs_path = project_root.join(DEFAULT_PROJECT_SRC_PATH);
        let target = match &self.manifest.document {
            Some(document) => &document.target,
            _ => {
                let err = ProjectError::DocumentSettingsNotFound;
                logger.error(&err);
                return Err(err);
            }
        };
        let target_path = project_root.join(DEFAULT_PROJECT_TARGET_PATH).join(target);
        try_create_dir(&project_root.join(DEFAULT_PROJECT_TARGET_PATH), logger)?;
        try_create_dir(&target_path, logger)?;
        let mut has_transform_error = false;
        for entry in WalkDir::new(&docs_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            logger.set_path(path.to_path_buf());
            let file_name = path.file_name().unwrap().to_str().unwrap();
            logger.debug(&ProjectDebug::BuildingFile {
                path: path.to_path_buf(),
                file_name: file_name.to_string(),
            });
            if !file_name.ends_with(".[]") {
                continue;
            }
            let file = match read_to_string(path) {
                Ok(file) => file,
                Err(source) => {
                    let err = ProjectError::FailedToReadFile {
                        path: path.to_path_buf(),
                        source,
                    };
                    logger.error(&err);
                    return Err(err);
                }
            };
            let tokens = brack_tokenizer::tokenize::tokenize(&file);
            let cst = brack_parser::parse::parse(&tokens);
            let (ast, errors) = brack_transformer::transform::transform(&cst);
            if !errors.is_empty() {
                for error in errors {
                    logger.error(&error.into());
                }
                has_transform_error = true;
            }
            let east = match brack_expander::expand::expander(&ast, &mut plugins) {
                Ok(east) => east,
                Err(_) => {
                    let err = ProjectError::ExpandError;
                    logger.error(&err);
                    return Err(err);
                }
            };
            let result = match brack_codegen::generate::generate(&east, &mut plugins) {
                Ok(result) => result,
                Err(_) => {
                    let err = ProjectError::CodegenError;
                    logger.error(&err);
                    return Err(err);
                }
            };
            let out_dir =
                target_path.join(path.strip_prefix(&docs_path).unwrap().parent().unwrap());
            let out_path = target_path.join(
                path.strip_prefix(&docs_path)
                    .unwrap()
                    .with_extension(target),
            );
            try_create_dir(&out_dir, logger)?;
            try_write(&out_path, &result, logger)?;
        }
        if has_transform_error {
            return Err(ProjectError::TransformError);
        }
        logger.info(&ProjectInfo::FinishedBuildingProject { name: name.clone() });
        Ok(())
    }

    /// Corresponds to `brack clean` command
    pub fn clean<L: Logger>(&self, logger: &L, dry_run: bool) -> Result<(), ProjectError> {
        logger.info(&ProjectInfo::CleaningProject);
        if self.project_root.is_none() {
            let err = ProjectError::UninitializedProject;
            logger.error(&err);
            return Err(err);
        }
        let project_root = self.project_root.as_ref().unwrap();
        let target_path = project_root.join(DEFAULT_PROJECT_TARGET_PATH);
        let mut num_files = 0;
        let mut file_size = 0;
        for entry in WalkDir::new(&target_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            num_files += 1;
            match path.metadata() {
                Ok(meta) => {
                    file_size += meta.len();
                }
                Err(source) => {
                    let err = ProjectError::FailedToReadFileSize {
                        path: path.to_path_buf(),
                        source,
                    };
                    logger.error(&err);
                    return Err(err);
                }
            }
            logger.debug(&ProjectDebug::RemoveFile {
                path: path.to_path_buf(),
            });
            if !dry_run {
                remove_file(path).map_err(|source| {
                    let err = ProjectError::FailedToRemoveFile {
                        path: path.to_path_buf(),
                        source,
                    };
                    logger.error(&err);
                    err
                })?;
            }
        }
        logger.debug(&ProjectDebug::RemoveDir {
            path: target_path.clone(),
        });
        if !dry_run && target_path.exists() {
            remove_dir_all(&target_path).map_err(|source| {
                let err = ProjectError::FailedToRemoveDir {
                    path: target_path,
                    source,
                };
                logger.error(&err);
                err
            })?;
        }
        logger.info(&ProjectInfo::FinishedCleaningProject {
            num_files,
            file_size,
        });
        if dry_run {
            logger.warn(&ProjectWarning::NoFilesRemovedDueToDryRun);
        }
        Ok(())
    }

    pub async fn add_channel<L: Logger>(
        &mut self,
        logger: &L,
        name: &str,
        url: &str,
    ) -> Result<(), ProjectError> {
        logger.info(&ProjectInfo::AddingChannel {
            name: name.to_string(),
        });
        if self.project_root.is_none() {
            let err = ProjectError::UninitializedProject;
            logger.error(&err);
            return Err(err);
        }
        let mut channels = self.manifest.channels.clone().unwrap_or_default();
        if channels.contains_key(name) {
            let err = ProjectError::ChannelAlreadyExists {
                name: name.to_string(),
            };
            logger.error(&err);
            return Err(err);
        }
        if name.is_empty() {
            let err = ProjectError::ChannelNameCannotBeEmpty;
            logger.error(&err);
            return Err(err);
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            let err = ProjectError::InvalidChannelName {
                name: name.to_string(),
            };
            logger.error(&err);
            return Err(err);
        }
        if url.is_empty() {
            let err = ProjectError::ChannelUrlCannotBeEmpty;
            logger.error(&err);
            return Err(err);
        }
        if url.starts_with("http") {
            let _channel = fetch_channel(logger, url).await?;
        } else {
            let err = ProjectError::InvalidChannelUrl {
                url: url.to_string(),
            };
            logger.error(&err);
            return Err(err);
        }
        channels.insert(name.to_string(), url.to_string());
        self.manifest.channels = Some(channels);
        self.write_manifest(logger)?;
        logger.info(&ProjectInfo::FinishedAddingChannel {
            name: name.to_string(),
        });
        Ok(())
    }

    pub fn remove_channel<L: Logger>(
        &mut self,
        logger: &L,
        name: &str,
    ) -> Result<(), ProjectError> {
        logger.info(&ProjectInfo::RemovingChannel {
            name: name.to_string(),
        });
        if self.project_root.is_none() {
            let err = ProjectError::UninitializedProject;
            logger.error(&err);
            return Err(err);
        }
        let mut channels = self.manifest.channels.clone().unwrap_or_default();
        if name.is_empty() {
            let err = ProjectError::ChannelNameCannotBeEmpty;
            logger.error(&err);
            return Err(err);
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            let err = ProjectError::InvalidChannelName {
                name: name.to_string(),
            };
            logger.error(&err);
            return Err(err);
        }
        if !channels.contains_key(name) {
            let err = ProjectError::ChannelNotFound {
                name: name.to_string(),
            };
            logger.error(&err);
            return Err(err);
        }
        channels.remove(name);
        self.manifest.channels = Some(channels);
        self.write_manifest(logger)?;
        logger.info(&ProjectInfo::FinishedRemovingChannel {
            name: name.to_string(),
        });
        Ok(())
    }

    pub async fn update_channel<L: Logger>(
        &self,
        logger: &L,
        name: Option<String>,
    ) -> Result<(), ProjectError> {
        logger.info(&ProjectInfo::UpdatingChannel { name: name.clone() });
        if self.project_root.is_none() {
            let err = ProjectError::UninitializedProject;
            logger.error(&err);
            return Err(err);
        }
        let channels = self.manifest.channels.clone().unwrap_or_default();
        match name {
            Some(ref name) => {
                if !channels.contains_key(name) {
                    let err = ProjectError::ChannelNotFound { name: name.clone() };
                    logger.error(&err);
                    return Err(err);
                }
                let url = channels.get(name).unwrap();
                let _channel = fetch_channel(logger, url).await?;
            }
            _ => {
                for (_name, url) in channels {
                    let _channel = fetch_channel(logger, &url).await?;
                }
            }
        }
        logger.info(&ProjectInfo::FinishedUpdatingChannel { name: name.clone() });
        Ok(())
    }

    pub async fn list_channels<L: Logger>(&self, logger: &L) -> Result<(), ProjectError> {
        logger.info(&ProjectInfo::ListingChannels);
        if self.project_root.is_none() {
            let err = ProjectError::UninitializedProject;
            logger.error(&err);
            return Err(err);
        }
        let channels = self.manifest.channels.clone().unwrap_or_default();
        if channels.is_empty() {
            logger.warn(&ProjectWarning::NoChannelsFound);
        } else {
            for (name, url) in channels {
                let health = channel_health_check(&url).await;
                logger.info(&ProjectInfo::ChannelInfo { name, url, health });
            }
        }
        logger.info(&ProjectInfo::FinishedListingChannels);
        Ok(())
    }
}

async fn channel_health_check(url: &str) -> bool {
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_) => return false,
    };
    if !response.status().is_success() {
        return false;
    }
    let downloaded_data = match response.bytes().await {
        Ok(data) => data,
        Err(_) => return false,
    };
    let downloaded_data_str = match std::str::from_utf8(&downloaded_data) {
        Ok(data) => data,
        Err(_) => return false,
    };
    let channel: Channel = match toml::from_str(downloaded_data_str) {
        Ok(channel) => channel,
        Err(_) => return false,
    };
    if channel.is_empty() {
        return false;
    }
    true
}

async fn fetch_channel<L: Logger>(logger: &L, url: &str) -> Result<Channel, ProjectError> {
    let response = reqwest::get(url).await.map_err(|source| {
        let err = ProjectError::FailedToFetchResource {
            url: url.to_string(),
            source,
        };
        logger.error(&err);
        err
    })?;
    if !response.status().is_success() {
        let status = response.status();
        let reason = status.canonical_reason().unwrap_or("Unknown reqwest error");
        let err = ProjectError::ReceivedHttpNonSuccessStatus {
            url: url.to_string(),
            status,
            err_msg: reason.to_string(),
        };
        logger.error(&err);
        return Err(err);
    }
    let downloaded_data = response.bytes().await.map_err(|source| {
        let err = ProjectError::FailedToFetchResource {
            url: url.to_string(),
            source,
        };
        logger.error(&err);
        err
    })?;
    let downloaded_data_str = std::str::from_utf8(&downloaded_data).map_err(|source| {
        let err = ProjectError::FailedToConvertBytesToStr { source };
        logger.error(&err);
        err
    })?;
    let channel: Channel = toml::from_str(downloaded_data_str).map_err(|source| {
        let err = ProjectError::FailedToParseChannel { source };
        logger.error(&err);
        err
    })?;
    Ok(channel)
}
