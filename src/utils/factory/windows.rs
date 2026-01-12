pub fn cache_dir(home: PathBuf) -> PathBuf {
    todo!(cache_dir(home))
}

pub fn config_dir(home: PathBuf) -> PathBuf {
    todo!(config_dir(home))
}

pub fn data_dir(home: PathBuf) -> PathBuf {
    todo!(data_dir(home))
}

pub fn runtime_dir() -> Option<PathBuf> {
    todo!(runtime_dir())
}

pub fn state_dir(home: PathBuf) -> Option<PathBuf> {
    todo!(state_dir(home))
}

pub fn executable_dir(home: PathBuf) -> Option<PathBuf> {
    todo!(executable_dir(home))
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

pub fn font_dir(home: PathBuf) -> PathBuf {
    todo!(font_dir(home))
}

pub fn get_usr_dirs<P: Into<PathBuf>>(home: P) -> AreiaResult<HashMap<String, PathBuf>> {
    todo!(get_usr_dirs(home))
}
