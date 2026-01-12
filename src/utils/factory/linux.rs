use std::{collections::HashMap, env, ffi::OsString, fs::File, io::Read, os::unix::ffi::OsStringExt, path::PathBuf};

use crate::{error::{AreiaError, AreiaResult}, utils::unix::{is_absolute_path, is_existent_path}};

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

pub fn config_local_dir(home: PathBuf) -> PathBuf {
    config_dir(home)
}

pub fn data_local_dir(home: PathBuf) -> PathBuf {
    data_dir(home)
}

pub fn preference_dir(home: PathBuf) -> PathBuf {
    config_dir(home)
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

pub fn state_dir(home: PathBuf) -> Option<PathBuf> {
    if let Some(path) = env::var_os("XDG_STATE_HOME").and_then(|x| is_absolute_path(x)) {
        Some(path)
    } else {
        if let Some(path) = is_existent_path(home.join(".local/state")) {
            Some(path)
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("State directory could not be found!");
        }
    }
}

pub fn executable_dir(home: PathBuf) -> Option<PathBuf> {
    if let Some(path) = env::var_os("XDG_BIN_HOME").and_then(|x| is_absolute_path(x)) {
        Some(path)
    } else {
        if let Some(path) = is_existent_path(home.join(".local/bin")) {
            Some(path)
        } else {
            // Judging by the `directories` source code, `home.join` should always return a valid path
            unreachable!("Executable directory could not be found!");
        }
    }
}

pub fn font_dir(home: PathBuf) -> Option<PathBuf> { is_existent_path(home.join(".local/share/fonts")) }

pub fn get_usr_dirs<P: Into<PathBuf>>(home: P) -> AreiaResult<HashMap<String, PathBuf>> {
    let home = home.into();
    let usr_xdg_file = home.clone().join(".config").join("user-dirs.dirs");
    match read_to_byte_vec(usr_xdg_file) {
        Ok(bytes) => {
            parse_xdg_dirs(bytes, home.into())
        },
        Err(err) => Err(AreiaError::IoError(err)),
    }
}

fn read_to_byte_vec(path: PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    // 1024 is used by `directories` - on my machine the file is 633 bytes; seems prudent to use a
    //      somewhat larger buffer
    let mut bytes = Vec::with_capacity(1024);
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn split_once(bytes: &[u8], sep: u8) -> Option<(&[u8], &[u8])> {
    for (i, byte) in bytes.iter().enumerate() {
        if *byte == sep {
            return Some((&bytes[..i], &bytes[i + 1..]));
        }
    }
    None
}
fn parse_xdg_dirs(bytes: Vec<u8>, home: PathBuf) -> AreiaResult<HashMap<String, PathBuf>> {
    let mut out = HashMap::new();

    // We always continue (fail forwards) as not all lines are valid user dirs - there is normal
    //      text in there
    for line in bytes.split(|x| *x == b'\n') {
        let (key, val) = match split_once(line, b'=') {
            Some((key, val)) => (key, val),
            None => continue,
        };

        let key = key.trim_ascii();
        let key = if key.starts_with(b"XDG_") && key.ends_with(b"_DIR") {
            match str::from_utf8(&key[4..key.len() - 4]) {
                Ok(key) => key,
                // Err should never match, but just in case
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let val = val.trim_ascii();
        let mut val = if val.starts_with(b"\"") && val.ends_with(b"\"") {
            &val[1..val.len() - 1]
        } else {
            continue;
        };

        let path_is_relative = if val == b"$HOME/" {
            // "To disable a directory, point it to the homedir"
            // XDG doc
            continue;
        } else if val.starts_with(b"$HOME/") {
            val = &val[b"$HOME/".len()..];
            true
        } else if val.starts_with(b"/") {
            false
        } else {
            continue;
        };

        // Still need to remove all b'\\' from val (escaped characters)
        let val = OsString::from_vec(val.iter().copied().filter(|x| *x != b'\\').collect());

        let path = if path_is_relative {
            let mut path = home.clone();
            path.push(val);
            path
        } else {
            PathBuf::from(val)
        };
        
        out.insert(key.to_owned(), path);
    }

    Ok(out)
}

#[test]
fn xdg_user_dir_parsing() {
    let home = env::var_os("HOME").unwrap();
    let home_path = PathBuf::from(home.clone());
    let dirs = get_usr_dirs(home).unwrap();

    assert_eq!(dirs.len(), 8);

    assert_eq!(dirs.get("DESKTOP").unwrap(), &home_path.join("Desktop"));
    assert_eq!(dirs.get("DOCUMENTS").unwrap(), &home_path.join("Documents"));
    assert_eq!(dirs.get("DOWNLOAD").unwrap(), &home_path.join("Downloads"));
    assert_eq!(dirs.get("MUSIC").unwrap(), &home_path.join("Music"));
    assert_eq!(dirs.get("PICTURES").unwrap(), &home_path.join("Pictures"));
    assert_eq!(dirs.get("PUBLICSHARE").unwrap(), &home_path.join("Public"));
    assert_eq!(dirs.get("TEMPLATES").unwrap(), &home_path.join("Templates"));
    assert_eq!(dirs.get("VIDEOS").unwrap(), &home_path.join("Videos"));
}
