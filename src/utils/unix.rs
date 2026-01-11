use crate::error::{AreiaError, AreiaResult};

use super::platform::unix::get_unix_home_fallback;

#[cfg(target_os = "macos")]
use super::platform::macos::get_mac_home_fallback;

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

/// Get the path to the current executable
///
/// Only supported on Linux
///
/// # Returns
/// `Ok(path)` if the path could be found
/// `Err(AreiaError::IoError(err))` if IO error occurred
pub fn get_exe() -> AreiaResult<std::path::PathBuf> {
    match std::env::current_exe() {
        Ok(path) => Ok(path),
        Err(err) => Err(AreiaError::IoError(err)),
    }
}
