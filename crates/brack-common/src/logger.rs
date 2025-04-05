use crate::project_errors::{ProjectDebug, ProjectError, ProjectInfo, ProjectWarning};

pub trait Logger {
    fn error(&self, error: &ProjectError);
    fn warn(&self, warning: &ProjectWarning);
    fn info(&self, info: &ProjectInfo);
    fn debug(&self, debug: &ProjectDebug);
}
