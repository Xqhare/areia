use std::{collections::HashMap, path::PathBuf};

use crate::error::{AreiaError, AreiaResult};

pub fn cache_dir(home: PathBuf) -> PathBuf {
    let out = home.join("Library").join("Caches");
    if out.exists() {
        out
    } else {
        unreachable!("cache dir does not exist")
    }
}

pub fn config_dir(home: PathBuf) -> PathBuf {
    let out = home.join("Library").join("Application Support");
    if out.exists() {
        out
    } else {
        unreachable!("config dir does not exist")
    }
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
    let out = home.join("Library").join("Preferences");
    if out.exists() {
        out
    } else {
        unreachable!("preference dir does not exist")
    }
}

fn font_dir(home: PathBuf) -> Option<PathBuf> {
    let out = home.join("Library").join("Fonts");
    if out.exists() {
        Some(out)
    } else {
        None
    }
}

pub fn get_usr_dirs<P: Into<PathBuf>>(home: P) -> AreiaResult<HashMap<String, Option<PathBuf>>> {
    let home = home.into();
    let mut out = HashMap::new();
    let music = home.join("Music");
    if !music.exists() {
        AreiaError::MacError("Music directory does not exist".to_owned());
    }
    let desktop = home.join("Desktop");
    if !desktop.exists() {
        AreiaError::MacError("Desktop directory does not exist".to_owned());
    }
    let documents = home.join("Documents");
    if !documents.exists() {
        AreiaError::MacError("Documents directory does not exist".to_owned());
    }
    let downloads = home.join("Downloads");
    if !downloads.exists() {
        AreiaError::MacError("Downloads directory does not exist".to_owned());
    }
    let pictures = home.join("Pictures");
    if !pictures.exists() {
        AreiaError::MacError("Pictures directory does not exist".to_owned());
    }
    let public = home.join("Public");
    if !public.exists() {
        AreiaError::MacError("Public directory does not exist".to_owned());
    }
    let movies = home.join("Movies");
    if !movies.exists() {
        AreiaError::MacError("Movies directory does not exist".to_owned());
    }

    out.insert("MUSIC".to_owned(), Some(music));
    out.insert("DESKTOP".to_owned(), Some(desktop));
    out.insert("DOCUMENTS".to_owned(), Some(documents));
    out.insert("DOWNLOADS".to_owned(), Some(downloads));
    out.insert("PICTURES".to_owned(), Some(pictures));
    out.insert("PUBLICSHARE".to_owned(), Some(public));
    out.insert("VIDEOS".to_owned(), Some(movies));
    out.insert("FONTS".to_owned(), font_dir(home));
    out.insert("TEMPLATES".to_owned(), None);
    Ok(out)
}
