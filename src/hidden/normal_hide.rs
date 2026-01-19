use std::path::PathBuf;

use crate::error::{AreiaError, AreiaResult};
use crate::utils::{hide_path, unhide_path};
#[cfg(unix)]
use crate::utils::{make_hidden_path, make_unhidden_path};

use super::Hidden;

impl Hidden for PathBuf {
    fn is_hidden(&self) -> AreiaResult<bool> {
        is_path_empty(&self)?;
        crate::utils::is_hidden(&self)
    }

    fn hide(&mut self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        hide_path(self)
    }

    fn unhide(&mut self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        unhide_path(self)
    }

    fn try_into_hidden_path(&self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        if cfg!(target_os = "windows") {
            return Err(AreiaError::MakingHiddenPathNotSupported(
                "Unavailable on Windows".to_string(),
            ));    
        } 
        if cfg!(not(target_os = "windows")) {
            return Ok(make_hidden_path(&self));
        }
        unreachable!("Only Linux, macOS and Windows are supported")
    }

    fn try_into_unhidden_path(&self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        if cfg!(target_os = "windows") {
            return Err(AreiaError::MakingHiddenPathNotSupported(
                "Unavailable on Windows".to_string(),
            ));
        } 
        if cfg!(not(target_os = "windows")) {
            return make_unhidden_path(&self);
        }
        unreachable!("Only Linux, macOS and Windows are supported")
    }
}

fn is_path_empty(path: &PathBuf) -> AreiaResult<()> {
    if path.as_os_str().is_empty() {
        return Err(AreiaError::PathMustBeSomething(path.clone()));
    }
    Ok(())
}
