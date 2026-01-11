use std::path::PathBuf;

use crate::{error::AreiaResult, utils};

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

#[cfg(target_os = "linux")]
pub fn base_dirs() -> AreiaResult<BaseDirs> {

    match utils::get_home() {
        Ok(home) => {
            let cache_dir = ca
            
        },
        Err(err) => Err(err)
    }
}
