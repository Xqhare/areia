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

fn font_dir(home: PathBuf) -> Option<PathBuf> {
    Some(home.join("Library").join("Fonts"))
}

pub fn get_usr_dirs<P: Into<PathBuf>>(home: P) -> AreiaResult<HashMap<String, Option<PathBuf>>> {
    let home = home.into();
    let mut out = HashMap::new();
    out.insert("MUSIC".to_owned(), Some(home.join("Music")));
    out.insert("DESKTOP".to_owned(), Some(home.join("Desktop")));
    out.insert("DOCUMENTS".to_owned(), Some(home.join("Documents")));
    out.insert("DOWNLOADS".to_owned(), Some(home.join("Downloads")));
    out.insert("PICTURES".to_owned(), Some(home.join("Pictures")));
    out.insert("PUBLICSHARE".to_owned(), Some(home.join("Public")));
    out.insert("VIDEOS".to_owned(), Some(home.join("Movies")));
    out.insert("FONTS".to_owned(), font_dir(home));
    out.insert("TEMPLATES".to_owned(), None);
    Ok(out)
}
