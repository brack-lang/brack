use crate::errors::{Debug, Error, Info, Warning};
use std::path::PathBuf;

pub trait Logger {
    fn error(&self, error: &Error);
    fn warn(&self, warning: &Warning);
    fn info(&self, info: &Info);
    fn debug(&self, debug: &Debug);
    fn set_path(&mut self, path: PathBuf);
    fn get_path(&self) -> PathBuf;
}
