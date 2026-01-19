use crate::error::{AreiaError, AreiaResult};
use crate::BaseDirs;
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

pub fn hide_file(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    if !path.exists() {
        return Err(AreiaError::FileDoesNotExist(path.to_path_buf()));
    }
    windows::hide(path)?;
    Ok(path.clone())
}

pub fn unhide_file(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    if !path.exists() {
        return Err(AreiaError::FileDoesNotExist(path.to_path_buf()));
    }

    // need to check if inside hidden directory
    // if yes, do nothing, and return path
    let mut sys_dirs = Vec::new();
    let base_dirs = BaseDirs::new().unwrap();
    sys_dirs.push(base_dirs.cache_dir());
    sys_dirs.push(base_dirs.config_dir());
    sys_dirs.push(base_dirs.config_local_dir());
    sys_dirs.push(base_dirs.data_dir());
    sys_dirs.push(base_dirs.data_local_dir());
    sys_dirs.push(base_dirs.preference_dir());

    let mut check_path = PathBuf::new();
    for component in path.components() {
        check_path.push(component);
        for dir in sys_dirs {
            if check_path.starts_with(dir) {
                return Ok(path.clone())
            }
        }
    }
    windows::unhide(path)?;
    Ok(path.clone())
}

/// Checks not only the file pointed to, but also all components
pub fn is_any_component_hidden(path: &PathBuf) -> AreiaResult<bool> {
    let mut tmp = PathBuf::new();
    for component in path.components() {
        tmp.push(component);
        if windows::is_hidden(&tmp)? {
            return Ok(true);
        }
    }
}

pub fn is_superhidden(path: &PathBuf) -> AreiaResult<bool> {
    windows::is_super_hidden(path)
}

pub fn super_hide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    windows::super_hide(path)?;
    Ok(path.clone())
}

pub fn super_unhide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    windows::super_unhide(path)?;
    Ok(path.clone())
}
