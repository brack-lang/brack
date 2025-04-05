use std::io;
use std::path::PathBuf;
use std::str;

use crate::location::Location;

#[derive(Debug)]
pub enum ProjectError {
    /// If the path terminates in `..` or is `/`.
    FailedToGetFileNameFromPath {
        path: PathBuf,
    },
    /// If the path is not valid UTF-8.
    FailedToConvertPathToStr {
        path: PathBuf,
    },
    /// If the project already exists.
    ProjectAlreadyExists {
        path: PathBuf,
    },
    /// If the directory already exists.
    DirectoryAlreadyExists {
        path: PathBuf,
    },
    /// If `std::io::write` fails.
    FailedToWriteFile {
        path: PathBuf,
        source: io::Error,
    },
    /// If `std::fs::create_dir` fails.
    FailedToCreateDirectory {
        path: PathBuf,
        source: io::Error,
    },
    /// If `toml::to_string` fails.
    FailedToSerializeManifest {
        source: toml::ser::Error,
    },
    FailedToDeserializeManifest {
        source: toml::de::Error,
    },
    UninitializedProject,
    ChannelAlreadyExists {
        name: String,
    },
    FailedToFetchResource {
        url: String,
        source: reqwest::Error,
    },
    ReceivedHttpNonSuccessStatus {
        url: String,
        status: reqwest::StatusCode,
        err_msg: String,
    },
    /// If `std::str::from_utf8` fails.
    FailedToConvertBytesToStr {
        source: str::Utf8Error,
    },
    FailedToParseChannel {
        source: toml::de::Error,
    },
    ChannelNameCannotBeEmpty,
    InvalidChannelName {
        name: String,
    },
    ChannelUrlCannotBeEmpty,
    InvalidChannelUrl {
        url: String,
    },
    FailedToReadFile {
        path: PathBuf,
        source: io::Error,
    },
    ManifestNotFound {
        path: PathBuf,
    },
    FailedToRemoveFile {
        path: PathBuf,
        source: io::Error,
    },
    FailedToRemoveDir {
        path: PathBuf,
        source: io::Error,
    },
    FailedToReadFileSize {
        path: PathBuf,
        source: io::Error,
    },
    ChannelNotFound {
        name: String,
    },
    AngleNotOpened {
        location: Location,
    },
    AngleNotClosed {
        location: Location,
    },
    CurlyNotOpened {
        location: Location,
    },
    CurlyNotClosed {
        location: Location,
    },
    SquareNotOpened {
        location: Location,
    },
    SquareNotClosed {
        location: Location,
    },
    MismatchedBracket {
        location: Location,
    },
    ModuleNotFound {
        location: Location,
    },
    IdentifierNotFound {
        location: Location,
    },
    DotNotFound {
        location: Location,
    },
    CommaNotFound {
        location: Location,
    },
    UnexpectedDot {
        location: Location,
    },
    UnexpectedComma {
        location: Location,
    },
    InvalidBackslash {
        location: Location,
    },
}

impl ProjectError {
    pub fn exit_code(&self) -> i32 {
        match self {
            ProjectError::FailedToGetFileNameFromPath { .. } => 1,
            ProjectError::FailedToConvertPathToStr { .. } => 2,
            ProjectError::ProjectAlreadyExists { .. } => 3,
            ProjectError::DirectoryAlreadyExists { .. } => 4,
            ProjectError::FailedToWriteFile { .. } => 5,
            ProjectError::FailedToCreateDirectory { .. } => 6,
            ProjectError::FailedToSerializeManifest { .. } => 7,
            ProjectError::FailedToDeserializeManifest { .. } => 8,
            ProjectError::UninitializedProject => 9,
            ProjectError::ChannelAlreadyExists { .. } => 10,
            ProjectError::FailedToFetchResource { .. } => 11,
            ProjectError::ReceivedHttpNonSuccessStatus { .. } => 12,
            ProjectError::FailedToConvertBytesToStr { .. } => 13,
            ProjectError::FailedToParseChannel { .. } => 14,
            ProjectError::ChannelNameCannotBeEmpty => 15,
            ProjectError::InvalidChannelName { .. } => 16,
            ProjectError::ChannelUrlCannotBeEmpty => 17,
            ProjectError::InvalidChannelUrl { .. } => 18,
            ProjectError::FailedToReadFile { .. } => 19,
            ProjectError::ManifestNotFound { .. } => 20,
            ProjectError::FailedToRemoveFile { .. } => 21,
            ProjectError::FailedToRemoveDir { .. } => 22,
            ProjectError::FailedToReadFileSize { .. } => 23,
            ProjectError::ChannelNotFound { .. } => 24,
            ProjectError::AngleNotOpened { .. } => 25,
            ProjectError::AngleNotClosed { .. } => 26,
            ProjectError::CurlyNotOpened { .. } => 27,
            ProjectError::CurlyNotClosed { .. } => 28,
            ProjectError::SquareNotOpened { .. } => 29,
            ProjectError::SquareNotClosed { .. } => 30,
            ProjectError::MismatchedBracket { .. } => 31,
            ProjectError::ModuleNotFound { .. } => 32,
            ProjectError::IdentifierNotFound { .. } => 33,
            ProjectError::DotNotFound { .. } => 34,
            ProjectError::CommaNotFound { .. } => 35,
            ProjectError::UnexpectedDot { .. } => 36,
            ProjectError::UnexpectedComma { .. } => 37,
            ProjectError::InvalidBackslash { .. } => 38,
        }
    }

