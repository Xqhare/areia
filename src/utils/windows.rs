use crate::error::{AreiaError, AreiaResult};

use super::ffi::windows;

/// Get the user's home directory
///
/// Works on MacOS, Linux, 
/// Behaviour of `std::env::home_dir()` on Windows is apparently problematic
///
/// # Returns
/// `Ok(path)` if the home directory could be found
/// `Err(AreiaError::CantGetHomeDir)` if the home directory could not be found
pub fn get_home() -> AreiaResult<std::path::PathBuf> {
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

/// Get the path to the current executable
///
/// Only supported on Linux
///
/// # Returns
/// `Ok(path)` if the path could be found
/// `Err(AreiaError::IoError(err))` if IO error occurred
pub fn get_exe() -> AreiaResult<std::path::PathBuf> {
    unimplemented!();
}
