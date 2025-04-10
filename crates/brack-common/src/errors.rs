use std::{ffi::OsString, io, path::PathBuf};

use crate::{location::Location, logger::Logger};

pub enum Error {
    TransformingError(TransformingError),
    LoweringError(LoweringError),
    CodegenError(CodegenError),
    ProjectError(ProjectError),
    ReleaseError(ReleaseError),
    InternalError { panic_message: String },
}

impl Error {
    pub fn code(&self) -> String {
        match self {
            Self::TransformingError(err) => err.code(),
            Self::LoweringError(err) => err.code(),
            Self::CodegenError(err) => err.code(),
            Self::ProjectError(err) => err.code(),
            Self::ReleaseError(err) => err.code(),
            Self::InternalError { .. } => String::from("Fatal"),
        }
    }
}

pub enum TransformingError {
    AngleNotOpened(Location),
    AngleNotClosed(Location),
    CurlyNotOpened(Location),
    CurlyNotClosed(Location),
    SquareNotOpened(Location),
    SquareNotClosed(Location),
    MismatchedBracket(Location),
    ModuleNotFound(Location),
    IdentifierNotFound(Location),
    DotNotFound(Location),
    CommaNotFound(Location),
    UnexpectedDot(Location),
    UnexpectedComma(Location),
    InvalidBackslash(Location),
}

impl TransformingError {
    pub fn code(&self) -> String {
        match self {
            Self::AngleNotOpened(_) => String::from("ET001"),
            Self::AngleNotClosed(_) => String::from("ET002"),
            Self::CurlyNotOpened(_) => String::from("ET003"),
            Self::CurlyNotClosed(_) => String::from("ET004"),
            Self::SquareNotOpened(_) => String::from("ET005"),
            Self::SquareNotClosed(_) => String::from("ET006"),
            Self::MismatchedBracket(_) => String::from("ET007"),
            Self::ModuleNotFound(_) => String::from("ET008"),
            Self::IdentifierNotFound(_) => String::from("ET009"),
            Self::DotNotFound(_) => String::from("ET010"),
            Self::CommaNotFound(_) => String::from("ET011"),
            Self::UnexpectedDot(_) => String::from("ET012"),
            Self::UnexpectedComma(_) => String::from("ET013"),
            Self::InvalidBackslash(_) => String::from("ET014"),
        }
    }
}

pub enum LoweringError {}

impl LoweringError {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("EL001"),
        }
    }
}

pub enum CodegenError {}

impl CodegenError {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("EC001"),
        }
    }
}

pub enum ProjectError {
    /// This error is used when the following function returns None:
    /// file_name(self: &std::path::Path) -> Option<&OsStr>
    FailedToGetFileNameFromPath { path: PathBuf },
    /// This error is used when the following function returns None:
    /// to_str(self: &OsStr) -> Option<&str>
    FailedToConvertOsStrToStr { os_str: OsString },
    /// This error is used when the project is initialized but the project is already created.
    ProjectAlreadyExists { path: PathBuf },
    /// This error is used when the target directory already exists.
    DirectoryAlreadyExists { path: PathBuf },
    /// This error is used when creating a directory fails.
    /// create_dir_all(self: &Path) -> Result<(), std::io::Error>
    FailedToCreateDirectory { path: PathBuf, source: io::Error },
    /// This error is used when writing a file fails.
    /// write(self: &Path, content: &str) -> Result<(), std::io::Error>
    FailedToWriteFile { path: PathBuf, source: io::Error },
    /// This error is used when the project is not initialized.
    ProjectNotInitialized,
    /// This error is used when the serialization of manifest fails.
    /// to_string(self: &toml::Value) -> Result<String, toml::ser::Error>
    FailedToSerializeManifest { source: toml::ser::Error },
    /// This error is used when the manifest file is not found.
    ManifestNotFound { path: PathBuf },
    /// This error is used when reading a file fails.
    /// read_to_string(self: &Path) -> Result<String, std::io::Error>
    FailedToReadFile { path: PathBuf, source: io::Error },
    /// This error is used when the deserialization of manifest fails.
    /// from_str(self: &str) -> Result<toml::Value, toml::de::Error>
    FailedToDeserializeManifest { source: toml::de::Error },
    /// This error is used when reading file size fails.
    /// metadata(self: &Path) -> Result<std::fs::Metadata, std::io::Error>
    FailedToReadFileSize { path: PathBuf, source: io::Error },
    /// This error is used when removing a file fails.
    /// remove_file(self: &Path) -> Result<(), std::io::Error>
    FailedToRemoveFile { path: PathBuf, source: io::Error },
    /// This error is used when removing directories fails.
    /// remove_dir_all(self: &Path) -> Result<(), std::io::Error>
    FailedToRemoveDirectory { path: PathBuf, source: io::Error },
    /// This error is used when fetching a channel fails.
    /// get<T: IntoUrl>(url: T) -> Result<reqwest::Response, reqwest::Error>
    FailedToFetchChannel { url: String, source: reqwest::Error },
    /// This error is used when the HTTP response status is not success.
    /// status(self: &reqwest::Response) -> reqwest::StatusCode
    ReceivedHttpNonSuccessStatus {
        url: String,
        status: u16,
        reason: String,
    },
    /// This error is used when converting Response to bytes fails.
    /// bytes(self: &reqwest::Response) -> Result<bytes::Bytes, reqwest::Error>
    FailedToConvertResponseToBytes { url: String, source: reqwest::Error },
    /// This error is used when converting bytes to str fails.
    /// from_utf8(self: &[u8]) -> Result<&str, std::str::Utf8Error>
    FailedToConvertBytesToStr { source: std::str::Utf8Error },
    /// This error is used when parsing a channel fails.
    /// toml::from_str(self: &str) -> Result<Channel, toml::de::Error>
    FailedToDeserializeToChannel { source: toml::de::Error },
    /// This error is used when the channel name is invalid.
    InvalidChannelName { name: String },
    /// This error is used when the channel URL is invalid.
    InvalidChannelUrl { url: String },
    /// This error is used when the channel already exists.
    ChannelAlreadyExists { name: String },
    /// This error is used when the channel is not found.
    ChannelNotFound { name: String },
}

