use std::path::PathBuf;

use crate::error::{AreiaError, AreiaResult};
#[cfg(unix)]
use crate::utils::make_hidden_path;
use crate::utils::{hide_path, unhide_path};

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

    fn into_hidden_path(&self) -> AreiaResult<PathBuf> {
        is_path_empty(&self)?;
        #[cfg(unix)]
        return Ok(make_hidden_path(&self));
        #[cfg(windows)]
        return Err(AreiaError::MakingHiddenPathNotSupported(
            "Unavailable on Windows".to_string(),
        ));
    }
}

fn is_path_empty(path: &PathBuf) -> AreiaResult<()> {
    if path.as_os_str().is_empty() {
        return Err(AreiaError::PathMustBeSomething(path.clone()));
    }
    Ok(())
}
