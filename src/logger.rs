use brack_common::{
    logger::Logger as BrackLogger,
    project_errors::{ProjectDebug, ProjectError, ProjectInfo, ProjectWarning},
};
use clap::ValueEnum;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label, Severity},
    files::SimpleFile,
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use colored::Colorize;
use file_size::fit_4;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Clone, ValueEnum, PartialOrd, PartialEq, Eq)]
pub enum CliLogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct Logger {
    pub raw_command: String,
    pub cli_log_level: CliLogLevel,
    pub path: Option<PathBuf>,
}

impl BrackLogger for Logger {
    fn error(&self, error: &ProjectError) {
        emit_error_diagnostic(self, error);
    }

    fn warn(&self, warning: &ProjectWarning) {
        if self.cli_log_level > CliLogLevel::Warn {
            return;
        }
        emit_warning_diagnostic(self, warning);
    }

    fn info(&self, info: &ProjectInfo) {
        if self.cli_log_level > CliLogLevel::Info {
            return;
        }
        emit_info_output(info);
    }

    fn debug(&self, debug: &ProjectDebug) {
        if self.cli_log_level > CliLogLevel::Debug {
            return;
        }
        emit_debug_output(debug);
    }

    /// Set the file path which is processing currently
    fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    /// Get the file path which is processing currently
    /// If the path is not set, it will panic
    fn get_path(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| panic!("file path is not set"))
    }
}

fn emit_error_diagnostic(logger: &Logger, error: &ProjectError) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let file = get_file_from_error(logger, error);
    let diagnostic_message = get_diagnostic_message_from_error(error);
    let diagnostic_labels = get_diagnostic_labels_from_error(logger, error);
    let diagnostic_notes = get_diagnostic_notes_from_error(error);
    let diagnostic = Diagnostic::new(Severity::Error)
        .with_message(diagnostic_message)
        .with_code(error.codespan_code())
        .with_labels(diagnostic_labels)
        .with_notes(diagnostic_notes);
    let mut writer_lock = writer.lock();
    if let Err(err) = emit(&mut writer_lock, &config, &file, &diagnostic) {
        eprintln!("Error emitting diagnostic: {}", err);
        exit(1);
    }
}

fn emit_warning_diagnostic(logger: &Logger, warning: &ProjectWarning) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let file = get_file_from_warning(logger, warning);
    let diagnostic_message = get_diagnostic_message_from_warning(warning);
    let diagnostic_labels = get_diagnostic_labels_from_warning(logger, warning);
    let diagnostic_notes = get_diagnostic_notes_from_warning(warning);
    let diagnostic = Diagnostic::new(Severity::Warning)
        .with_message(diagnostic_message)
        .with_code(warning.codespan_code())
        .with_labels(diagnostic_labels)
        .with_notes(diagnostic_notes);
    let mut writer_lock = writer.lock();
    if let Err(err) = emit(&mut writer_lock, &config, &file, &diagnostic) {
        eprintln!("Error emitting diagnostic: {}", err);
        exit(1);
    }
}

const TAG_WIDTH: usize = 12;

fn emit_info_output(info: &ProjectInfo) {
    let tag = get_info_tag(info);
    let heading = get_info_heading(info);
    let padding = " ".repeat(TAG_WIDTH - tag.len());
    let message = format!("{}{} {}", padding, tag.green().bold(), heading);
    println!("{}", message);
}

fn emit_debug_output(debug: &ProjectDebug) {
    let tag = "Debug";
    let heading = get_debug_heading(debug);
    let padding = " ".repeat(TAG_WIDTH - tag.len());
    let message = format!("{}{} {}", padding, tag.black().bold(), heading);
    println!("{}", message);
}

