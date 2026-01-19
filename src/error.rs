use std::path::PathBuf;

pub type AreiaResult<T> = Result<T, AreiaError>;

type WinErrString = String;
type MacErrString = String;
type Reason = String;

#[derive(Debug)]
pub enum AreiaError {
    PathMustBeAbsolute(PathBuf),
    PathMustBeSomething(PathBuf),
    FileDoesNotExist(PathBuf),
    CantGetHomeDir,
    MakingHiddenPathNotSupported(Reason),
    SuperHidingNotSupported(Reason),
    SuperHidingRequiresExistingPath(PathBuf),
    IoError(std::io::Error),
    WindowsError(WinErrString),
    WindowsIoError(std::io::Error),
    WindowsErrorPathDoesNotExist(PathBuf),
    MacError(MacErrString),
    HiddenFileInsideSystemDir(PathBuf),
}

impl From<std::io::Error> for AreiaError {
    fn from(err: std::io::Error) -> Self {
        AreiaError::IoError(err)
    }
}

impl std::fmt::Display for AreiaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AreiaError::PathMustBeAbsolute(path) => {
                write!(f, "Path must be absolute: {:?}", path)
            }
            AreiaError::CantGetHomeDir => write!(f, "Can't get home directory"),
            AreiaError::FileDoesNotExist(path) => write!(f, "File does not exist: {:?}", path),
            AreiaError::PathMustBeSomething(path) => {
                write!(f, "Path must be something: {:?}", path)
            }
            AreiaError::MakingHiddenPathNotSupported(reason) => {
                write!(f, "Making hidden path not supported. Reason: {}", reason)
            }
            AreiaError::SuperHidingNotSupported(reason) => {
                write!(f, "Super hiding not supported. Reason: {}", reason)
            }
            AreiaError::SuperHidingRequiresExistingPath(path) => write!(
                f,
                "Super hiding requires an existing path (either file or directory): {:?}",
                path
            ),
            AreiaError::IoError(err) => write!(f, "IO error: {}", err),
            AreiaError::WindowsError(err) => write!(f, "Windows error. \n Unable to get: {}", err),
            AreiaError::WindowsIoError(err) => write!(f, "Windows IO error: {}", err),
            AreiaError::WindowsErrorPathDoesNotExist(path) => {
                write!(f, "Windows Error: Path does not exist: {:?}", path)
            }
            AreiaError::MacError(err) => write!(f, "Mac error. \n Unable to get: {}", err),
            AreiaError::HiddenFileInsideSystemDir(path) => {
                write!(f, "Hidden file inside system directory: {:?}", path)
            }
        }
    }
}
