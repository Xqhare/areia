mod platform;
use crate::error::AreiaResult;
use std::path::PathBuf;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use unix as os;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

pub fn get_home() -> AreiaResult<PathBuf> {
    os::get_home()
}
