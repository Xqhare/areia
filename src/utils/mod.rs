mod ffi;
use crate::error::AreiaResult;
use std::path::PathBuf;

pub mod factory;

#[cfg(any(unix, doc))]
mod unix;

#[cfg(unix)]
use unix as os;

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

pub fn is_hidden(path: &PathBuf) -> bool {
    os::is_any_component_hidden(path)
}
