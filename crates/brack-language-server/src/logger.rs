use std::path::PathBuf;

use brack_common::errors::{Debug, Error, Info, Warning};

pub struct Logger {}

impl brack_common::logger::Logger for Logger {
    fn error(&self, _error: &Error) {}

    fn warn(&self, _warning: &Warning) {}

    fn info(&self, _info: &Info) {}

    fn debug(&self, _debug: &Debug) {}

    fn set_path(&mut self, _path: PathBuf) {}

    fn get_path(&self) -> PathBuf {
        PathBuf::new()
    }
}
