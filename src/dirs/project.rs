use std::path::PathBuf;


pub struct ProjectDirs {
    home_dir: PathBuf,
    cache_dir: PathBuf,
    config_dir: PathBuf,
    config_local_dir: PathBuf,
    data_dir: PathBuf,
    data_local_dir: PathBuf,
    preference_dir: PathBuf,
    runtime_dir: Option<PathBuf>,
    state_dir: Option<PathBuf>
    
}
