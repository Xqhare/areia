mod utils;
mod dirs;
use std::path::PathBuf;

pub use dirs::{BaseDirs, UserDirs};
use error::AreiaResult;
use utils::is_component_hidden;

pub mod error;

pub trait Hidden {
    fn is_hidden(&self) -> bool;
    fn hide(&mut self) -> AreiaResult<()>;
    fn unhide(&mut self) -> AreiaResult<()>;
    fn is_super_hidden(&self) -> bool;
    fn super_hide(&mut self) -> AreiaResult<()>;
    fn super_unhide(&mut self) -> AreiaResult<()>;
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

    fn is_super_hidden(&self) -> bool {
        todo!()
    }

    fn super_hide(&mut self) -> AreiaResult<()> {
        todo!()
    }

    fn super_unhide(&mut self) -> AreiaResult<()> {
        todo!()
    }
}
