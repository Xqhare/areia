use std::path::PathBuf;

use crate::{
    error::AreiaResult,
    utils::{factory::get_usr_dirs, get_home},
};

/// User directories
///
/// This struct contains all the user directories:
///
/// - `home_dir`
/// - `audio_dir`
/// - `desktop_dir`
/// - `document_dir`
/// - `download_dir`
/// - `font_dir`
/// - `picture_dir`
/// - `public_dir`
/// - `template_dir`
/// - `video_dir`
///
/// Create with `UserDirs::new()`.
/// Get paths by using the provided functions on `UserDirs`.
#[derive(Debug, Clone)]
pub struct UserDirs {
    home_dir: PathBuf,
    audio_dir: Option<PathBuf>,
    desktop_dir: Option<PathBuf>,
    document_dir: Option<PathBuf>,
    download_dir: Option<PathBuf>,
    font_dir: Option<PathBuf>,
    picture_dir: Option<PathBuf>,
    public_dir: Option<PathBuf>,
    template_dir: Option<PathBuf>,
    video_dir: Option<PathBuf>,
}

impl UserDirs {
    /// Create a new `UserDirs`.
    /// All paths will be constructed based on the current user.
    ///
    /// Returns an `AreiaResult` containing the `UserDirs`
    ///
    /// # Example
    ///
    /// ```
    /// use areia::UserDirs;
    ///
    /// let user = UserDirs::new();
    /// assert!(user.is_ok());
    /// let user = user.unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `AreiaError` if the home directory could not be found or an OS error occurred
    pub fn new() -> AreiaResult<UserDirs> {
        user_dirs()
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
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let home = user.home_dir();
    /// assert!(home.is_absolute());
    /// ```
    pub fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }

    /// Get the audio directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_MUSIC_DIR)` OR `None` | `Some({FolderID::Music})` | `Some($HOME/Music)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let audio = user.audio_dir();
    /// if audio.is_some() {
    ///     assert!(audio.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn audio_dir(&self) -> &Option<PathBuf> {
        &self.audio_dir
    }

    /// Get the desktop directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_DESKTOP_DIR)` OR `None` | `Some({FolderID::Desktop})` | `Some($HOME/Desktop)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let desktop = user.desktop_dir();
    /// if desktop.is_some() {
    ///     assert!(desktop.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn desktop_dir(&self) -> &Option<PathBuf> {
        &self.desktop_dir
    }

    /// Get the document directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_DOCUMENTS_DIR)` OR `None` | `Some({FolderID::Documents})` | `Some($HOME/Documents)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let document = user.document_dir();
    /// if document.is_some() {
    ///     assert!(document.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn document_dir(&self) -> &Option<PathBuf> {
        &self.document_dir
    }

    /// Get the download directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_DOWNLOAD_DIR)` OR `None` | `Some({FolderID::Downloads})` | `Some($HOME/Downloads)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let download = user.download_dir();
    /// if download.is_some() {
    ///     assert!(download.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn download_dir(&self) -> &Option<PathBuf> {
        &self.download_dir
    }

    /// Get the font directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_DATA_HOME/fonts)` OR `Some($HOME/.local/share/fonts)` | `None` | `Some($HOME/Library/Fonts)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let font = user.font_dir();
    /// if font.is_some() {
    ///     assert!(font.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn font_dir(&self) -> &Option<PathBuf> {
        &self.font_dir
    }

    /// Get the picture directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_PICTURES_DIR)` OR `None` | `Some({FolderID::Pictures})` | `Some($HOME/Pictures)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let picture = user.picture_dir();
    /// if picture.is_some() {
    ///     assert!(picture.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn picture_dir(&self) -> &Option<PathBuf> {
        &self.picture_dir
    }

    /// Get the public directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_PUBLICSHARE_DIR)` OR `None` | `Some({FolderID::Public})` | `Some($HOME/Public)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let public = user.public_dir();
    /// if public.is_some() {
    ///     assert!(public.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn public_dir(&self) -> &Option<PathBuf> {
        &self.public_dir
    }

    /// Get the template directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_TEMPLATES_DIR)` OR `None` | `Some({FolderID::Templates})` | `None` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let template = user.template_dir();
    /// if template.is_some() {
    ///     assert!(template.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn template_dir(&self) -> &Option<PathBuf> {
        &self.template_dir
    }

    /// Get the video directory
    ///
    /// | Linux | Windows | macOS |
    /// | --- | --- | --- |
    /// | `Some($XDG_VIDEOS_DIR)` OR `None` | `Some({FolderID::Videos})` | `Some($HOME/Movies)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use areia::UserDirs;
    /// let user = UserDirs::new().unwrap();
    /// let video = user.video_dir();
    /// if video.is_some() {
    ///     assert!(video.as_ref().unwrap().is_absolute());
    /// }
    /// ```
    pub fn video_dir(&self) -> &Option<PathBuf> {
        &self.video_dir
    }
}

fn user_dirs() -> AreiaResult<UserDirs> {
    let home_dir = get_home()?;

    let usr_dirs = get_usr_dirs(home_dir.clone())?;
    let audio_dir = usr_dirs
        .get("MUSIC")
        .expect("Key is always created by factory")
        .clone();
    let desktop_dir = usr_dirs
        .get("DESKTOP")
        .expect("Key is always created by factory")
        .clone();
    let document_dir = usr_dirs
        .get("DOCUMENTS")
        .expect("Key is always created by factory")
        .clone();
    let download_dir = usr_dirs
        .get("DOWNLOADS")
        .expect("Key is always created by factory")
        .clone();
    let font_dir = usr_dirs
        .get("FONTS")
        .expect("Key is always created by factory")
        .clone();
    let picture_dir = usr_dirs
        .get("PICTURES")
        .expect("Key is always created by factory")
        .clone();
    let public_dir = usr_dirs
        .get("PUBLICSHARE")
        .expect("Key is always created by factory")
        .clone();
    let template_dir = usr_dirs
        .get("TEMPLATES")
        .expect("Key is always created by factory")
        .clone();
    let video_dir = usr_dirs
        .get("VIDEOS")
        .expect("Key is always created by factory")
        .clone();

    Ok(UserDirs {
        home_dir,
        audio_dir,
        desktop_dir,
        document_dir,
        download_dir,
        font_dir,
        picture_dir,
        public_dir,
        template_dir,
        video_dir,
    })
}
