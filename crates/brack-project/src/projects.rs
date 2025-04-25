use crate::channels::{check_valid_channel_name, ChannelProvider, Channels};
use crate::manifest::{Author, DocumentSettings, Manifest};
use crate::utils::{
    create_dir, ensure_directory_not_exists, ensure_manifest_exists, ensure_project_exists,
    ensure_project_not_exists, get_file_name, get_file_size, get_manifest, get_project_root,
    remove_dir, remove_file, write, write_manifest,
};
use anyhow::Result;
use brack_common::errors::{Error, ProjectError, ProjectInfo, ProjectResultExt, ProjectWarning};
use brack_common::logger::Logger;
use brack_plugin::feature_flag::FeatureFlag;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Project {
    pub manifest: Manifest,
    pub project_root: Option<PathBuf>,
    pub channels: Option<Channels>,
    // todo: delete this field
    pub plugins_metadata: HashMap<String, (String, FeatureFlag)>,
}

const DEFAULT_PROJECT_SRC_PATH: &str = "docs";
const DEFAULT_PROJECT_TARGET_PATH: &str = "target";

impl Default for Project {
    fn default() -> Self {
        Self::new()
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
        project_root: &P,
    ) -> Result<Self, Error> {
        ensure_manifest_exists(project_root).map_project_err(logger)?;
        Ok(Self {
            manifest: get_manifest(project_root, logger).map_project_err(logger)?,
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
    ) -> Result<(), Error> {
        let path = path.as_ref();
        let name = get_file_name(&path).map_project_err(logger)?;

        logger.info(&ProjectInfo::CreatingProject { name: name.clone() }.into());

        ensure_project_not_exists(self).map_project_err(logger)?;
        ensure_directory_not_exists(&path).map_project_err(logger)?;

        create_dir(&path.join(DEFAULT_PROJECT_SRC_PATH), logger).map_project_err(logger)?;
        write(
            &path.join(DEFAULT_PROJECT_SRC_PATH).join("main.[]"),
            "Hello, Brack!",
            logger,
        )
        .map_project_err(logger)?;

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
        write_manifest(self, logger).map_project_err(logger)?;

        logger.info(&ProjectInfo::FinishedCreatingProject { name }.into());
        Ok(())
    }

    pub async fn build<L: Logger>(&self, _logger: &mut L) -> Result<(), ProjectError> {
        // let channels = download_channels(&self.manifest).await?;
        // let _plugins = download_plugins(&self.manifest, &channels).await?;
        // let mut plugins = match brack_plugin::plugins::Plugins::new(vec![]) {
        //     Ok(plugins) => plugins,
        //     Err(_) => {
        //         let err = ProjectError::FailedToCreatePlugin;
        //         logger.error(&err);
        //         return Err(err);
        //     }
        // };
        // let name = self.manifest.name.clone();
        // logger.info(&ProjectInfo::BuildingProject { name: name.clone() });
        // if self.project_root.is_none() {
        //     let err = ProjectError::UninitializedProject;
        //     logger.error(&err);
        //     return Err(err);
        // }
        // let project_root = self.project_root.as_ref().unwrap();
        // let docs_path = project_root.join(DEFAULT_PROJECT_SRC_PATH);
        // let target = match &self.manifest.document {
        //     Some(document) => &document.target,
        //     _ => {
        //         let err = ProjectError::DocumentSettingsNotFound;
        //         logger.error(&err);
        //         return Err(err);
        //     }
        // };
        // let target_path = project_root.join(DEFAULT_PROJECT_TARGET_PATH).join(target);
        // try_create_dir(&project_root.join(DEFAULT_PROJECT_TARGET_PATH), logger)?;
        // try_create_dir(&target_path, logger)?;
        // let mut has_transform_error = false;
        // for entry in WalkDir::new(&docs_path).into_iter().filter_map(|e| e.ok()) {
        //     let path = entry.path();
        //     if !path.is_file() {
        //         continue;
        //     }
        //     logger.set_path(path.to_path_buf());
        //     let file_name = path.file_name().unwrap().to_str().unwrap();
        //     logger.debug(&ProjectDebug::BuildingFile {
        //         path: path.to_path_buf(),
        //         file_name: file_name.to_string(),
        //     });
        //     if !file_name.ends_with(".[]") {
        //         continue;
        //     }
        //     let file = match read_to_string(path) {
        //         Ok(file) => file,
        //         Err(source) => {
        //             let err = ProjectError::FailedToReadFile {
        //                 path: path.to_path_buf(),
        //                 source,
        //             };
        //             logger.error(&err);
        //             return Err(err);
        //         }
        //     };
        //     let tokens = brack_tokenizer::tokenize::tokenize(&file);
        //     let cst = brack_parser::parse::parse(&tokens);
        //     let (ast, errors) = brack_transformer::transform::transform(&cst);
        //     if !errors.is_empty() {
        //         for error in errors {
        //             logger.error(&error.into());
        //         }
        //         has_transform_error = true;
        //     }
        //     let east = match brack_expander::expand::expander(&ast, &mut plugins) {
        //         Ok(east) => east,
        //         Err(_) => {
        //             let err = ProjectError::ExpandError;
        //             logger.error(&err);
        //             return Err(err);
        //         }
        //     };
        //     let result = match brack_codegen::generate::generate(&east, &mut plugins) {
        //         Ok(result) => result,
        //         Err(_) => {
        //             let err = ProjectError::CodegenError;
        //             logger.error(&err);
        //             return Err(err);
        //         }
        //     };
        //     let out_dir =
        //         target_path.join(path.strip_prefix(&docs_path).unwrap().parent().unwrap());
        //     let out_path = target_path.join(
        //         path.strip_prefix(&docs_path)
        //             .unwrap()
        //             .with_extension(target),
        //     );
        //     try_create_dir(&out_dir, logger)?;
        //     try_write(&out_path, &result, logger)?;
        // }
        // if has_transform_error {
        //     return Err(ProjectError::TransformError);
        // }
        // logger.info(&ProjectInfo::FinishedBuildingProject { name: name.clone() });
        Ok(())
    }

    /// Corresponds to `brack clean` command
    pub fn clean<L: Logger>(&self, logger: &L, dry_run: bool) -> Result<(), Error> {
        ensure_project_exists(self).map_project_err(logger)?;
        logger.info(
            &ProjectInfo::CleaningProject {
                name: self.manifest.name.clone(),
            }
            .into(),
        );
        let project_root = self.project_root.as_ref().unwrap();
        let target_path = project_root.join(DEFAULT_PROJECT_TARGET_PATH);
        let mut num_files = 0;
        let mut file_size = 0;
        let files = WalkDir::new(&target_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());
        for file in files {
            let path = file.path();
            num_files += 1;
            file_size += get_file_size(&path).map_project_err(logger)?;
            if !dry_run {
                remove_file(&path, logger).map_project_err(logger)?;
            }
        }
        if !dry_run && target_path.exists() {
            remove_dir(&target_path, logger).map_project_err(logger)?;
        }
        logger.info(
            &ProjectInfo::FinishedCleaningProject {
                name: self.manifest.name.clone(),
                num_files,
                file_size,
            }
            .into(),
        );
        if dry_run {
            logger.warn(&ProjectWarning::NoFilesRemovedDueToDryRun.into());
        }
        Ok(())
    }

    pub async fn add_channel<L: Logger>(
        &mut self,
        logger: &L,
        name: &str,
        url: &str,
    ) -> Result<(), Error> {
        logger.info(
            &ProjectInfo::AddingChannel {
                name: name.to_string(),
            }
            .into(),
        );

        ensure_project_exists(self).map_project_err(logger)?;
        let mut channels = self.manifest.channels.clone().unwrap_or_default();
        if channels.contains_key(name) {
            return Err(ProjectError::ChannelAlreadyExists {
                name: name.to_string(),
            }
            .into());
        }
        let channel_provider = ChannelProvider::new(name, url).map_project_err(logger)?;
        let channel = channel_provider
            .fetch(logger)
            .await
            .map_project_err(logger)?;
        let project_root = get_project_root(self)?;
        channel_provider.write(&project_root, &channel, logger)?;
        channels.insert(name.to_string(), url.to_string());
        self.manifest.channels = Some(channels);
        write_manifest(self, logger).map_project_err(logger)?;

        logger.info(
            &ProjectInfo::FinishedAddingChannel {
                name: name.to_string(),
            }
            .into(),
        );
        Ok(())
    }

    pub fn remove_channel<L: Logger>(&mut self, logger: &L, name: &str) -> Result<(), Error> {
        logger.info(
            &ProjectInfo::RemovingChannel {
                name: name.to_string(),
            }
            .into(),
        );

        ensure_project_exists(self).map_project_err(logger)?;
        let mut channels = self.manifest.channels.clone().unwrap_or_default();
        check_valid_channel_name(name).map_project_err(logger)?;
        if !channels.contains_key(name) {
            return Err(ProjectError::ChannelNotFound {
                name: name.to_string(),
            }
            .into());
        }
        channels.remove(name);
        self.manifest.channels = Some(channels);
        write_manifest(self, logger).map_project_err(logger)?;

        logger.info(
            &ProjectInfo::FinishedRemovingChannel {
                name: name.to_string(),
            }
            .into(),
        );
        Ok(())
    }

    pub async fn update_channel<L: Logger>(
        &self,
        logger: &L,
        name: Option<String>,
    ) -> Result<(), Error> {
        logger.info(&ProjectInfo::UpdatingChannel { name: name.clone() }.into());

        ensure_project_exists(self).map_project_err(logger)?;
        let channels = self.manifest.channels.clone().unwrap_or_default();
        let channels = match name {
            Some(ref name) => {
                if !channels.contains_key(name) {
                    return Err(ProjectError::ChannelNotFound { name: name.clone() }.into());
                }
                let url = channels.get(name).unwrap();
                HashMap::from([(name.clone(), url.clone())])
            }
            _ => channels.clone(),
        };
        for (name, url) in channels {
            let channel_provider = ChannelProvider::new(&name, &url).map_project_err(logger)?;
            let channel = channel_provider
                .fetch(logger)
                .await
                .map_project_err(logger)?;
            let project_root = get_project_root(self)?;
            channel_provider.write(&project_root, &channel, logger)?;
        }

        logger.info(&ProjectInfo::FinishedUpdatingChannel { name: name.clone() }.into());
        Ok(())
    }

    pub async fn list_channels<L: Logger>(&self, logger: &L) -> Result<(), Error> {
        logger.info(&ProjectInfo::ListingChannels.into());

        ensure_project_exists(self).map_project_err(logger)?;
        let channels = self.manifest.channels.clone().unwrap_or_default();
        if channels.is_empty() {
            logger.warn(&ProjectWarning::NoChannelsFound.into());
        } else {
            for (name, url) in channels {
                let channels_provider =
                    ChannelProvider::new(&name, &url).map_project_err(logger)?;
                let health = channels_provider.fetch(logger).await.is_ok();
                logger.info(&ProjectInfo::ChannelInfo { name, url, health }.into());
            }
        }

        logger.info(&ProjectInfo::FinishedListingChannels.into());
        Ok(())
    }
}
