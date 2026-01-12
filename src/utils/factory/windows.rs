use std::{collections::HashMap, path::PathBuf};

use crate::{error::AreiaResult, utils::ffi::windows::{get_path, FolderID}};

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
        Err(_) => PathBuf::new()
    }
}

pub fn data_local_dir(_home: PathBuf) -> PathBuf {
    match get_path(FolderID::LocalAppData) {
        Ok(path) => path,
        Err(_) => PathBuf::new()
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

pub fn font_dir(_home: PathBuf) -> Option<PathBuf> {
    None
}

pub fn get_usr_dirs<P: Into<PathBuf>>(_home: P) -> AreiaResult<HashMap<String, PathBuf>> {
    let mut out = HashMap::new();

    out.insert("Music".to_string(), get_path(FolderID::Music)?);
    out.insert("Desktop".to_string(), get_path(FolderID::Desktop)?);
    out.insert("Document".to_string(), get_path(FolderID::Document)?);
    out.insert("Download".to_string(), get_path(FolderID::Download)?);
    out.insert("Picture".to_string(), get_path(FolderID::Picture)?);
    out.insert("Public".to_string(), get_path(FolderID::Public)?);
    out.insert("Template".to_string(), get_path(FolderID::Template)?);
    out.insert("Video".to_string(), get_path(FolderID::Video)?);

    Ok(out)
}
