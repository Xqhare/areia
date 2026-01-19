
use std::path::PathBuf;
use crate::error::AreiaResult;
use crate::utils::{create_all_dir_with_file, delete_all_dir_with_files};

/// Convenience function to create a directory with a file in it.
/// All parent directories are created if they do not exist.
///
/// # Example
///
/// ```
/// # use areia::auto_deletor;
/// use areia::auto_creator;
/// use std::path::PathBuf;
/// let path = PathBuf::from("test_dir/test_file.txt");
/// assert!(auto_creator(&path).is_ok());
/// assert!(&path.exists());
/// # assert!(auto_deletor(&path).is_ok());
/// ```
///
/// # Errors
///
/// Errors if the directory cannot be created and returns the OS error.
pub fn auto_creator<P: Into<PathBuf>>(path: P) -> AreiaResult<()> {
    let path = path.into();
    create_all_dir_with_file(&path)
}

/// Convenience function to delete a directory with files in it.
///
/// ! USE WITH CAUTION !
///
/// All parent directories and any contained files are deleted if they exist.
/// If only a file is passed in, all files and directories inside the same directory are deleted.
///
/// # Example
///
/// Given this structure:
///
/// parent/
/// ├── child/
/// │   ├── a.file
/// │   └── b.file
/// ├── child2/
/// │   └── c.file
/// ├── another.file
/// └── my.file
///
/// If `auto_deletor(child/)` (or `auto_deletor(child/a.file)`) is called, `a.file`, `b.file` and it's parent directory `child` are
/// deleted, resulting in:
///
/// parent/
/// ├── child2/
/// │   └── c.file
/// ├── another.file
/// └── my.file
///
/// By calling `auto_deletor(parent/my.file)` all files and directories in `parent/` are deleted
/// alongisde `parent/` itself.
///
/// # Usage Example
///
/// ```
/// use areia::{auto_creator, auto_deletor};
/// use std::path::PathBuf;
/// let path = PathBuf::from("testing_dir/test_file.txt");
/// assert!(auto_creator(&path).is_ok());
/// assert!(&path.exists());
/// assert!(auto_deletor(&path).is_ok());
/// assert!(!&path.exists());
/// ```
///
/// # Errors
///
/// Errors if the directory cannot be deleted (e.g. does not exist) and returns the OS error.
pub fn auto_deletor<P: Into<PathBuf>>(path: P) -> AreiaResult<()> {
    let path = path.into();
    delete_all_dir_with_files(&path)
}
