use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use crate::{
    BaseDirs, UserDirs,
    error::{AreiaError, AreiaResult},
};

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
#[cfg(target_os = "linux")]
pub fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() { Some(path) } else { None }
}

/// only handles unix dotfiles
fn is_component_hidden(path: &OsStr) -> bool {
    path.to_str()
        .expect("Unix path is valid UTF-8 by convention")
        .starts_with(".")
}

pub fn is_any_component_hidden(path: &PathBuf) -> AreiaResult<bool> {
    Ok(path
        .components()
        .any(|c| is_component_hidden(c.as_os_str())))
}

/// Takes in a path, and returns a new path where the last component is hidden.
/// Returns the path unmodified if the last component is already hidden
pub fn make_hidden_path(path: &PathBuf) -> PathBuf {
    let mut new_path = path.clone();
    if is_any_component_hidden(&new_path).expect("Always Ok") {
        new_path
    } else {
        new_path.set_file_name(format!(
            ".{}",
            new_path
                .file_name()
                .unwrap()
                .to_str()
                .expect("Unix path is valid UTF-8 by convention")
        ));
        new_path
    }
}

/// First checks if any component is hidden.
/// If yes -> returns the path and does nothing
/// If no -> moves the last component to be hidden, returns the new path
pub fn hide_file(path: &PathBuf) -> AreiaResult<PathBuf> {
    if is_any_component_hidden(&path).expect("Always Ok") {
        return Ok(path.clone());
    }
    let old_path = path.clone();
    let new_path = make_hidden_path(&path);
    if !old_path.exists() {
        create_all_dir_with_file(&old_path)?;
    }
    atomic_move(&old_path, &new_path)?;
    Ok(new_path)
}

/// First checks if path points inside a hidden system folder.
/// If yes, returns an Error
/// If no, moves the first hidden component to be unhidden and returns the new path
pub fn unhide_file(path: &PathBuf) -> AreiaResult<PathBuf> {
    let old_path = path.clone();
    let mut new_path = path.clone();

    if !is_any_component_hidden(&old_path).expect("Always Ok") {
        return Ok(old_path);
    }

    // check if any path is equal to a system directory
    // - On mac:
    //     - there are no hidden system directories
    // - On linux:
    //     - Of all `BaseDir` only runtime and home is not hidden
    //     - Of all `UserDir` only font is hidden
    let base_dirs = BaseDirs::new()?;
    let user_dirs = UserDirs::new()?;
    let mut system_dirs = Vec::new();

    system_dirs.push(base_dirs.cache_dir());
    system_dirs.push(base_dirs.config_dir());
    system_dirs.push(base_dirs.config_local_dir());
    system_dirs.push(base_dirs.data_dir());
    system_dirs.push(base_dirs.data_local_dir());
    system_dirs.push(base_dirs.preference_dir());
    if let Some(path) = base_dirs.state_dir() {
        system_dirs.push(path);
    }
    if let Some(path) = base_dirs.executable_dir() {
        system_dirs.push(path);
    }

    if let Some(path) = user_dirs.font_dir() {
        system_dirs.push(path);
    }

    for dir in system_dirs {
        if path.starts_with(dir) {
            return Err(AreiaError::HiddenFileInsideSystemDir(path.clone()));
        }
    }

    // check for the first hidden component, remove the leading dot, reconstruct the path
    let mut new_components = Vec::new();
    for component in new_path.components() {
        if is_component_hidden(component.as_os_str()) {
            let unhidden_component = component
                .as_os_str()
                .to_str()
                .expect("Unix path is valiud UTF-8 by convention")
                .strip_prefix(".")
                .expect("Component must be hidden");
            let tmp = OsString::from(unhidden_component);
            new_components.push(tmp);
        } else {
            new_components.push(component.as_os_str().to_os_string());
        }
    }
    new_path = PathBuf::from_iter(new_components);

    if old_path.exists() {
        atomic_move(&old_path, &new_path)?;
    }
    return Ok(new_path);
}

fn create_all_dir_with_file(path: &PathBuf) -> AreiaResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::File::create(path)?;
    Ok(())
}

// I suspect this wont work as I expect it to - especially with several nested dirs
fn atomic_move(from: &PathBuf, to: &PathBuf) -> AreiaResult<()> {
    std::fs::rename(from, to)?;
    Ok(())
}
