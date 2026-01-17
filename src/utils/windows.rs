use crate::error::{AreiaError, AreiaResult};
use std::path::PathBuf;

use super::ffi::windows;

/// Get the user's home directory
///
/// Works on MacOS, Linux, 
/// Behaviour of `std::env::home_dir()` on Windows is apparently problematic
///
/// # Returns
/// `Ok(path)` if the home directory could be found
/// `Err(AreiaError::CantGetHomeDir)` if the home directory could not be found
pub fn get_home() -> AreiaResult<PathBuf> {
    if let Ok(path) = windows::get_path(windows::FolderID::Profile) {
        Ok(path)
    } else { 
        if let Some(path) = std::env::var_os("USERPROFILE") {
            Ok(path.into())
        } else {
            Err(AreiaError::CantGetHomeDir)
        }
    }
}

/// Checks only the file pointed to, not the entire path like with unix
pub fn is_any_component_hidden(path: &PathBuf) -> AreiaResult<bool> {
    windows::is_hidden(path)
}

pub fn is_superhidden(path: &PathBuf) -> AreiaResult<bool> {
    windows::is_super_hidden(path)
}

pub fn super_hide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    windows::super_hide(path)
}

pub fn super_unhide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    windows::super_unhide(path)
}
