use areia::BaseDirs;
use areia::Hidden;
use std::path::PathBuf;

#[test]
fn create_new_base() {
    let new_base = BaseDirs::new();
    assert!(new_base.is_ok());
}

#[test]
#[cfg(target_os = "linux")]
fn all_paths_present() {
    let base = BaseDirs::new();
    assert!(base.is_ok());
    let base = base.unwrap();

    assert!(base.home_dir().is_absolute());
    assert_ne!(base.home_dir(), &PathBuf::from(""));
    assert_ne!(base.home_dir(), &PathBuf::from("/"));
    assert!(base.cache_dir().is_absolute());
    assert_ne!(base.cache_dir(), &PathBuf::from(""));
    assert!(base.config_dir().is_absolute());
    assert_ne!(base.config_dir(), &PathBuf::from(""));
    assert!(base.config_local_dir().is_absolute());
    assert_ne!(base.config_local_dir(), &PathBuf::from(""));
    assert!(base.data_dir().is_absolute());
    assert_ne!(base.data_dir(), &PathBuf::from(""));
    assert!(base.data_local_dir().is_absolute());
    assert_ne!(base.data_local_dir(), &PathBuf::from(""));
    assert!(base.executable_dir().is_some());
    assert!(base.executable_dir().unwrap().is_absolute());
    assert_ne!(base.executable_dir().unwrap(), &PathBuf::from(""));
    assert!(base.preference_dir().is_absolute());
    assert_ne!(base.preference_dir(), &PathBuf::from(""));
    // Runtime may be set or not
    if base.runtime_dir().is_some() {
        assert!(base.runtime_dir().unwrap().is_absolute());
        assert_ne!(base.runtime_dir().unwrap(), &PathBuf::from(""));
    }
    assert!(base.state_dir().is_some());
    assert!(base.state_dir().unwrap().is_absolute());
    assert_ne!(base.state_dir().unwrap(), &PathBuf::from(""));
}

#[test]
#[cfg(target_os = "macos")]
fn all_paths_present() {
    let base = BaseDirs::new();
    assert!(base.is_ok());
    let base = base.unwrap();

    assert!(base.home_dir().is_absolute());
    assert_ne!(base.home_dir(), &PathBuf::from(""));
    assert_ne!(base.home_dir(), &PathBuf::from("/"));
    assert!(base.cache_dir().is_absolute());
    assert_ne!(base.cache_dir(), &PathBuf::from(""));
    assert!(base.config_dir().is_absolute());
    assert_ne!(base.config_dir(), &PathBuf::from(""));
    assert!(base.config_local_dir().is_absolute());
    assert_ne!(base.config_local_dir(), &PathBuf::from(""));
    assert!(base.data_dir().is_absolute());
    assert_ne!(base.data_dir(), &PathBuf::from(""));
    assert!(base.data_local_dir().is_absolute());
    assert_ne!(base.data_local_dir(), &PathBuf::from(""));
    assert!(base.executable_dir().is_none());
    assert!(base.preference_dir().is_absolute());
    assert_ne!(base.preference_dir(), &PathBuf::from(""));
    assert!(base.runtime_dir().is_none());
    assert!(base.state_dir().is_none());
}

#[test]
#[cfg(target_os = "windows")]
fn all_paths_present() {
    let base = BaseDirs::new();
    assert!(base.is_ok());
    let base = base.unwrap();

    assert!(base.home_dir().is_absolute());
    assert_ne!(base.home_dir(), &PathBuf::from(""));
    assert!(base.cache_dir().is_absolute());
    assert_ne!(base.cache_dir(), &PathBuf::from(""));
    assert!(base.config_dir().is_absolute());
    assert_ne!(base.config_dir(), &PathBuf::from(""));
    assert!(base.config_local_dir().is_absolute());
    assert_ne!(base.config_local_dir(), &PathBuf::from(""));
    assert!(base.data_dir().is_absolute());
    assert_ne!(base.data_dir(), &PathBuf::from(""));
    assert!(base.data_local_dir().is_absolute());
    assert_ne!(base.data_local_dir(), &PathBuf::from(""));
    assert!(base.executable_dir().is_none());
    assert!(base.preference_dir().is_absolute());
    assert_ne!(base.preference_dir(), &PathBuf::from(""));
    assert!(base.runtime_dir().is_none());
    assert!(base.state_dir().is_none());
}
