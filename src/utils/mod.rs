mod ffi;
use crate::error::AreiaResult;
use std::{ffi::OsStr, path::PathBuf};

pub mod factory;

#[cfg(any(unix, doc))]
mod unix;

#[cfg(unix)]
use unix as os;

#[cfg(any(target_os = "windows", all(target_os = "windows", doc)))]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;
#[cfg(target_os = "windows")]
pub use windows::is_windows_hidden;

pub fn get_home() -> AreiaResult<PathBuf> {
    os::get_home()
}

/// only handles unix dotfiles
pub fn is_component_hidden(path: &OsStr) -> bool {
    path.to_str().expect("Unix path is valid UTF-8 by convention").starts_with(".")
}

