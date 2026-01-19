use areia::BaseDirs;

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
    assert!(base.home_dir().is_dir());
    assert!(base.cache_dir().is_absolute());
    assert!(base.cache_dir().is_dir());
    assert!(base.config_dir().is_absolute());
    assert!(base.config_dir().is_dir());
    assert!(base.config_local_dir().is_absolute());
    assert!(base.config_local_dir().is_dir());
    assert!(base.data_dir().is_absolute());
    assert!(base.data_dir().is_dir());
    assert!(base.data_local_dir().is_absolute());
    assert!(base.data_local_dir().is_dir());
    assert!(base.executable_dir().is_some());
    assert!(base.executable_dir().unwrap().is_absolute());
    assert!(base.executable_dir().unwrap().is_dir());
    assert!(base.preference_dir().is_absolute());
    assert!(base.preference_dir().is_dir());
    // Runtime may be set or not
    if base.runtime_dir().is_some() {
        assert!(base.runtime_dir().unwrap().is_absolute());
        assert!(base.runtime_dir().unwrap().is_dir());
    }
    assert!(base.state_dir().is_some());
    assert!(base.state_dir().unwrap().is_absolute());
    assert!(base.state_dir().unwrap().is_dir());
}

#[test]
#[cfg(target_os = "macos")]
fn all_paths_present() {
    let base = BaseDirs::new();
    assert!(base.is_ok());
    let base = base.unwrap();

    assert!(base.home_dir().is_absolute());
    assert!(base.home_dir().is_dir());
    assert!(base.cache_dir().is_absolute());
    assert!(base.cache_dir().is_dir());
    assert!(base.config_dir().is_absolute());
    assert!(base.config_dir().is_dir());
    assert!(base.config_local_dir().is_absolute());
    assert!(base.config_local_dir().is_dir());
    assert!(base.data_dir().is_absolute());
    assert!(base.data_dir().is_dir());
    assert!(base.data_local_dir().is_absolute());
    assert!(base.data_local_dir().is_dir());
    assert!(base.executable_dir().is_none());
    assert!(base.preference_dir().is_absolute());
    assert!(base.preference_dir().is_dir());
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
    assert!(base.home_dir().is_dir());
    assert!(base.cache_dir().is_absolute());
    assert!(base.cache_dir().is_dir());
    assert!(base.config_dir().is_absolute());
    assert!(base.config_dir().is_dir());
    assert!(base.config_local_dir().is_absolute());
    assert!(base.config_local_dir().is_dir());
    assert!(base.data_dir().is_absolute());
    assert!(base.data_dir().is_dir());
    assert!(base.data_local_dir().is_absolute());
    assert!(base.data_local_dir().is_dir());
    assert!(base.executable_dir().is_none());
    assert!(base.preference_dir().is_absolute());
    assert!(base.preference_dir().is_dir());
    assert!(base.runtime_dir().is_none());
    assert!(base.state_dir().is_none());
}
