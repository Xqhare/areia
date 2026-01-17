use areia::UserDirs;
use std::path::PathBuf;

#[test]
fn create_new_user() {
    let new_user = UserDirs::new();
    if new_user.is_err() {
        println!("{:?}", new_user);
    }
    assert!(new_user.is_ok());
}

#[test]
#[cfg(target_os = "linux")]
fn all_paths_present() {
    let user = UserDirs::new();
    assert!(user.is_ok());
    let user = user.unwrap();

    assert!(user.home_dir().is_absolute());
    // All linux user directories may not exist, with the exception of `Home`, `Fonts`
    if user.audio_dir().is_some() {
        assert!(user.audio_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.audio_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    if user.desktop_dir().is_some() {
        assert!(user.desktop_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.desktop_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    if user.document_dir().is_some() {
        assert!(user.document_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.document_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    if user.download_dir().is_some() {
        assert!(user.download_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.download_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    assert!(user.font_dir().is_some());
    assert!(user.font_dir().as_ref().unwrap().is_absolute());
    assert_ne!(user.font_dir().as_ref().unwrap(), &PathBuf::from(""));
    if user.picture_dir().is_some() {
        assert!(user.picture_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.picture_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    if user.public_dir().is_some() {
        assert!(user.public_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.public_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    if user.template_dir().is_some() {
        assert!(user.template_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.template_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
    if user.video_dir().is_some() {
        assert!(user.video_dir().as_ref().unwrap().is_absolute());
        assert_ne!(user.video_dir().as_ref().unwrap(), &PathBuf::from(""));
    }
}

#[test]
#[cfg(target_os = "macos")]
fn all_paths_present() {
    let user = UserDirs::new();
    assert!(user.is_ok());
    let user = user.unwrap();

    assert!(user.home_dir().is_absolute());
    // In contrast to linux, only templates will never exists, all others always will
    assert!(user.audio_dir().is_some());
    assert_ne!(user.audio_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.desktop_dir().is_some());
    assert_ne!(user.desktop_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.document_dir().is_some());
    assert_ne!(user.document_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.download_dir().is_some());
    assert_ne!(user.download_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.font_dir().is_some());
    assert_ne!(user.font_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.picture_dir().is_some());
    assert_ne!(user.picture_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.public_dir().is_some());
    assert_ne!(user.public_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.template_dir().is_none());
    assert!(user.video_dir().is_some());
    assert_ne!(user.video_dir().as_ref().unwrap(), &PathBuf::from(""));
}

#[test]
#[cfg(target_os = "windows")]
fn all_paths_present() {
    let user = UserDirs::new();
    assert!(user.is_ok());
    let user = user.unwrap();

    assert!(user.home_dir().is_absolute());
    // In contrast to mac, only fonts will never exists, all others always will
    assert!(user.audio_dir().is_some());
    assert_ne!(user.audio_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.desktop_dir().is_some());
    assert_ne!(user.desktop_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.document_dir().is_some());
    assert_ne!(user.document_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.download_dir().is_some());
    assert_ne!(user.download_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.font_dir().is_none());
    assert!(user.picture_dir().is_some());
    assert_ne!(user.picture_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.public_dir().is_some());
    assert_ne!(user.public_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.template_dir().is_some());
    assert_ne!(user.template_dir().as_ref().unwrap(), &PathBuf::from(""));
    assert!(user.video_dir().is_some());
    assert_ne!(user.video_dir().as_ref().unwrap(), &PathBuf::from(""));
}
