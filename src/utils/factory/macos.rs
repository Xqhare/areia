use std::{collections::HashMap, path::PathBuf};

use crate::error::AreiaResult;

pub fn cache_dir(home: PathBuf) -> PathBuf {
    home.join("Library").join("Caches")
}

pub fn config_dir(home: PathBuf) -> PathBuf {
    home.join("Library").join("Application Support")
}

pub fn config_local_dir(home: PathBuf) -> PathBuf {
    config_dir(home)
}

pub fn data_dir(home: PathBuf) -> PathBuf {
    config_dir(home)
}

pub fn data_local_dir(home: PathBuf) -> PathBuf {
    config_dir(home)
}

pub fn runtime_dir() -> Option<PathBuf> {
    None
}

pub fn state_dir(_home: PathBuf) -> Option<PathBuf> {
    None
}

pub fn executable_dir(_home: PathBuf) -> Option<PathBuf> {
    None
}

pub fn preference_dir(home: PathBuf) -> PathBuf {
    home.join("Library").join("Preferences")
}
pub fn font_dir(home: PathBuf) -> Option<PathBuf> {
    Some(home.join("Library").join("Fonts"))
}

pub fn get_usr_dirs<P: Into<PathBuf>>(home: P) -> AreiaResult<HashMap<String, PathBuf>> {
    let home = home.into();
    let mut out = HashMap::new();
    out.insert("Music".to_owned(), home.join("Music"));
    out.insert("Desktop".to_owned(), home.join("Desktop"));
    out.insert("Documents".to_owned(), home.join("Documents"));
    out.insert("Downloads".to_owned(), home.join("Downloads"));
    out.insert("Pictures".to_owned(), home.join("Pictures"));
    out.insert("Public".to_owned(), home.join("Public"));
    out.insert("Videos".to_owned(), home.join("Movies"));
    Ok(out)
}
