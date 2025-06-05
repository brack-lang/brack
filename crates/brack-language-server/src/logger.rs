use brack_common::project_errors::{ProjectDebug, ProjectError, ProjectInfo, ProjectWarning};
use std::path::PathBuf;

pub struct Logger {}

impl brack_common::logger::Logger for Logger {
    fn error(&self, _error: &ProjectError) {}

    fn warn(&self, _warning: &ProjectWarning) {}

    fn info(&self, _info: &ProjectInfo) {}

    fn debug(&self, _debug: &ProjectDebug) {}

    fn set_path(&mut self, _path: PathBuf) {}

    fn get_path(&self) -> PathBuf {
        PathBuf::new()
    }
}
