use crate::project_errors::{ProjectDebug, ProjectError, ProjectInfo, ProjectWarning};
use std::path::PathBuf;

pub trait Logger {
    fn error(&self, error: &ProjectError);
    fn warn(&self, warning: &ProjectWarning);
    fn info(&self, info: &ProjectInfo);
    fn debug(&self, debug: &ProjectDebug);
    fn set_path(&mut self, path: PathBuf);
    fn get_path(&self) -> PathBuf;
}
