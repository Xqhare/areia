use areia::{Hidden, BaseDirs};
use std::path::PathBuf;
use std::fs::{remove_dir_all, remove_file, remove_dir};

#[test]
fn hide_non_existent_path() {
    let mut path = PathBuf::from("to_hide/some.file");
    let hidden_path = path.hide();
    if cfg!(target_os = "windows") {
        assert!(hidden_path.is_err());
    } else {
        assert!(hidden_path.is_ok());
        assert!(hidden_path.as_ref().unwrap().exists());
        assert!(hidden_path.as_ref().unwrap().is_file());
        assert!(hidden_path.as_ref().unwrap().is_hidden().unwrap());

        let unhidden_path = hidden_path.as_ref().unwrap().clone().unhide();
        assert!(unhidden_path.is_ok());
        assert!(unhidden_path.as_ref().unwrap().exists() && !hidden_path.as_ref().unwrap().exists());
        assert!(unhidden_path.as_ref().unwrap().is_file());
        assert!(!unhidden_path.as_ref().unwrap().is_hidden().unwrap());
        assert_eq!(unhidden_path.as_ref().unwrap(), &path);
        // Cleanup created files
        assert!(remove_file(unhidden_path.as_ref().unwrap()).is_ok());
        assert!(remove_dir_all(unhidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
    }
}

#[test]
fn hide_existing_path() {
    let mut path = PathBuf::from("real_dir/existing.file");
    assert!(std::fs::create_dir_all(&path.parent().unwrap()).is_ok());
    assert!(std::fs::File::create(&path).is_ok());

    let mut hidden_path = path.hide().unwrap();
    assert!(hidden_path.exists());
    assert!(hidden_path.is_file());
    assert!(hidden_path.is_hidden().unwrap());

    let unhidden_path = hidden_path.unhide().unwrap();
    assert!(unhidden_path.exists() && !hidden_path.exists());
    assert!(unhidden_path.is_file());
    assert!(!unhidden_path.is_hidden().unwrap());
    assert_eq!(unhidden_path, path);

    // cleanup created files
    assert!(remove_file(&unhidden_path).is_ok());
    assert!(remove_dir_all(&unhidden_path.parent().unwrap()).is_ok());
}

#[test]
fn hide_already_hidden_path() {
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = PathBuf::from(base_dirs.cache_dir());

    let mut path = PathBuf::from(cache_dir.join("hidden_dir/hidden.file"));
    let hidden_path = path.hide();
    assert!(hidden_path.is_ok());
    assert_eq!(hidden_path.as_ref().unwrap(), &path);
    assert!(hidden_path.as_ref().unwrap().exists());
    assert!(hidden_path.as_ref().unwrap().is_file());
    assert!(hidden_path.as_ref().unwrap().is_hidden().unwrap());

    let unhidden_path = hidden_path.as_ref().unwrap().clone().unhide();
    // Can't unhide a file inside a hidden system directory
    assert!(unhidden_path.is_err());

    // Cleanup created files
    assert!(remove_file(hidden_path.as_ref().unwrap()).is_ok());
    assert!(remove_dir(hidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
}

#[test]
fn into_hidden() {
    if cfg!(not(target_os = "windows")) {
        use areia::Hidden;
        use std::path::PathBuf;

        let mut path = PathBuf::from("non_existing/some.file");
        let hidden_path = path.into_hidden_path();
        assert!(hidden_path.is_ok());
        assert_eq!(hidden_path.as_ref().unwrap(), &PathBuf::from("non_existing/.some.file"));
        assert!(!hidden_path.as_ref().unwrap().exists());
        assert!(hidden_path.as_ref().unwrap().is_hidden().unwrap());
    } else {
        assert!(true);
    }
}
