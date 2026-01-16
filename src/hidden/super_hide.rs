use std::path::PathBuf;

use crate::error::{AreiaError, AreiaResult};

use super::SuperHidden;


impl SuperHidden for PathBuf {
    /// Checks if any component of the path is `super hidden`.\
    /// The path must point to an existing file or directory.\
    /// This concept only applies to MacOS and Windows.
    ///
    /// `super hidden` is a file or directory that is hidden, as achieved with using `.hide()` provided by `areia`, and marked with a platform specific attribute or flag.
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// ### Linux
    ///
    /// Always returns `AreiaError::SuperHidingNotSupported`.
    ///
    /// ### MacOS
    ///
    /// Returns `true` if any component of the path is hidden (`.` prefix) and has the `hidden` flag set.
    ///
    /// ## Windows
    ///
    /// Only the file or folder pointed to by the path is checked.\
    /// Returns `true` if the attributes `Hidden` and `System` are set.
    ///
    /// # Errors
    /// 
    /// Errors if the path does not exist or an OS error occurs.
    fn is_super_hidden(&self) -> AreiaResult<bool> {
        if cfg!(target_os = "linux") {
            Err(AreiaError::SuperHidingNotSupported("No super hiding on Linux".to_string()))
        } else {
            if !self.exists() {
                return Err(AreiaError::SuperHidingRequiresExistingPath(self.clone()));
            }
            todo!()
        }
    }

    /// Super hides the file or directory pointed to by the path.\
    /// The path must point to an existing file or directory.\
    /// This concept only applies to MacOS and Windows.
    ///
    /// `super hidden` is a file or directory that is hidden, as achieved with using `.hide()` provided by `areia`, and marked with a platform specific attribute or flag.
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// ### Linux
    ///
    /// Always returns `AreiaError::SuperHidingNotSupported`.
    ///
    /// ### MacOS
    ///
    /// Does nothing if the file or directory is already super hidden.
    /// Otherwise, the file or directory is hidden and marked with the `hidden` flag.
    ///
    /// ## Windows
    ///
    /// Does nothing if the file or directory is already super hidden.
    /// Otherwise, the file or directory is hidden and marked with the `Hidden` and `System` attributes.
    ///
    /// # Errors
    /// 
    /// Errors if the path does not exist or an OS error occurs.
    fn super_hide(&mut self) -> AreiaResult<PathBuf> {
        if cfg!(target_os = "linux") {
            Err(AreiaError::SuperHidingNotSupported("No super hiding on Linux".to_string()))
        } else {
            if !self.exists() {
                return Err(AreiaError::SuperHidingRequiresExistingPath(self.clone()));
            }
            if self.is_super_hidden()? {
                return Ok(self.clone());
            }
            todo!()
        }
    }

    /// Super un-hides the file or directory pointed to by the path.\
    /// The reverse of `.super_hide()`.\
    /// The path must point to an existing file or directory.\
    /// This concept only applies to MacOS and Windows.
    ///
    /// `super hidden` is a file or directory that is hidden, as achieved with using `.hide()` provided by `areia`, and marked with a platform specific attribute or flag.
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// ### Linux
    ///
    /// Always returns `AreiaError::SuperHidingNotSupported`.
    ///
    /// ### MacOS
    ///
    /// Does nothing if the file or directory is not super hidden.
    /// Otherwise, the file or directory is unhidden and unmarked with the `hidden` flag.
    ///
    /// ## Windows
    ///
    /// Does nothing if the file or directory is not super hidden.
    /// Otherwise, the file or directory is unhidden and unmarked with the `Hidden` and `System` attributes.
    ///
    /// # Errors
    /// 
    /// Errors if the path does not exist or an OS error occurs.
    fn super_unhide(&mut self) -> AreiaResult<PathBuf> {
        if cfg!(target_os = "linux") {
            Err(AreiaError::SuperHidingNotSupported("No super hiding on Linux".to_string()))
        } else {
            if !self.exists() {
                return Err(AreiaError::SuperHidingRequiresExistingPath(self.clone()));
            }
            if !self.is_super_hidden()? {
                return Ok(self.clone());
            }
            todo!()
        }
    }
}
