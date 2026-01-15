use std::path::PathBuf;

use crate::error::AreiaResult;
use crate::utils::{hide_path, is_hidden, unhide_path};
#[cfg(target_os = "windows")]
use crate::utils::windows::is_windows_hidden;

use super::Hidden;

impl Hidden for PathBuf {
    /// Returns true if the path is hidden
    /// 
    /// # Platform specific behaviour
    ///
    /// ## Unix
    ///
    /// Checks if any component of the path is hidden (`.` prefix).
    /// Does not check if path exists.
    /// Returns `true` if any component is hidden.
    /// Returns `false` if any component is not hidden.
    ///
    /// ## Windows
    ///
    /// Checks for the hidden attribute on the file or folder pointed to with the path.
    /// Returns `true` if the file or folder is hidden.
    /// Returns `false` if the file or folder is not hidden.
    /// SPECIAL: Returns `false` if the path does not exist or an error occurs.
    fn is_hidden(&self) -> bool {
        #[cfg(unix)]
        return is_hidden(&self);
        #[cfg(windows)]
        if self.exists() {
            // `is_windows_hidden` may return an os error - while returning false is suboptimal
            // I really don't want to `Result` the return value here
            is_windows_hidden(&self).unwrap_or(false)
        } else {
            false
        }
    }

    /// Hides the path
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// Checks if any component of the path is already hidden.
    /// If true, returns the path unmodified and does nothing.
    /// If no part of the path is hidden, the last directory or file contained in the path is hidden.
    /// 
    /// The returned `PathBuf` is the new hidden path.
    /// If the path exists, the last directory or file in the path is moved to be hidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// If the path does not exist, it is created.
    /// 
    /// ## Windows
    /// 
    /// Checks for the hidden attribute on the file or folder pointed to with the path.
    /// If the file or folder is already hidden, it does nothing.
    /// If the file or folder is not hidden, the file or folder is hidden.
    /// 
    /// The returned `PathBuf` is the provided path.
    /// If the path exists, the file or folder is moved to be hidden.
    /// If the path does not exist, it is created.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// 
    /// # Errors
    ///
    /// Errors if the program has insufficient permissions to move the file or folder.
    fn hide(&mut self) -> AreiaResult<PathBuf> {
        #[cfg(unix)]
        if self.is_hidden() {
            Ok(self.clone())
        } else {
            hide_path(self)
        }
        #[cfg(windows)]
        if self.is_hidden() {
            Ok(self.clone())
        } else {
            hide_path(self)
        }
    }

    /// Unhides a hidden path
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// Checks if any component of the path is hidden.
    /// If true, the last directory or file contained in the path is unhidden and the new path is returned.
    /// Should the file be located in a hidden system path, the path is not unhidden and an error is returned.
    ///
    /// If no part of the path is hidden, returns the path unmodified and does nothing.
    /// 
    /// The returned `PathBuf` is the new unhidden path.
    /// If the path exists, the last directory or file in the path is moved to be unhidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// If the path does not exist, the new hidden path is returned.
    /// 
    /// ## Windows
    /// 
    /// Checks for the hidden attribute on the file or folder pointed to with the path.
    /// If the file or folder is already visible, it does nothing.
    /// If the file or folder is hidden, the file or folder is unhidden.
    /// 
    /// The returned `PathBuf` is the provided path.
    /// If the path exists, the file or folder is moved to be unhidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// If the path does not exist, an error is returned.
    /// 
    /// # Errors
    ///
    /// Errors if the program has insufficient permissions to move the file or folder.
    fn unhide(&mut self) -> AreiaResult<PathBuf> {
        #[cfg(unix)]
        unhide_path(self)
    }
}
