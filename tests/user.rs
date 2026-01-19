use areia::UserDirs;

#[test]
fn create_new_user() {
    let new_user = UserDirs::new();
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
        assert!(user.audio_dir().as_ref().unwrap().is_dir());
    }
    if user.desktop_dir().is_some() {
        assert!(user.desktop_dir().as_ref().unwrap().is_absolute());
        assert!(user.desktop_dir().as_ref().unwrap().is_dir());
    }
    if user.document_dir().is_some() {
        assert!(user.document_dir().as_ref().unwrap().is_absolute());
        assert!(user.document_dir().as_ref().unwrap().is_dir());
    }
    if user.download_dir().is_some() {
        assert!(user.download_dir().as_ref().unwrap().is_absolute());
        assert!(user.download_dir().as_ref().unwrap().is_dir());
    }
    assert!(user.font_dir().is_some());
    assert!(user.font_dir().as_ref().unwrap().is_absolute());
    assert!(user.font_dir().as_ref().unwrap().is_dir());
    if user.picture_dir().is_some() {
        assert!(user.picture_dir().as_ref().unwrap().is_absolute());
        assert!(user.picture_dir().as_ref().unwrap().is_dir());
    }
    if user.public_dir().is_some() {
        assert!(user.public_dir().as_ref().unwrap().is_absolute());
        assert!(user.public_dir().as_ref().unwrap().is_dir());
    }
    if user.template_dir().is_some() {
        assert!(user.template_dir().as_ref().unwrap().is_absolute());
        assert!(user.template_dir().as_ref().unwrap().is_dir());
    }
    if user.video_dir().is_some() {
        assert!(user.video_dir().as_ref().unwrap().is_absolute());
        assert!(user.video_dir().as_ref().unwrap().is_dir());
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
    assert!(user.audio_dir().as_ref().unwrap().is_dir());
    assert!(user.desktop_dir().is_some());
    assert!(user.desktop_dir().as_ref().unwrap().is_dir());
    assert!(user.document_dir().is_some());
    assert!(user.document_dir().as_ref().unwrap().is_dir());
    assert!(user.download_dir().is_some());
    assert!(user.download_dir().as_ref().unwrap().is_dir());
    assert!(user.font_dir().is_some());
    assert!(user.font_dir().as_ref().unwrap().is_dir());
    assert!(user.picture_dir().is_some());
    assert!(user.picture_dir().as_ref().unwrap().is_dir());
    assert!(user.public_dir().is_some());
    assert!(user.public_dir().as_ref().unwrap().is_dir());
    assert!(user.template_dir().is_none());
    assert!(user.video_dir().is_some());
    assert!(user.video_dir().as_ref().unwrap().is_dir());
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
    assert!(user.audio_dir().as_ref().unwrap().is_dir());
    assert!(user.desktop_dir().is_some());
    assert!(user.desktop_dir().as_ref().unwrap().is_dir());
    assert!(user.document_dir().is_some());
    assert!(user.document_dir().as_ref().unwrap().is_dir());
    assert!(user.download_dir().is_some());
    assert!(user.download_dir().as_ref().unwrap().is_dir());
    assert!(user.font_dir().is_none());
    assert!(user.picture_dir().is_some());
    assert!(user.picture_dir().as_ref().unwrap().is_dir());
    assert!(user.public_dir().is_some());
    assert!(user.public_dir().as_ref().unwrap().is_dir());
    assert!(user.template_dir().is_none());
    assert!(user.video_dir().is_some());
    assert!(user.video_dir().as_ref().unwrap().is_dir());
}