fn get_file_from_error(logger: &Logger, error: &ProjectError) -> SimpleFile<String, String> {
    let cli_command = SimpleFile::new(String::from("CLI Command"), logger.raw_command.clone());
    match error {
        ProjectError::FailedToGetFileNameFromPath { .. } => cli_command,
        ProjectError::FailedToConvertPathToStr { .. } => cli_command,
        ProjectError::ProjectAlreadyExists { .. } => cli_command,
        ProjectError::DirectoryAlreadyExists { .. } => cli_command,
        ProjectError::FailedToWriteFile { .. } => cli_command,
        ProjectError::FailedToCreateDirectory { .. } => cli_command,
        ProjectError::FailedToSerializeManifest { .. } => cli_command,
        ProjectError::FailedToDeserializeManifest { .. } => cli_command,
        ProjectError::UninitializedProject => cli_command,
        ProjectError::ChannelAlreadyExists { .. } => cli_command,
        ProjectError::FailedToFetchResource { .. } => cli_command,
        ProjectError::ReceivedHttpNonSuccessStatus { .. } => cli_command,
        ProjectError::FailedToConvertBytesToStr { .. } => cli_command,
        ProjectError::FailedToParseChannel { .. } => cli_command,
        ProjectError::ChannelNameCannotBeEmpty => cli_command,
        ProjectError::InvalidChannelName { .. } => cli_command,
        ProjectError::ChannelUrlCannotBeEmpty => cli_command,
        ProjectError::InvalidChannelUrl { .. } => cli_command,
        ProjectError::FailedToReadFile { .. } => cli_command,
        ProjectError::ManifestNotFound { .. } => cli_command,
        ProjectError::FailedToRemoveFile { .. } => cli_command,
        ProjectError::FailedToRemoveDir { .. } => cli_command,
        ProjectError::FailedToReadFileSize { .. } => cli_command,
        ProjectError::ChannelNotFound { .. } => cli_command,
        ProjectError::AngleNotOpened { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::AngleNotClosed { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::CurlyNotOpened { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::CurlyNotClosed { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::SquareNotOpened { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::SquareNotClosed { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::MismatchedBracket { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::ModuleNotFound { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::IdentifierNotFound { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::DotNotFound { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::CommaNotFound { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::UnexpectedDot { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::UnexpectedComma { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::InvalidBackslash { .. } => {
            let path = logger.get_path();
            let file = read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                exit(1);
            });
            SimpleFile::new(path.display().to_string(), file)
        }
        ProjectError::TransformError => cli_command,
        ProjectError::FailedToCreatePlugin => cli_command,
        ProjectError::ExpandError => cli_command,
        ProjectError::CodegenError => cli_command,
        ProjectError::DocumentSettingsNotFound => cli_command,
    }
}

fn get_diagnostic_message_from_error(error: &ProjectError) -> String {
    match error {
        ProjectError::FailedToGetFileNameFromPath { .. } => {
            String::from("failed to get file name from path")
        }
        ProjectError::FailedToConvertPathToStr { .. } => {
            String::from("failed to convert path to string")
        }
        ProjectError::ProjectAlreadyExists { .. } => String::from("project already exists"),
        ProjectError::DirectoryAlreadyExists { .. } => String::from("directory already exists"),
        ProjectError::FailedToWriteFile { .. } => String::from("failed to write file"),
        ProjectError::FailedToCreateDirectory { .. } => String::from("failed to create directory"),
        ProjectError::FailedToSerializeManifest { .. } => {
            String::from("failed to serialize manifest")
        }
        ProjectError::FailedToDeserializeManifest { .. } => {
            String::from("failed to deserialize manifest")
        }
        ProjectError::UninitializedProject => String::from("uninitialized project"),
        ProjectError::ChannelAlreadyExists { .. } => String::from("channel already exists"),
        ProjectError::FailedToFetchResource { .. } => String::from("failed to fetch resource"),
        ProjectError::ReceivedHttpNonSuccessStatus { .. } => {
            String::from("received non-success status")
        }
        ProjectError::FailedToConvertBytesToStr { .. } => {
            String::from("failed to convert bytes to string")
        }
        ProjectError::FailedToParseChannel { .. } => String::from("failed to parse channel"),
        ProjectError::ChannelNameCannotBeEmpty => String::from("channel name cannot be empty"),
        ProjectError::InvalidChannelName { .. } => String::from("invalid channel name"),
        ProjectError::ChannelUrlCannotBeEmpty => String::from("channel url cannot be empty"),
        ProjectError::InvalidChannelUrl { .. } => String::from("invalid channel url"),
        ProjectError::FailedToReadFile { .. } => String::from("failed to read file"),
        ProjectError::ManifestNotFound { .. } => String::from("manifest not found"),
        ProjectError::FailedToRemoveFile { .. } => String::from("failed to remove file"),
        ProjectError::FailedToRemoveDir { .. } => String::from("failed to remove directory"),
        ProjectError::FailedToReadFileSize { .. } => String::from("failed to read file size"),
        ProjectError::ChannelNotFound { .. } => String::from("channel not found"),
        ProjectError::AngleNotOpened { .. } => String::from("angle not opened"),
        ProjectError::AngleNotClosed { .. } => String::from("angle not closed"),
        ProjectError::CurlyNotOpened { .. } => String::from("curly not opened"),
        ProjectError::CurlyNotClosed { .. } => String::from("curly not closed"),
        ProjectError::SquareNotOpened { .. } => String::from("square not opened"),
        ProjectError::SquareNotClosed { .. } => String::from("square not closed"),
        ProjectError::MismatchedBracket { .. } => String::from("mismatched bracket"),
        ProjectError::ModuleNotFound { .. } => String::from("module not found"),
        ProjectError::IdentifierNotFound { .. } => String::from("identifier not found"),
        ProjectError::DotNotFound { .. } => String::from("dot not found"),
        ProjectError::CommaNotFound { .. } => String::from("comma not found"),
        ProjectError::UnexpectedDot { .. } => String::from("unexpected dot"),
        ProjectError::UnexpectedComma { .. } => String::from("unexpected comma"),
        ProjectError::InvalidBackslash { .. } => String::from("invalid backslash"),
        ProjectError::TransformError => String::from("transform error"),
        ProjectError::FailedToCreatePlugin => String::from("failed to create plugin"),
        ProjectError::ExpandError => String::from("expand error"),
        ProjectError::CodegenError => String::from("codegen error"),
        ProjectError::DocumentSettingsNotFound => String::from("document settings not found"),
    }
}

fn get_label_message_from_error(error: &ProjectError) -> String {
    match error {
        ProjectError::FailedToGetFileNameFromPath { path } => {
            let needle = path.display().to_string();
            if needle.ends_with("..") {
                String::from("path cannot terminate with `..`")
            } else if needle == "/" {
                String::from("path cannot be `/`")
            } else if needle == "" {
                String::from("path cannot be empty")
            } else {
                String::from("failed to get file name for unknown reason")
            }
        }
        ProjectError::FailedToConvertPathToStr { path } => {
            format!("path cannot be converted UTF-8: {}", path.display())
        }
        ProjectError::ProjectAlreadyExists { path } => {
            format!("project already exists: {}", path.display())
        }
        ProjectError::DirectoryAlreadyExists { path } => {
            format!("directory already exists: {}", path.display())
        }
        ProjectError::FailedToWriteFile { path, .. } => {
            format!("failed to write file: {}", path.display())
        }
        ProjectError::FailedToCreateDirectory { path, .. } => {
            format!("failed to create directory: {}", path.display())
        }
        ProjectError::FailedToSerializeManifest { .. } => String::from(""),
        ProjectError::FailedToDeserializeManifest { .. } => String::from(""),
        ProjectError::UninitializedProject => String::from(""),
        ProjectError::ChannelAlreadyExists { name } => {
            format!("channel already exists: {}", name)
        }
        ProjectError::FailedToFetchResource { url, .. } => {
            format!("failed to fetch resource: {}", url)
        }
        ProjectError::ReceivedHttpNonSuccessStatus {
            url,
            status,
            err_msg,
        } => {
            format!(
                "received non-success status: {} {}: {}",
                url, status, err_msg
            )
        }
        ProjectError::FailedToConvertBytesToStr { source } => {
            format!("failed to convert bytes to string: {}", source)
        }
        ProjectError::FailedToParseChannel { source } => {
            format!("failed to parse channel: {}", source)
        }
        ProjectError::ChannelNameCannotBeEmpty => String::from("channel name cannot be empty"),
        ProjectError::InvalidChannelName { name } => {
            format!("invalid channel name: {}", name)
        }
        ProjectError::ChannelUrlCannotBeEmpty => String::from("channel url cannot be empty"),
        ProjectError::InvalidChannelUrl { url } => {
            format!("invalid channel url: {}", url)
        }
        ProjectError::FailedToReadFile { path, .. } => {
            format!("failed to read file: {}", path.display())
        }
        ProjectError::ManifestNotFound { path } => {
            format!("manifest not found: {}", path.display())
        }
        ProjectError::FailedToRemoveFile { path, .. } => {
            format!("failed to remove file: {}", path.display())
        }
        ProjectError::FailedToRemoveDir { path, .. } => {
            format!("failed to remove directory: {}", path.display())
        }
        ProjectError::FailedToReadFileSize { path, .. } => {
            format!("failed to read file size: {}", path.display())
        }
        ProjectError::ChannelNotFound { name } => {
            format!("channel not found: {}", name)
        }
        ProjectError::AngleNotOpened { .. } => String::from("angle not opened"),
        ProjectError::AngleNotClosed { .. } => String::from("angle not closed"),
        ProjectError::CurlyNotOpened { .. } => String::from("curly not opened"),
        ProjectError::CurlyNotClosed { .. } => String::from("curly not closed"),
        ProjectError::SquareNotOpened { .. } => String::from("square not opened"),
        ProjectError::SquareNotClosed { .. } => String::from("square not closed"),
        ProjectError::MismatchedBracket { .. } => String::from("mismatched bracket"),
        ProjectError::ModuleNotFound { .. } => String::from("module not found"),
        ProjectError::IdentifierNotFound { .. } => String::from("identifier not found"),
        ProjectError::DotNotFound { .. } => String::from("dot not found"),
        ProjectError::CommaNotFound { .. } => String::from("comma not found"),
        ProjectError::UnexpectedDot { .. } => String::from("unexpected dot"),
        ProjectError::UnexpectedComma { .. } => String::from("unexpected comma"),
        ProjectError::InvalidBackslash { .. } => String::from("invalid backslash"),
        ProjectError::TransformError => String::from("transform error"),
        ProjectError::FailedToCreatePlugin => String::from("failed to create plugin"),
        ProjectError::ExpandError => String::from("expand error"),
        ProjectError::CodegenError => String::from("codegen error"),
        ProjectError::DocumentSettingsNotFound => String::from("document settings not found"),
    }
}

fn search_for_needle_in_heystack(needle: &str, heystack: &str) -> (usize, usize) {
    let mut start = 0;
    let mut end = 0;
    for word in heystack.split_whitespace() {
        if word == needle {
            end = start + word.len();
            break;
        }
        start += word.len() + 1;
    }
    if end == 0 {
        end = start + needle.len();
    }
    (start, end)
}

fn common_label<P: AsRef<Path>>(logger: &Logger, error: &ProjectError, path: &P) -> Label<()> {
    let (start, end) =
        search_for_needle_in_heystack(&path.as_ref().display().to_string(), &logger.raw_command);
    Label::primary((), start..end).with_message(get_label_message_from_error(error))
}

fn get_diagnostic_labels_from_error(logger: &Logger, error: &ProjectError) -> Vec<Label<()>> {
    match error {
        ProjectError::FailedToGetFileNameFromPath { path } => {
            vec![common_label(logger, error, path)]
        }
        ProjectError::FailedToConvertPathToStr { path } => vec![common_label(logger, error, path)],
        ProjectError::ProjectAlreadyExists { path } => vec![common_label(logger, error, path)],
        ProjectError::DirectoryAlreadyExists { path } => vec![common_label(logger, error, path)],
        ProjectError::FailedToWriteFile { path, .. } => vec![common_label(logger, error, path)],
        ProjectError::FailedToCreateDirectory { path, .. } => {
            vec![common_label(logger, error, path)]
        }
        ProjectError::FailedToSerializeManifest { .. } => vec![],
        ProjectError::FailedToDeserializeManifest { .. } => vec![],
        ProjectError::UninitializedProject => vec![],
        ProjectError::ChannelAlreadyExists { name } => vec![common_label(logger, error, name)],
        ProjectError::FailedToFetchResource { url, .. } => {
            vec![common_label(logger, error, url)]
        }
        ProjectError::ReceivedHttpNonSuccessStatus { url, .. } => {
            vec![common_label(logger, error, url)]
        }
        ProjectError::FailedToConvertBytesToStr { .. } => vec![],
        ProjectError::FailedToParseChannel { .. } => vec![],
        ProjectError::ChannelNameCannotBeEmpty => {
            vec![common_label(logger, error, &logger.raw_command)]
        }
        ProjectError::InvalidChannelName { name } => vec![common_label(logger, error, name)],
        ProjectError::ChannelUrlCannotBeEmpty => {
            vec![common_label(logger, error, &logger.raw_command)]
        }
        ProjectError::InvalidChannelUrl { url } => vec![common_label(logger, error, url)],
        ProjectError::FailedToReadFile { path, .. } => vec![common_label(logger, error, path)],
        ProjectError::ManifestNotFound { path } => vec![common_label(logger, error, path)],
        ProjectError::FailedToRemoveFile { path, .. } => vec![common_label(logger, error, path)],
        ProjectError::FailedToRemoveDir { path, .. } => vec![common_label(logger, error, path)],
        ProjectError::FailedToReadFileSize { path, .. } => vec![common_label(logger, error, path)],
        ProjectError::ChannelNotFound { name } => {
            vec![common_label(logger, error, name)]
        }
        ProjectError::AngleNotOpened { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("angle not opened")]
        }
        ProjectError::AngleNotClosed { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("angle not closed")]
        }
        ProjectError::CurlyNotOpened { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("curly not opened")]
        }
        ProjectError::CurlyNotClosed { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("curly not closed")]
        }
        ProjectError::SquareNotOpened { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("square not opened")]
        }
        ProjectError::SquareNotClosed { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("square not closed")]
        }
        ProjectError::MismatchedBracket { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("mismatched bracket")]
        }
        ProjectError::ModuleNotFound { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("module not found")]
        }
        ProjectError::IdentifierNotFound { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("identifier not found")]
        }
        ProjectError::DotNotFound { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("dot not found")]
        }
        ProjectError::CommaNotFound { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("comma not found")]
        }
        ProjectError::UnexpectedDot { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("unexpected dot")]
        }
        ProjectError::UnexpectedComma { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("unexpected comma")]
        }
        ProjectError::InvalidBackslash { location } => {
            let path = logger.get_path();
            let file = match read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    exit(1);
                }
            };
            vec![Label::primary(
                (),
                location.start.to_usize(&file)..location.end.to_usize(&file),
            )
            .with_message("invalid backslash")]
        }
        ProjectError::TransformError => vec![],
        ProjectError::FailedToCreatePlugin => vec![],
        ProjectError::ExpandError => vec![],
        ProjectError::CodegenError => vec![],
        ProjectError::DocumentSettingsNotFound => vec![],
    }
}

