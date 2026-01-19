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

/// Base directories
///
/// This struct contains all the base directories:
///
/// - `home_dir`
/// - `cache_dir`
/// - `config_dir`
/// - `config_local_dir`
/// - `data_dir`
/// - `data_local_dir`
/// - `executable_dir` (If available)
/// - `preference_dir`
/// - `runtime_dir` (If available)
/// - `state_dir` (If available)
///
/// Create with `BaseDirs::new()`.
/// Get paths by using the provided functions on `BaseDirs`.
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
    /// Create a new `BaseDirs`.
    /// All paths will be constructed based on the current user.
    ///
    /// Returns an `AreiaResult` containing the `BaseDirs`
    ///
    /// # Example
    ///
    /// ```
    /// use areia::BaseDirs;
    ///
    /// let base = BaseDirs::new();
    /// assert!(base.is_ok());
    /// let base = base.unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `AreiaError` if the home directory could not be found or an OS error occurred
    pub fn new() -> AreiaResult<BaseDirs> {
        base_dirs()
    }

    /// Get the home directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$HOME` | `{FolderID::Profile}` | `$HOME` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let home = base.home_dir();
    /// assert!(home.is_absolute());
    /// ```
    pub fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }

    /// Get the cache directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$XDG_CACHE_HOME` OR `$HOME/.cache` | `{FolderID::LocalAppData}` | `$HOME/Library/Caches` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let cache = base.cache_dir();
    /// assert!(cache.is_absolute());
    /// ```
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Get the config directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$XDG_CONFIG_HOME` OR `$HOME/.config` | `{FolderID::RoamingAppData}` | `$HOME/Library/Application Support` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let config = base.config_dir();
    /// assert!(config.is_absolute());
    /// ```
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    /// Get the config local directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$XDG_CONFIG_HOME` OR `$HOME/.config` | `{FolderID::RoamingAppData}` | `$HOME/Library/Application Support` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let config = base.config_local_dir();
    /// assert!(config.is_absolute());
    /// ```
    pub fn config_local_dir(&self) -> &PathBuf {
        &self.config_local_dir
    }

    /// Get the data directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$XDG_DATA_HOME` OR `$HOME/.local/share` | `{FolderID::RoamingAppData}` | `$HOME/Library/Application Support` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let data = base.data_dir();
    /// assert!(data.is_absolute());
    /// ```
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Get the data local directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$XDG_DATA_HOME` OR `$HOME/.local/share` | `{FolderID::LocalAppData}` | `$HOME/Library/Application Support` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let data = base.data_local_dir();
    /// assert!(data.is_absolute());
    /// ```
    pub fn data_local_dir(&self) -> &PathBuf {
        &self.data_local_dir
    }

    /// Get the executable directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_BIN_HOME)` OR `Some($HOME/.local/bin)` | `None` | `None` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let executable = base.executable_dir();
    /// if cfg!(target_os = "linux") {
    ///     assert!(executable.is_some());
    ///     assert!(executable.unwrap().is_absolute());
    /// } else {
    ///     assert!(executable.is_none());
    ///     }
    /// ```
    pub fn executable_dir(&self) -> Option<&PathBuf> {
        self.executable_dir.as_ref()
    }

    /// Get the preference directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `$XDG_CONFIG_HOME` OR `$HOME/.config` | `{FolderID::RoamingAppData}` | `$HOME/Library/Preferences` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let preference = base.preference_dir();
    /// assert!(preference.is_absolute());
    /// ```
    pub fn preference_dir(&self) -> &PathBuf {
        &self.preference_dir
    }

    /// Get the runtime directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_RUNTIME_DIR)` OR `None` | `None` | `None` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let runtime = base.runtime_dir();
    /// if cfg!(target_os = "linux") {
    ///     if runtime.is_some() {
    ///         assert!(runtime.unwrap().is_absolute());
    ///     }
    /// } else {
    ///     assert!(runtime.is_none());
    ///     }
    /// ```
    pub fn runtime_dir(&self) -> Option<&PathBuf> {
        self.runtime_dir.as_ref()
    }

    /// Get the state directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_STATE_HOME)` OR `Some($HOME/.local/state)` | `None` | `None` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::BaseDirs;
    /// let base = BaseDirs::new().unwrap();
    /// let state = base.state_dir();
    /// if cfg!(target_os = "linux") {
    ///     assert!(state.is_some());
    ///     assert!(state.unwrap().is_absolute());
    /// } else {
    ///     assert!(state.is_none());
    ///     }
    /// ```
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
