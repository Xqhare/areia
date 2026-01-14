mod ffi;
use crate::error::AreiaResult;
use std::{ffi::OsStr, path::PathBuf};

pub mod factory;

#[cfg(any(target_os = "linux", target_os = "macos", doc))]
mod unix;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use unix as os;

#[cfg(any(target_os = "windows", doc))]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

pub fn get_home() -> AreiaResult<PathBuf> {
    os::get_home()
}

pub trait Hidden {
    fn is_hidden(&self) -> bool;
    fn hide(&mut self) -> AreiaResult<()>;
    fn unhide(&mut self) -> AreiaResult<()>;
}

// Would it make more sense to implement this for `File` instead?
impl Hidden for PathBuf {
    /// Returns true if the path is hidden
    ///
    /// Every part of the path is checked - Returns true if any part is hidden
    /// Does not check if a path exists
    fn is_hidden(&self) -> bool {
        self.components().any(|c| is_component_hidden(c.as_os_str()))
    }

    /// Hides the path
    ///
    /// Checks if any component of the path is already hidden.
    /// If not, the last directory contained in the path is hidden.
    ///
    /// Returns an error if the path does not exist
    fn hide(&mut self) -> AreiaResult<()> {
        todo!()
    }

    fn unhide(&mut self) -> AreiaResult<()> {
        todo!()
    }
}

fn is_component_hidden(path: &OsStr) -> bool {
    todo!()
}