fn get_diagnostic_notes_from_error(error: &ProjectError) -> Vec<String> {
    match error {
        ProjectError::FailedToGetFileNameFromPath { .. } => {
            vec![format!(
                "{}: Check the file path unless it terminates with `..` or is `/`",
                "hint".bold(),
            )]
        }
        ProjectError::FailedToConvertPathToStr { .. } => {
            vec![format!(
                "{}: Check the file path unless it is not valid UTF-8",
                "hint".bold(),
            ), format!(
                "{}: The file path in Unix contains u8 bytes sequence, so it sometimes cannot be converted to UTF-8",
                "note".bold(),
            ), format!(
                "{}: The file path in Windows contains UTF-16 sequence, so it cannot be converted to UTF-8",
                "note".bold(),
            )]
        }
        ProjectError::ProjectAlreadyExists { path } => {
            vec![
                format!(
                    "{}: Check the file path unless it already exists",
                    "hint".bold(),
                ),
                format!(
                    "{}: You can run `rm -rf {}` to remove the project",
                    "note".bold(),
                    path.display(),
                ),
            ]
        }
        ProjectError::DirectoryAlreadyExists { path } => {
            vec![
                format!(
                    "{}: Check the directory path unless it already exists",
                    "hint".bold(),
                ),
                format!(
                    "{}: You can run `rm -rf {}` to remove the directory",
                    "note".bold(),
                    path.display(),
                ),
            ]
        }
        ProjectError::FailedToWriteFile { path, source } => {
            vec![
                format!(
                    "{}: The file path is not writable: {}",
                    "hint".bold(),
                    source,
                ),
                format!(
                    "{}: You can run `chmod +w {}` to grant the write permission",
                    "note".bold(),
                    path.display(),
                ),
            ]
        }
        ProjectError::FailedToCreateDirectory { path, source } => {
            vec![
                format!(
                    "{}: The directory path is not writable: {}",
                    "hint".bold(),
                    source,
                ),
                format!(
                    "{}: You can run `chmod +w {}` to grant the write permission",
                    "note".bold(),
                    path.display(),
                ),
            ]
        }
        ProjectError::FailedToSerializeManifest { source } => {
            vec![format!(
                "{}: Check the manifest file unless it is not valid TOML: {}",
                "hint".bold(),
                source,
            )]
        }
        ProjectError::FailedToDeserializeManifest { source } => {
            vec![format!(
                "{}: Check the manifest file unless it is not valid TOML: {}",
                "hint".bold(),
                source,
            )]
        }
        ProjectError::UninitializedProject => {
            vec![
                format!(
                    "{}: Check the Brack.toml exists in the current directory",
                    "hint".bold(),
                ),
                format!(
                    "{}: You can run `brack create` to initialize the project",
                    "note".bold(),
                ),
            ]
        }
        ProjectError::ChannelAlreadyExists { name } => {
            vec![
                format!(
                    "{}: Check the channel name unless it already exists",
                    "hint".bold(),
                ),
                format!(
                    "{}: You can run `brack channel remove {}` to remove the channel",
                    "note".bold(),
                    name,
                ),
                format!(
                    "{}: You can run `brack channel add --force {}` to force add the channel",
                    "note".bold(),
                    name,
                ),
            ]
        }
        ProjectError::FailedToFetchResource { url, source } => {
            vec![
                format!(
                    "{}: Check the URL unless it is not reachable: {}",
                    "hint".bold(),
                    source,
                ),
                format!(
                    "{}: You can run `curl -v {}` to check the URL",
                    "note".bold(),
                    url,
                ),
            ]
        }
        ProjectError::ReceivedHttpNonSuccessStatus {
            url,
            status,
            err_msg,
        } => {
            vec![
                format!(
                    "{}: Check the URL unless it is not reachable: {}",
                    "hint".bold(),
                    err_msg,
                ),
                format!(
                    "{}: You can run `curl -v {}` to check the URL",
                    "note".bold(),
                    url,
                ),
                format!("{}: The status code is {}", "note".bold(), status,),
            ]
        }
        ProjectError::FailedToConvertBytesToStr { source } => {
            vec![format!(
                "{}: Check the bytes sequence unless it is not valid UTF-8: {}",
                "hint".bold(),
                source,
            )]
        }
        ProjectError::FailedToParseChannel { source } => {
            vec![format!(
                "{}: Check the channel file unless it is not valid TOML: {}",
                "hint".bold(),
                source,
            )]
        }
        ProjectError::ChannelNameCannotBeEmpty => {
            vec![format!("{}: Channel name cannot be empty", "hint".bold(),)]
        }
        ProjectError::InvalidChannelName { .. } => {
            vec![format!(
                "{}: Channel name can only contain alphanumeric characters and underscores",
                "hint".bold(),
            )]
        }
        ProjectError::ChannelUrlCannotBeEmpty => {
            vec![format!("{}: Channel URL cannot be empty", "hint".bold(),)]
        }
        ProjectError::InvalidChannelUrl { .. } => {
            vec![format!(
                "{}: Channel URL must start with `http://` or `https://`",
                "hint".bold(),
            )]
        }
        ProjectError::FailedToReadFile { path, source } => {
            vec![format!(
                "{}: Check the file path {} unless it is not readable: {}",
                "hint".bold(),
                path.display(),
                source,
            )]
        }
        ProjectError::ManifestNotFound { path } => {
            vec![format!(
                "{}: Check the file path unless it is not readable: {}",
                "hint".bold(),
                path.display(),
            )]
        }
        ProjectError::FailedToRemoveFile { path, source } => {
            vec![format!(
                "{}: Check the file path {} unless it is not removable: {}",
                "hint".bold(),
                path.display(),
                source,
            )]
        }
        ProjectError::FailedToRemoveDir { path, source } => {
            vec![format!(
                "{}: Check the directory path {} unless it is not removable: {}",
                "hint".bold(),
                path.display(),
                source,
            )]
        }
        ProjectError::FailedToReadFileSize { path, source } => {
            vec![format!(
                "{}: Check the file path {} unless it is not readable: {}",
                "hint".bold(),
                path.display(),
                source,
            )]
        }
        ProjectError::ChannelNotFound { name } => {
            vec![
                format!(
                    "{}: Check the channel name {} unless it is not found",
                    "hint".bold(),
                    name,
                ),
                format!(
                    "{}: You can run `brack channel list` to list all channels",
                    "note".bold(),
                ),
            ]
        }
        ProjectError::AngleNotOpened { .. } => vec![],
        ProjectError::AngleNotClosed { .. } => vec![],
        ProjectError::CurlyNotOpened { .. } => vec![],
        ProjectError::CurlyNotClosed { .. } => vec![],
        ProjectError::SquareNotOpened { .. } => vec![],
        ProjectError::SquareNotClosed { .. } => vec![],
        ProjectError::MismatchedBracket { .. } => vec![],
        ProjectError::ModuleNotFound { .. } => vec![],
        ProjectError::IdentifierNotFound { .. } => vec![],
        ProjectError::DotNotFound { .. } => vec![],
        ProjectError::CommaNotFound { .. } => vec![],
        ProjectError::UnexpectedDot { .. } => vec![],
        ProjectError::UnexpectedComma { .. } => vec![],
        ProjectError::InvalidBackslash { .. } => vec![],
        ProjectError::TransformError => vec![],
        ProjectError::FailedToCreatePlugin => vec![],
        ProjectError::ExpandError => vec![],
        ProjectError::CodegenError => vec![],
        ProjectError::DocumentSettingsNotFound => vec![],
    }
}

