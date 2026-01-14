use std::{ffi::OsString, path::PathBuf};

use crate::error::{AreiaError, AreiaResult};

use super::ffi::unix::get_unix_home_fallback;

#[cfg(target_os = "macos")]
use super::ffi::macos::get_mac_home_fallback;

/// Get the user's home directory
///
/// Works on MacOS, Linux, 
/// Behaviour of `std::env::home_dir()` on Windows is apparently problematic
///
/// # Returns
/// `Ok(path)` if the home directory could be found
/// `Err(AreiaError::CantGetHomeDir)` if the home directory could not be found
pub fn get_home() -> AreiaResult<std::path::PathBuf> {
    if let Some(path) = std::env::var_os("HOME") {
        return Ok(path.into());
    } else {
        if let Some(path) = get_unix_home_fallback() {
            return Ok(path);
        } 

        #[cfg(target_os = "macos")]
        if let Some(path) = get_mac_home_fallback() {
            return Ok(path);
        }

        Err(AreiaError::CantGetHomeDir)
    }
}

/// Checks if supplied path is an absolute path
///
/// # Returns
/// `Some(path)` if the path is absolute, the path is converted to `PathBuf`
/// `None` if the path is not absolute
pub fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

/// Checks if supplied path exists
///
/// # Returns
/// `Some(path)` if the path exists, the path is converted to `PathBuf`
/// `None` if the path does not exist
pub fn is_existent_path<P: Into<PathBuf>>(path: P) -> Option<PathBuf> {
    let path = path.into();
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

pub fn hide_file(path: &PathBuf) -> AreiaResult<PathBuf> {
    let mut path = path.clone();
    path.set_file_name(format!(".{}", path.file_name().unwrap().to_str().expect("Unix path is valid UTF-8 by convention")));
    Ok(path)
}

pub fn unhide_file(path: &PathBuf) -> AreiaResult<PathBuf> {
    let mut path = path.clone();
    path.set_file_name(path.clone().strip_prefix(".").unwrap());
    Ok(path)
}

pub fn is_dotfile_hidden(path: &PathBuf) -> bool {
    path.file_name().unwrap().to_str().unwrap().starts_with(".")
}
