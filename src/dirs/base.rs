use std::path::PathBuf;

use crate::{
    error::AreiaResult,
    utils::{
        factory::{
            cache_dir, config_dir, config_local_dir, data_dir, data_local_dir, executable_dir,
            preference_dir, runtime_dir, state_dir,
        },
        get_home,
    },
};

#[derive(Debug, Clone)]
pub struct BaseDirs {
    home_dir: PathBuf,
    cache_dir: PathBuf,
    config_dir: PathBuf,
    config_local_dir: PathBuf,
    data_dir: PathBuf,
    data_local_dir: PathBuf,
    executable_dir: Option<PathBuf>,
    preference_dir: PathBuf,
    runtime_dir: Option<PathBuf>,
    state_dir: Option<PathBuf>,
}

impl BaseDirs {
    pub fn new() -> AreiaResult<BaseDirs> {
        base_dirs()
    }

    pub fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn config_local_dir(&self) -> &PathBuf {
        &self.config_local_dir
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn data_local_dir(&self) -> &PathBuf {
        &self.data_local_dir
    }

    pub fn executable_dir(&self) -> Option<&PathBuf> {
        self.executable_dir.as_ref()
    }

    pub fn preference_dir(&self) -> &PathBuf {
        &self.preference_dir
    }

    pub fn runtime_dir(&self) -> Option<&PathBuf> {
        self.runtime_dir.as_ref()
    }

    pub fn state_dir(&self) -> Option<&PathBuf> {
        self.state_dir.as_ref()
    }
}

fn base_dirs() -> AreiaResult<BaseDirs> {
    match get_home() {
        Ok(home_dir) => {
            let cache_dir = cache_dir(home_dir.clone());
            let config_dir = config_dir(home_dir.clone());
            let config_local_dir = config_local_dir(home_dir.clone());
            let data_dir = data_dir(home_dir.clone());
            let data_local_dir = data_local_dir(home_dir.clone());
            let preference_dir = preference_dir(home_dir.clone());
            let runtime_dir = runtime_dir();
            let state_dir = state_dir(home_dir.clone());
            let executable_dir = executable_dir(home_dir.clone());
            Ok(BaseDirs {
                home_dir,
                cache_dir,
                config_dir,
                config_local_dir,
                data_dir,
                data_local_dir,
                executable_dir,
                preference_dir,
                runtime_dir,
                state_dir,
            })
        }
        Err(err) => Err(err),
    }
}