fn get_file_from_warning(logger: &Logger, warning: &ProjectWarning) -> SimpleFile<String, String> {
    let cli_command = SimpleFile::new(String::from("CLI Command"), logger.raw_command.clone());
    match warning {
        ProjectWarning::NoFilesRemovedDueToDryRun => cli_command,
        ProjectWarning::NoChannelsFound => cli_command,
    }
}

fn get_diagnostic_message_from_warning(warning: &ProjectWarning) -> String {
    match warning {
        ProjectWarning::NoFilesRemovedDueToDryRun => {
            String::from("no files removed due to dry run")
        }
        ProjectWarning::NoChannelsFound => String::from("no channels found"),
    }
}

fn get_diagnostic_labels_from_warning(
    _logger: &Logger,
    warning: &ProjectWarning,
) -> Vec<Label<()>> {
    match warning {
        ProjectWarning::NoFilesRemovedDueToDryRun => vec![],
        ProjectWarning::NoChannelsFound => vec![],
    }
}

fn get_diagnostic_notes_from_warning(warning: &ProjectWarning) -> Vec<String> {
    match warning {
        ProjectWarning::NoFilesRemovedDueToDryRun => vec![],
        ProjectWarning::NoChannelsFound => vec![],
    }
}

fn get_info_tag(info: &ProjectInfo) -> String {
    match info {
        ProjectInfo::CreatingProject { .. } => String::from("Creating"),
        ProjectInfo::FinishedCreatingProject { .. } => String::from("Finished"),
        ProjectInfo::AddingChannel { .. } => String::from("Adding"),
        ProjectInfo::FinishedAddingChannel { .. } => String::from("Finished"),
        ProjectInfo::CleaningProject => String::from("Cleaning"),
        ProjectInfo::FinishedCleaningProject { .. } => String::from("Finished"),
        ProjectInfo::ListingChannels => String::from("Listing"),
        ProjectInfo::FinishedListingChannels => String::from("Finished"),
        ProjectInfo::ChannelInfo { .. } => String::from("Channel"),
        ProjectInfo::RemovingChannel { .. } => String::from("Removing"),
        ProjectInfo::FinishedRemovingChannel { .. } => String::from("Finished"),
        ProjectInfo::UpdatingChannel { .. } => String::from("Updating"),
        ProjectInfo::FinishedUpdatingChannel { .. } => String::from("Finished"),
        ProjectInfo::BuildingProject { .. } => String::from("Building"),
        ProjectInfo::FinishedBuildingProject { .. } => String::from("Finished"),
    }
}