    pub fn codespan_code(&self) -> String {
        match self {
            ProjectError::FailedToGetFileNameFromPath { .. } => String::from("E0001"),
            ProjectError::FailedToConvertPathToStr { .. } => String::from("E0002"),
            ProjectError::ProjectAlreadyExists { .. } => String::from("E0003"),
            ProjectError::DirectoryAlreadyExists { .. } => String::from("E0004"),
            ProjectError::FailedToWriteFile { .. } => String::from("E0005"),
            ProjectError::FailedToCreateDirectory { .. } => String::from("E0006"),
            ProjectError::FailedToSerializeManifest { .. } => String::from("E0007"),
            ProjectError::FailedToDeserializeManifest { .. } => String::from("E0008"),
            ProjectError::UninitializedProject => String::from("E0009"),
            ProjectError::ChannelAlreadyExists { .. } => String::from("E0010"),
            ProjectError::FailedToFetchResource { .. } => String::from("E0011"),
            ProjectError::ReceivedHttpNonSuccessStatus { .. } => String::from("E0012"),
            ProjectError::FailedToConvertBytesToStr { .. } => String::from("E0013"),
            ProjectError::FailedToParseChannel { .. } => String::from("E0014"),
            ProjectError::ChannelNameCannotBeEmpty => String::from("E0015"),
            ProjectError::InvalidChannelName { .. } => String::from("E0016"),
            ProjectError::ChannelUrlCannotBeEmpty => String::from("E0017"),
            ProjectError::InvalidChannelUrl { .. } => String::from("E0018"),
            ProjectError::FailedToReadFile { .. } => String::from("E0019"),
            ProjectError::ManifestNotFound { .. } => String::from("E0020"),
            ProjectError::FailedToRemoveFile { .. } => String::from("E0021"),
            ProjectError::FailedToRemoveDir { .. } => String::from("E0022"),
            ProjectError::FailedToReadFileSize { .. } => String::from("E0023"),
            ProjectError::ChannelNotFound { .. } => String::from("E0024"),
            ProjectError::AngleNotOpened { .. } => String::from("E0025"),
            ProjectError::AngleNotClosed { .. } => String::from("E0026"),
            ProjectError::CurlyNotOpened { .. } => String::from("E0027"),
            ProjectError::CurlyNotClosed { .. } => String::from("E0028"),
            ProjectError::SquareNotOpened { .. } => String::from("E0029"),
            ProjectError::SquareNotClosed { .. } => String::from("E0030"),
            ProjectError::MismatchedBracket { .. } => String::from("E0031"),
            ProjectError::ModuleNotFound { .. } => String::from("E0032"),
            ProjectError::IdentifierNotFound { .. } => String::from("E0033"),
            ProjectError::DotNotFound { .. } => String::from("E0034"),
            ProjectError::CommaNotFound { .. } => String::from("E0035"),
            ProjectError::UnexpectedDot { .. } => String::from("E0036"),
            ProjectError::UnexpectedComma { .. } => String::from("E0037"),
            ProjectError::InvalidBackslash { .. } => String::from("E0038"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProjectWarning {
    NoFilesRemovedDueToDryRun,
    NoChannelsFound,
}

impl ProjectWarning {
    pub fn codespan_code(&self) -> String {
        match self {
            ProjectWarning::NoFilesRemovedDueToDryRun => String::from("W0001"),
            ProjectWarning::NoChannelsFound => String::from("W0002"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProjectInfo {
    CreatingProject {
        name: String,
    },
    FinishedCreatingProject {
        name: String,
    },
    CleaningProject,
    FinishedCleaningProject {
        num_files: u64,
        file_size: u64,
    },
    AddingChannel {
        name: String,
    },
    FinishedAddingChannel {
        name: String,
    },
    ListingChannels,
    FinishedListingChannels,
    ChannelInfo {
        name: String,
        url: String,
        health: bool,
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
    BuildingProject {
        name: String,
    },
    FinishedBuildingProject {
        name: String,
    },
}

#[derive(Debug)]
pub enum ProjectDebug {
    WritingFile { path: PathBuf, content: String },
    CreatingDirectory { path: PathBuf },
    RemoveFile { path: PathBuf },
    RemoveDir { path: PathBuf },
}
