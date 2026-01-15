mod normal_hide;
mod super_hide;
use std::path::PathBuf;

use crate::error::AreiaResult;

pub trait Hidden {
    fn is_hidden(&self) -> bool;
    fn hide(&mut self) -> AreiaResult<PathBuf>;
    fn unhide(&mut self) -> AreiaResult<PathBuf>;
}

pub trait SuperHidden {
    fn is_super_hidden(&self) -> bool;
    fn super_hide(&mut self) -> AreiaResult<PathBuf>;
    fn super_unhide(&mut self) -> AreiaResult<PathBuf>;
}
