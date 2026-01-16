use std::path::PathBuf;

use crate::error::AreiaResult;
use crate::utils::{hide_path, is_hidden, unhide_path};
#[cfg(target_os = "windows")]
use crate::utils::windows::is_windows_hidden;

use super::Hidden;

impl Hidden for PathBuf {
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

    fn unhide(&mut self) -> AreiaResult<PathBuf> {
        #[cfg(unix)]
        unhide_path(self)
    }
}
