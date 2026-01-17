use std::path::PathBuf;

use crate::error::{AreiaError, AreiaResult};
use crate::utils::{hide_path, make_hidden_path, unhide_path};
#[cfg(target_os = "windows")]
use crate::utils::windows::is_windows_hidden;

use super::Hidden;

impl Hidden for PathBuf {
    fn is_hidden(&self) -> AreiaResult<bool> {
        is_path_empty(&self)?;
        crate::utils::is_hidden(&self)
    }
    
    fn hide(&mut self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        if self.is_hidden()? {
            return Ok(self.clone());
        }
        hide_path(self)
    }

    fn unhide(&mut self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        unhide_path(self)
    }
    fn into_hidden_path(self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        #[cfg(unix)]
        return Ok(make_hidden_path(&self));
        #[cfg(windows)]
        return Err(AreiaError::MakingHiddenPathNotSupported("Unavailable on Windows".to_string()));
    }
}

fn is_path_empty(path: &PathBuf) -> AreiaResult<()> {
    if path.as_os_str().is_empty() {
        return Err(AreiaError::PathMustBeSomething(path.clone()));
    }
    Ok(())
}
