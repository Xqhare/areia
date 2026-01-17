use std::path::PathBuf;

use crate::{
    error::AreiaResult,
    utils::{factory::get_usr_dirs, get_home},
};

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
    pub fn new() -> AreiaResult<UserDirs> {
        user_dirs()
    }

    pub fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }

    pub fn audio_dir(&self) -> &Option<PathBuf> {
        &self.audio_dir
    }

    pub fn desktop_dir(&self) -> &Option<PathBuf> {
        &self.desktop_dir
    }

    pub fn document_dir(&self) -> &Option<PathBuf> {
        &self.document_dir
    }

    pub fn download_dir(&self) -> &Option<PathBuf> {
        &self.download_dir
    }

    pub fn font_dir(&self) -> &Option<PathBuf> {
        &self.font_dir
    }

    pub fn picture_dir(&self) -> &Option<PathBuf> {
        &self.picture_dir
    }

    pub fn public_dir(&self) -> &Option<PathBuf> {
        &self.public_dir
    }

    pub fn template_dir(&self) -> &Option<PathBuf> {
        &self.template_dir
    }

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
