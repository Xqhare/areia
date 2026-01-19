#![cfg(not(target_os = "linux"))]

use areia::{SuperHidden, BaseDirs};
use std::path::PathBuf;
use std::fs::{remove_dir_all, remove_file, remove_dir};

#[test]
fn super_hide_non_existent_path() {
    use areia::SuperHidden;
    use std::path::PathBuf;

    let mut path = PathBuf::from("super_hide/some.file");
    let super_hidden_path = path.super_hide();
    assert!(super_hidden_path.is_ok());
    assert!(super_hidden_path.as_ref().unwrap().exists());
    assert!(super_hidden_path.as_ref().unwrap().is_file());
    assert!(super_hidden_path.as_ref().unwrap().is_super_hidden().unwrap());

    let unhidden_path = super_hidden_path.as_ref().unwrap().clone().super_unhide();
    assert!(unhidden_path.is_ok());
    assert!(unhidden_path.as_ref().unwrap().exists() && !super_hidden_path.as_ref().unwrap().exists());
    assert!(unhidden_path.as_ref().unwrap().is_file());
    assert!(!unhidden_path.as_ref().unwrap().is_super_hidden().unwrap());
    assert_eq!(unhidden_path.as_ref().unwrap(), &path);
    // Cleanup created files
    assert!(std::fs::remove_file(unhidden_path.as_ref().unwrap()).is_ok());
    assert!(std::fs::remove_dir_all(unhidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
}

#[test]
fn super_hide_existing_path() { 
    use areia::SuperHidden;
    use std::path::PathBuf;

    let mut path = PathBuf::from("any_dir/existing.file");
    assert!(std::fs::create_dir_all(&path.parent().unwrap()).is_ok());
    assert!(std::fs::File::create(&path).is_ok());

    let mut super_hidden_path = path.super_hide().unwrap();
    assert!(super_hidden_path.exists());
    assert!(super_hidden_path.is_file());
    assert!(super_hidden_path.is_super_hidden().unwrap());

    let unhidden_path = super_hidden_path.super_unhide().unwrap();
    assert!(unhidden_path.exists() && !super_hidden_path.exists());
    assert!(unhidden_path.is_file());
    assert!(!unhidden_path.is_super_hidden().unwrap());
    assert_eq!(unhidden_path, path);

    // cleanup created files
    assert!(std::fs::remove_file(&unhidden_path).is_ok());
    assert!(std::fs::remove_dir_all(&unhidden_path.parent().unwrap()).is_ok());
}

#[test]
fn super_hide_already_hidden_path() {
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = PathBuf::from(base_dirs.cache_dir());

    let mut path = PathBuf::from(cache_dir.join("hidden_dir/hidden.file"));
    let super_hidden_path = path.super_hide();
    assert!(super_hidden_path.is_ok());
    assert_eq!(super_hidden_path.as_ref().unwrap(), &path);
    assert!(super_hidden_path.as_ref().unwrap().exists());
    assert!(super_hidden_path.as_ref().unwrap().is_file());
    assert!(super_hidden_path.as_ref().unwrap().is_super_hidden().unwrap());

    let unhidden_path = super_hidden_path.as_ref().unwrap().clone().super_unhide();
    // Can't unhide a file inside a hidden system directory
    assert!(unhidden_path.is_err());

    // Cleanup created files
    assert!(std::fs::remove_file(super_hidden_path.as_ref().unwrap()).is_ok());
    assert!(std::fs::remove_dir(super_hidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
}
