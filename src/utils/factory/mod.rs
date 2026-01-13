
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as os;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as os;

use std::{collections::HashMap, path::PathBuf};

use crate::error::AreiaResult;

pub fn cache_dir(home: PathBuf) -> PathBuf {
    os::cache_dir(home)
}

pub fn config_dir(home: PathBuf) -> PathBuf {
    os::config_dir(home)
}

pub fn config_local_dir(home: PathBuf) -> PathBuf {
    os::config_local_dir(home)
}

pub fn data_dir(home: PathBuf) -> PathBuf {
    os::data_dir(home)
}

pub fn data_local_dir(home: PathBuf) -> PathBuf {
    os::data_local_dir(home)
}

pub fn preference_dir(home: PathBuf) -> PathBuf {
    os::preference_dir(home)
}

pub fn runtime_dir() -> Option<PathBuf> {
    os::runtime_dir()
}

pub fn state_dir(home: PathBuf) -> Option<PathBuf> {
    os::state_dir(home)
}

pub fn executable_dir(home: PathBuf) -> Option<PathBuf> {
    os::executable_dir(home)
}

pub fn get_usr_dirs(home: PathBuf) -> AreiaResult<HashMap<String, Option<PathBuf>>> {
    os::get_usr_dirs(home)
}
