use std::path::PathBuf;

use crate::error::AreiaResult;

use super::SuperHidden;


impl SuperHidden for PathBuf {
    /// ### MacOS
    ///
    /// After dotfile checking, also checks for file attributes if path points to an existing
    /// file or directory.
    /// Returns `true` if any component is hidden.
    ///
    fn is_super_hidden(&self) -> bool {
        todo!()
    }

    fn super_hide(&mut self) -> AreiaResult<PathBuf> {
        todo!()
    }

    fn super_unhide(&mut self) -> AreiaResult<PathBuf> {
        todo!()
    }
}
