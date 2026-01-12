use std::path::PathBuf;

use crate::{error::AreiaResult, utils::{get_home, factory::{cache_dir, config_dir, data_dir, executable_dir, runtime_dir, state_dir}}};

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
    state_dir: Option<PathBuf>
}

pub fn base_dirs() -> AreiaResult<BaseDirs> {
    match get_home() {
        Ok(home) => {
            let cache_dir = cache_dir(home.clone());
            let config_dir = config_dir(home.clone());
            let config_local_dir = config_dir.clone();
            let data_dir = data_dir(home.clone());
            let data_local_dir = data_dir.clone();
            let preference_dir = config_dir.clone();
            let runtime_dir = runtime_dir();
            let state_dir = state_dir(home.clone());
            let executable_dir = executable_dir(home.clone());
            Ok(BaseDirs {
                home_dir: home,
                cache_dir,
                config_dir,
                config_local_dir,
                data_dir,
                data_local_dir,
                executable_dir: Some(executable_dir),
                preference_dir,
                runtime_dir,
                state_dir: Some(state_dir)
            })
        },
        Err(err) => Err(err)
    }
}
