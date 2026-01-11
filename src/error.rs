
pub type AreiaResult<T> = Result<T, AreiaError>;

type WinErrString = String;

#[derive(Debug)]
pub enum AreiaError {
    CantGetHomeDir,
    IoError(std::io::Error),
    WindowsError(WinErrString),
}

impl std::fmt::Display for AreiaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AreiaError::CantGetHomeDir => write!(f, "Can't get home directory"),
            AreiaError::IoError(err) => write!(f, "IO error: {}", err),
            AreiaError::WindowsError(err) => write!(f, "Windows error. \n Unable to get: {}", err),
        }
    }
}
