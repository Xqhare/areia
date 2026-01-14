mod utils;
mod dirs;
use std::path::PathBuf;

pub use dirs::{BaseDirs, UserDirs};
use error::AreiaResult;
use utils::is_component_hidden;
#[cfg(target_os = "windows")]
use utils::windows;

pub mod error;

pub trait Hidden {
    fn is_hidden(&self) -> bool;
    fn hide(&mut self) -> AreiaResult<PathBuf>;
    fn unhide(&mut self) -> AreiaResult<PathBuf>;
    fn is_super_hidden(&self) -> bool;
    fn super_hide(&mut self) -> AreiaResult<PathBuf>;
    fn super_unhide(&mut self) -> AreiaResult<PathBuf>;
}

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
        return self.components().any(|c| is_component_hidden(c.as_os_str()));
        #[cfg(windows)]
        if self.exists() {
            // `is_hidden` may return an os error - while returning false is suboptimal
            // I really don't want to `Result` the return value here
            windows::is_hidden(&self).unwrap_or(false)
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
    /// If not, the last directory contained in the path is hidden.
    /// 
    /// The returned `PathBuf` is the new hidden path.
    /// If the path exists, the directory the path points to is hidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// 
    /// Returns an error if the path does not exist
    fn hide(&mut self) -> AreiaResult<PathBuf> {
        todo!()
    }

    fn unhide(&mut self) -> AreiaResult<PathBuf> {
        todo!()
    }
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
