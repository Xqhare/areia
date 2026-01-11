use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct UserDirs {
    home_dir: PathBuf,
    audio_dir: Option<PathBuf>,
    desktop_dir: Option<PathBuf>,
    document_dir: Option<PathBuf>,
    font_dir: Option<PathBuf>,
    picture_dir: Option<PathBuf>,
    public_dir: Option<PathBuf>,
    template_dir: Option<PathBuf>,
    video_dir: Option<PathBuf>
}