impl ProjectError {
    pub fn code(&self) -> String {
        match self {
            ProjectError::FailedToGetFileNameFromPath { .. } => String::from("E0001"),
            ProjectError::FailedToConvertOsStrToStr { .. } => String::from("E0002"),
            ProjectError::ProjectAlreadyExists { .. } => String::from("E0003"),
            ProjectError::DirectoryAlreadyExists { .. } => String::from("E0004"),
            ProjectError::FailedToCreateDirectory { .. } => String::from("E0005"),
            ProjectError::FailedToWriteFile { .. } => String::from("E0006"),
            ProjectError::ProjectNotInitialized => String::from("E0007"),
            ProjectError::FailedToSerializeManifest { .. } => String::from("E0008"),
            ProjectError::ManifestNotFound { .. } => String::from("E0009"),
            ProjectError::FailedToReadFile { .. } => String::from("E0010"),
            ProjectError::FailedToDeserializeManifest { .. } => String::from("E0011"),
            ProjectError::FailedToReadFileSize { .. } => String::from("E0012"),
            ProjectError::FailedToRemoveFile { .. } => String::from("E0013"),
            ProjectError::FailedToRemoveDirectory { .. } => String::from("E0014"),
            ProjectError::FailedToFetchChannel { .. } => String::from("E0015"),
            ProjectError::ReceivedHttpNonSuccessStatus { .. } => String::from("E0016"),
            ProjectError::FailedToConvertResponseToBytes { .. } => String::from("E0017"),
            ProjectError::FailedToConvertBytesToStr { .. } => String::from("E0018"),
            ProjectError::FailedToDeserializeToChannel { .. } => String::from("E0019"),
            ProjectError::InvalidChannelName { .. } => String::from("E0020"),
            ProjectError::InvalidChannelUrl { .. } => String::from("E0021"),
            ProjectError::ChannelAlreadyExists { .. } => String::from("E0022"),
            ProjectError::ChannelNotFound { .. } => String::from("E0023"),
        }
    }
}

impl From<ProjectError> for Error {
    fn from(err: ProjectError) -> Self {
        Error::ProjectError(err)
    }
}

pub trait ProjectResultExt<T> {
    fn map_project_err<L: Logger>(self, logger: &L) -> Result<T, Error>;
}

impl<T, E> ProjectResultExt<T> for Result<T, E>
where
    E: Into<ProjectError>,
{
    fn map_project_err<L: Logger>(self, logger: &L) -> Result<T, Error> {
        self.map_err(|err| {
            let err = Error::ProjectError(err.into());
            logger.error(&err);
            err
        })
    }
}

pub enum ReleaseError {}

impl ReleaseError {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("ER001"),
        }
    }
}

pub enum Warning {
    TransformingWarning(TransformingWarning),
    LoweringWarning(LoweringWarning),
    CodegenWarning(CodegenWarning),
    ProjectWarning(ProjectWarning),
    ReleaseWarning(ReleaseWarning),
}

impl Warning {
    pub fn code(&self) -> String {
        match self {
            Self::TransformingWarning(warn) => warn.code(),
            Self::LoweringWarning(warn) => warn.code(),
            Self::CodegenWarning(warn) => warn.code(),
            Self::ProjectWarning(warn) => warn.code(),
            Self::ReleaseWarning(warn) => warn.code(),
        }
    }
}