fn get_info_heading(info: &ProjectInfo) -> String {
    match info {
        ProjectInfo::CreatingProject { name } => format!("project `{}`", name),
        ProjectInfo::FinishedCreatingProject { name } => format!("creating project `{}`", name),
        ProjectInfo::AddingChannel { name } => format!("channel `{}`", name),
        ProjectInfo::FinishedAddingChannel { name } => format!("adding channel `{}`", name),
        ProjectInfo::CleaningProject => String::from("cleaning project"),
        ProjectInfo::FinishedCleaningProject {
            num_files,
            file_size,
        } => {
            format!(
                "cleaning project {} files, {} bytes",
                num_files,
                fit_4(*file_size)
            )
        }
        ProjectInfo::ListingChannels => String::from("channels"),
        ProjectInfo::FinishedListingChannels => String::from("listing channels"),
        ProjectInfo::ChannelInfo { name, url, health } => {
            let health = if *health { "healthy" } else { "unhealthy" };
            format!("channel `{}`: {} ({})", name, url, health)
        }
        ProjectInfo::RemovingChannel { name } => format!("channel `{}`", name),
        ProjectInfo::FinishedRemovingChannel { name } => format!("removing channel `{}`", name),
        ProjectInfo::UpdatingChannel { name } => {
            let name = match name {
                Some(name) => name,
                _ => "all",
            };
            format!("updating channel `{}`", name)
        }
        ProjectInfo::FinishedUpdatingChannel { name } => {
            let name = match name {
                Some(name) => name,
                _ => "all",
            };
            format!("updating channel `{}`", name)
        }
        ProjectInfo::BuildingProject { name } => format!("project `{}`", name),
        ProjectInfo::FinishedBuildingProject { name } => format!("building project `{}`", name),
    }
}

fn get_debug_heading(debug: &ProjectDebug) -> String {
    match debug {
        ProjectDebug::WritingFile { path, content } => {
            let path = path.display().to_string();
            let content = content
                .lines()
                .map(|line| format!("{} {}", " ".repeat(TAG_WIDTH), line))
                .collect::<Vec<_>>()
                .join("\n")
                .black();
            format!("writing file `{}`\n{}", path, content)
        }
        ProjectDebug::CreatingDirectory { path } => {
            let path = path.display().to_string();
            format!("creating directory `{}`", path)
        }
        ProjectDebug::RemoveFile { path } => {
            let path = path.display().to_string();
            format!("removing file `{}`", path)
        }
        ProjectDebug::RemoveDir { path } => {
            let path = path.display().to_string();
            format!("removing directory `{}`", path)
        }
        ProjectDebug::BuildingFile { path, file_name } => {
            let path = path.display().to_string();
            format!("building file `{}` in `{}`", file_name, path)
        }
    }
}
