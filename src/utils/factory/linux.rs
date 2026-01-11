use std::{env, path::PathBuf};

use crate::utils::unix::{is_absolute_path, is_existent_path};


pub fn cache_dir(home: PathBuf) -> PathBuf {
    if let Some(path) = env::var_os("XDG_CACHE_HOME").and_then(|x| is_absolute_path(x)) {
        path
    } else {
        if let Some(path) = is_existent_path(home.join(".cache")) {
            path
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("Cache directory could not be found!");
        }
    }
}

pub fn config_dir(home: PathBuf) -> PathBuf {
    if let Some(path) = env::var_os("XDG_CONFIG_HOME").and_then(|x| is_absolute_path(x)) {
        path
    } else {
        if let Some(path) = is_existent_path(home.join(".config")) {
            path
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("Config directory could not be found!");
        }
    }
}

pub fn data_dir(home: PathBuf) -> PathBuf {
    if let Some(path) = env::var_os("XDG_DATA_HOME").and_then(|x| is_absolute_path(x)) {
        path
    } else {
        if let Some(path) = is_existent_path(home.join(".local/share")) {
            path
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("Data directory could not be found!");
        }
    }
}

pub fn runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_DIR").and_then(|x| is_absolute_path(x))
}

pub fn state_dir(home: PathBuf) -> PathBuf {
    if let Some(path) = env::var_os("XDG_STATE_HOME").and_then(|x| is_absolute_path(x)) {
        path
    } else {
        if let Some(path) = is_existent_path(home.join(".local/state")) {
            path
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("State directory could not be found!");
        }
    }
}

pub fn executable_dir(home: PathBuf) -> PathBuf {
    if let Some(path) = env::var_os("XDG_BIN_HOME").and_then(|x| is_absolute_path(x)) {
        path
    } else {
        if let Some(path) = is_existent_path(home.join(".local/bin")) {
            path
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("Executable directory could not be found!");
        }
    }
}

pub fn font_dir(home: PathBuf) -> PathBuf {
    if let Some(path) = is_existent_path(home.join(".local/share/fonts")) {
        path
    } else {
        // Judging by the `directories` source code, `home.join` should always return a valid path
        unreachable!("Font directory could not be found!");
    }
}

// todo!("for rest of user dirs, read `home/.config/user-dirs.dirs` and parse that");
