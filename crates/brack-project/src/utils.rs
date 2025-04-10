use std::path::{Path, PathBuf};

use brack_common::{
    errors::{Debug, ProjectDebug, ProjectError},
    logger::Logger,
};

use crate::{manifest::Manifest, projects::Project};

const BRACK_TOML: &str = "Brack.toml";
pub const DEFAULT_PROJECT_TARGET_PATH: &str = "target";

pub fn get_file_name<P: AsRef<Path>>(path: &P) -> Result<String, ProjectError> {
    let path = path.as_ref();
    let os_str = path
        .file_name()
        .ok_or_else(|| ProjectError::FailedToGetFileNameFromPath {
            path: path.to_path_buf(),
        })?;
    let result = os_str
        .to_str()
        .ok_or_else(|| ProjectError::FailedToConvertOsStrToStr {
            os_str: os_str.to_os_string(),
        })?
        .to_string();
    Ok(result)
}

pub fn ensure_project_not_exists(project: &Project) -> Result<(), ProjectError> {
    if let Some(project_root) = &project.project_root {
        return Err(ProjectError::ProjectAlreadyExists {
            path: project_root.clone(),
        });
    }
    Ok(())
}

pub fn ensure_project_exists(project: &Project) -> Result<(), ProjectError> {
    if project.project_root.is_none() {
        return Err(ProjectError::ProjectNotInitialized);
    }
    Ok(())
}

pub fn get_project_root(project: &Project) -> Result<PathBuf, ProjectError> {
    if let Some(project_root) = &project.project_root {
        return Ok(project_root.clone());
    }
    Err(ProjectError::ProjectNotInitialized)
}

pub fn ensure_directory_not_exists<P: AsRef<Path>>(path: &P) -> Result<(), ProjectError> {
    let path = path.as_ref();
    if path.exists() {
        return Err(ProjectError::DirectoryAlreadyExists {
            path: path.to_path_buf(),
        });
    }
    Ok(())
}

pub fn create_dir<P: AsRef<Path>, L: Logger>(path: &P, logger: &L) -> Result<(), ProjectError> {
    let path = path.as_ref().to_path_buf();
    if path.exists() {
        logger.debug(&Debug::ProjectDebug(ProjectDebug::DirectoryAlreadyExists {
            path: path.to_path_buf(),
        }));
        return Ok(());
    }
    match std::fs::create_dir_all(&path) {
        Ok(_) => {
            logger.debug(&Debug::ProjectDebug(ProjectDebug::CreatingDirectory {
                path: path.to_path_buf(),
            }));
            Ok(())
        }
        Err(source) => Err(ProjectError::FailedToCreateDirectory {
            path: path.to_path_buf(),
            source,
        }),
    }
}

pub fn write<P: AsRef<Path>, L: Logger>(
    path: &P,
    content: &str,
    logger: &L,
) -> Result<(), ProjectError> {
    let path = path.as_ref().to_path_buf();
    match std::fs::write(&path, content) {
        Ok(_) => {
            logger.debug(&Debug::ProjectDebug(ProjectDebug::WritingFile {
                path,
                content: content.to_string(),
            }));
            Ok(())
        }
        Err(source) => Err(ProjectError::FailedToWriteFile { path, source }),
    }
}

pub fn toml_to_string<T, L>(value: &T, logger: &L) -> Result<String, ProjectError>
where
    T: serde::Serialize + ?Sized,
    L: Logger,
{
    match toml::to_string(value) {
        Ok(toml_string) => {
            logger.debug(
                &ProjectDebug::WritingToml {
                    content: toml_string.clone(),
                }
                .into(),
            );
            Ok(toml_string)
        }
        Err(source) => Err(ProjectError::FailedToSerializeManifest { source }),
    }
}

pub fn write_manifest<L: Logger>(project: &Project, logger: &L) -> Result<(), ProjectError> {
    let project_root = get_project_root(project)?;
    let path = project_root.join(BRACK_TOML);
    let content = toml_to_string(&project.manifest, logger)?;
    write(&path, &content, logger)?;
    Ok(())
}

pub fn ensure_manifest_exists<P: AsRef<Path>>(project_root: &P) -> Result<(), ProjectError> {
    let path = project_root.as_ref().join(BRACK_TOML);
    if !path.exists() {
        return Err(ProjectError::ManifestNotFound { path });
    }
    Ok(())
}

pub fn read_to_string<P: AsRef<Path>, L: Logger>(
    path: &P,
    logger: &L,
) -> Result<String, ProjectError> {
    let path = path.as_ref().to_path_buf();
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            logger.debug(
                &ProjectDebug::ReadingFile {
                    path,
                    content: content.clone(),
                }
                .into(),
            );
            Ok(content)
        }
        Err(source) => Err(ProjectError::FailedToReadFile { path, source }),
    }
}

pub fn toml_from_str<T, L: Logger>(content: &str, logger: &L) -> Result<T, ProjectError>
where
    T: serde::de::DeserializeOwned,
{
    match toml::from_str(content) {
        Ok(value) => {
            logger.debug(
                &ProjectDebug::ReadingToml {
                    content: content.to_string(),
                }
                .into(),
            );
            Ok(value)
        }
        Err(source) => Err(ProjectError::FailedToDeserializeManifest { source }),
    }
}

pub fn get_manifest<P: AsRef<Path>, L: Logger>(
    project_root: &P,
    logger: &L,
) -> Result<Manifest, ProjectError> {
    let manifest_file_path = project_root.as_ref().join(BRACK_TOML);
    let manifest_file = read_to_string(&manifest_file_path, logger)?;
    toml_from_str::<Manifest, _>(&manifest_file, logger)
}

pub fn get_file_size<P: AsRef<Path>>(path: &P) -> Result<u64, ProjectError> {
    let path = path.as_ref();
    match std::fs::metadata(&path) {
        Ok(metadata) => Ok(metadata.len()),
        Err(source) => Err(ProjectError::FailedToReadFileSize {
            path: path.to_path_buf(),
            source,
        }),
    }
}

pub fn remove_file<P: AsRef<Path>, L: Logger>(path: &P, logger: &L) -> Result<(), ProjectError> {
    let path = path.as_ref().to_path_buf();
    match std::fs::remove_file(&path) {
        Ok(()) => {
            logger.debug(&ProjectDebug::RemovingFile { path }.into());
            Ok(())
        }
        Err(source) => Err(ProjectError::FailedToRemoveFile { path, source }),
    }
}

pub fn remove_dir<P: AsRef<Path>, L: Logger>(path: &P, logger: &L) -> Result<(), ProjectError> {
    let path = path.as_ref().to_path_buf();
    match std::fs::remove_dir_all(&path) {
        Ok(()) => {
            logger.debug(&ProjectDebug::RemovingDirectory { path }.into());
            Ok(())
        }
        Err(source) => Err(ProjectError::FailedToRemoveDirectory { path, source }),
    }
}