pub enum TransformingWarning {}

impl TransformingWarning {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("WT001"),
        }
    }
}

pub enum LoweringWarning {}

impl LoweringWarning {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("WL001"),
        }
    }
}

pub enum CodegenWarning {}

impl CodegenWarning {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("WC001"),
        }
    }
}

pub enum ProjectWarning {
    NoFilesRemovedDueToDryRun,
    NoChannelsFound,
}

impl ProjectWarning {
    pub fn code(&self) -> String {
        match self {
            ProjectWarning::NoFilesRemovedDueToDryRun => String::from("WP001"),
            ProjectWarning::NoChannelsFound => String::from("WP002"),
        }
    }
}

impl From<ProjectWarning> for Warning {
    fn from(warning: ProjectWarning) -> Self {
        Warning::ProjectWarning(warning)
    }
}

pub enum ReleaseWarning {}

impl ReleaseWarning {
    pub fn code(&self) -> String {
        match self {
            _ => String::from("WR001"),
        }
    }
}

pub enum Info {
    TransformingInfo(TransformingInfo),
    LoweringInfo(LoweringInfo),
    CodegenInfo(CodegenInfo),
    ProjectInfo(ProjectInfo),
    ReleaseInfo(ReleaseInfo),
}

pub enum TransformingInfo {}

pub enum LoweringInfo {}

pub enum CodegenInfo {}

pub enum ProjectInfo {
    CreatingProject {
        name: String,
    },
    FinishedCreatingProject {
        name: String,
    },
    CleaningProject {
        name: String,
    },
    FinishedCleaningProject {
        name: String,
        num_files: i32,
        file_size: u64,
    },
    AddingChannel {
        name: String,
    },
    FinishedAddingChannel {
        name: String,
    },
    FetchingChannel {
        name: String,
        url: String,
    },
    FinishedFetchingChannel {
        name: String,
    },
    RemovingChannel {
        name: String,
    },
    FinishedRemovingChannel {
        name: String,
    },
    UpdatingChannel {
        name: Option<String>,
    },
    FinishedUpdatingChannel {
        name: Option<String>,
    },
    ListingChannels,
    FinishedListingChannels,
    ChannelInfo {
        name: String,
        url: String,
        health: bool,
    },
}

impl From<ProjectInfo> for Info {
    fn from(info: ProjectInfo) -> Self {
        Info::ProjectInfo(info)
    }
}

pub enum ReleaseInfo {}

pub enum Debug {
    TransformingDebug(TransformingDebug),
    LoweringDebug(LoweringDebug),
    CodegenDebug(CodegenDebug),
    ProjectDebug(ProjectDebug),
    ReleaseDebug(ReleaseDebug),
}

pub enum TransformingDebug {}

pub enum LoweringDebug {}

pub enum CodegenDebug {}

pub enum ProjectDebug {
    DirectoryAlreadyExists { path: PathBuf },
    CreatingDirectory { path: PathBuf },
    WritingFile { path: PathBuf, content: String },
    WritingToml { content: String },
    ReadingFile { path: PathBuf, content: String },
    ReadingToml { content: String },
    RemovingFile { path: PathBuf },
    RemovingDirectory { path: PathBuf },
    ReceivedResponse { url: String },
    ConvertingResponseToBytes { url: String },
    ConvertingBytesToStr { url: String },
    DeserializingToChannel,
}

impl ProjectDebug {
    pub fn code(&self) -> String {
        match self {
            ProjectDebug::DirectoryAlreadyExists { .. } => String::from("DP0001"),
            ProjectDebug::CreatingDirectory { .. } => String::from("DP0002"),
            ProjectDebug::WritingFile { .. } => String::from("DP0003"),
            ProjectDebug::WritingToml { .. } => String::from("DP0004"),
            ProjectDebug::ReadingFile { .. } => String::from("DP0005"),
            ProjectDebug::ReadingToml { .. } => String::from("DP0006"),
            ProjectDebug::RemovingFile { .. } => String::from("DP0007"),
            ProjectDebug::RemovingDirectory { .. } => String::from("DP0008"),
            ProjectDebug::ReceivedResponse { .. } => String::from("DP0009"),
            ProjectDebug::ConvertingResponseToBytes { .. } => String::from("DP0010"),
            ProjectDebug::ConvertingBytesToStr { .. } => String::from("DP0011"),
            ProjectDebug::DeserializingToChannel { .. } => String::from("DP0012"),
        }
    }
}

impl From<ProjectDebug> for Debug {
    fn from(debug: ProjectDebug) -> Self {
        Debug::ProjectDebug(debug)
    }
}

pub enum ReleaseDebug {}
