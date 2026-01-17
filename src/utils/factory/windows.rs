use std::{collections::HashMap, path::PathBuf};

use crate::{
    error::AreiaResult,
    utils::ffi::windows::{FolderID, get_path},
};

// TODO: If `get_path` returns an error, it is currently suppressed and an empty path is returned - I should fix that

pub fn cache_dir(home: PathBuf) -> PathBuf {
    data_local_dir(home)
}

pub fn config_dir(home: PathBuf) -> PathBuf {
    data_dir(home)
}

pub fn data_dir(_home: PathBuf) -> PathBuf {
    match get_path(FolderID::RoamingAppData) {
        Ok(path) => path,
        Err(_) => PathBuf::new(),
    }
}

pub fn data_local_dir(_home: PathBuf) -> PathBuf {
    match get_path(FolderID::LocalAppData) {
        Ok(path) => path,
        Err(_) => PathBuf::new(),
    }
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

pub fn config_local_dir(home: PathBuf) -> PathBuf {
    data_local_dir(home)
}

pub fn preference_dir(home: PathBuf) -> PathBuf {
    data_dir(home)
}

fn font_dir(_home: PathBuf) -> Option<PathBuf> {
    None
}

pub fn get_usr_dirs<P: Into<PathBuf>>(home: P) -> AreiaResult<HashMap<String, Option<PathBuf>>> {
    let mut out = HashMap::new();

    out.insert("MUSIC".to_string(), Some(get_path(FolderID::Music)?));
    out.insert("DESKTOP".to_string(), Some(get_path(FolderID::Desktop)?));
    out.insert("DOCUMENTS".to_string(), Some(get_path(FolderID::Document)?));
    out.insert("PICTURES".to_string(), Some(get_path(FolderID::Picture)?));
    out.insert("PUBLICSHARE".to_string(), Some(get_path(FolderID::Public)?));
    out.insert("TEMPLATES".to_string(), Some(get_path(FolderID::Template)?));
    out.insert("VIDEOS".to_string(), Some(get_path(FolderID::Video)?));
    out.insert("FONTS".to_string(), font_dir(home.into()));
    out.insert("DOWNLOADS".to_string(), Some(get_path(FolderID::Download)?));
    Ok(out)
}
