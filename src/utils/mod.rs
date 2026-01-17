mod ffi;
use crate::error::{AreiaError, AreiaResult};
use std::path::PathBuf;

pub mod factory;

#[cfg(any(unix, doc))]
mod unix;

#[cfg(unix)]
use unix as os;

#[cfg(any(target_os = "macos", doc))]
mod macos;

#[cfg(any(target_os = "windows", all(target_os = "windows", doc)))]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

pub fn get_home() -> AreiaResult<PathBuf> {
    os::get_home()
}

pub fn hide_path(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    os::hide_file(path)
}

pub fn unhide_path(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    os::unhide_file(path)
}

pub fn is_hidden(path: &PathBuf) -> AreiaResult<bool> {
    os::is_any_component_hidden(path)
}

// Allows because I want cargo check to shut up
#[allow(unused_variables, unreachable_code)]
/// Only supports Mac & Windows
pub fn is_superhidden(path: &PathBuf) -> AreiaResult<bool> {
    #[cfg(target_os = "macos")]
    return macos::is_superhidden(path);

    #[cfg(target_os = "windows")]
    return os::is_superhidden(path);

    Err(AreiaError::SuperHidingNotSupported(
        "Super hiding not supported on this OS".to_string(),
    ))
}

#[allow(unused_variables, unreachable_code)]
/// Only supports Mac & Windows
pub fn super_hide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    #[cfg(target_os = "macos")]
    return macos::superhide(path);

    #[cfg(target_os = "windows")]
    return os::super_hide(path);

    Err(AreiaError::SuperHidingNotSupported(
        "Super hiding not supported on this OS".to_string(),
    ))
}

#[allow(unused_variables, unreachable_code)]
/// Only supports Mac & Windows
pub fn super_unhide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    #[cfg(target_os = "macos")]
    return macos::super_unhide(path);

    #[cfg(target_os = "windows")]
    return os::super_unhide(path);

    Err(AreiaError::SuperHidingNotSupported(
        "Super hiding not supported on this OS".to_string(),
    ))
}

#[cfg(unix)]
pub fn make_hidden_path(path: &PathBuf) -> PathBuf {
    os::make_hidden_path(path)
}
