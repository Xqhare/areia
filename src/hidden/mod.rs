mod normal_hide;
mod super_hide;
use std::path::PathBuf;

use crate::error::AreiaResult;

pub trait Hidden {
    /// Returns true if the path is hidden
    /// 
    /// # Platform specific behaviour
    ///
    /// ## Unix
    ///
    /// Checks if any component of the path is hidden (`.` prefix).
    /// Does not check if path exists.
    /// Returns `true` if any component is hidden.
    /// Returns `false` if any component is not hidden.
    ///
    /// ## Windows
    ///
    /// Checks for the hidden attribute on the file or folder pointed to with the path.
    /// Returns `true` if the file or folder is hidden.
    /// Returns `false` if the file or folder is not hidden.
    /// SPECIAL: Returns `false` if the path does not exist or an error occurs.
    fn is_hidden(&self) -> bool;
    /// Hides the path
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// Checks if any component of the path is already hidden.
    /// If true, returns the path unmodified and does nothing.
    /// If no part of the path is hidden, the last directory or file contained in the path is hidden.
    /// 
    /// The returned `PathBuf` is the new hidden path.
    /// If the path exists, the last directory or file in the path is moved to be hidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// If the path does not exist, it is created.
    /// 
    /// ## Windows
    /// 
    /// Checks for the hidden attribute on the file or folder pointed to with the path.
    /// If the file or folder is already hidden, it does nothing.
    /// If the file or folder is not hidden, the file or folder is hidden.
    /// 
    /// The returned `PathBuf` is the provided path.
    /// If the path exists, the file or folder is moved to be hidden.
    /// If the path does not exist, it is created.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// 
    /// # Errors
    ///
    /// Errors if the program has insufficient permissions to move the file or folder.
    fn hide(&mut self) -> AreiaResult<PathBuf>;
    /// Unhides a hidden path
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// Checks if any component of the path is hidden.
    /// If true, the last directory or file contained in the path is unhidden and the new path is returned.
    /// Should the file be located in a hidden system path, the path is not unhidden and an error is returned.
    ///
    /// If no part of the path is hidden, returns the path unmodified and does nothing.
    /// 
    /// The returned `PathBuf` is the new unhidden path.
    /// If the path exists, the last directory or file in the path is moved to be unhidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// If the path does not exist, the new hidden path is returned.
    /// 
    /// ## Windows
    /// 
    /// Checks for the hidden attribute on the file or folder pointed to with the path.
    /// If the file or folder is already visible, it does nothing.
    /// If the file or folder is hidden, the file or folder is unhidden.
    /// 
    /// The returned `PathBuf` is the provided path.
    /// If the path exists, the file or folder is moved to be unhidden.
    /// This is done using 'atomic move' - ensuring no data is lost.
    /// If the path does not exist, an error is returned.
    /// 
    /// # Errors
    ///
    /// Errors if the program has insufficient permissions to move the file or folder.

    fn unhide(&mut self) -> AreiaResult<PathBuf>;
}

pub trait SuperHidden {
    /// Checks if any component of the path is `super hidden`.
    /// The path must point to an existing file or directory.
    /// This concept only applies to MacOS and Windows.
    ///
    /// `super hidden` is a file or directory that is hidden, as achieved with using `.hide()` provided by `areia`, and marked with a platform specific attribute or flag.
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// ### Linux
    ///
    /// Always returns `AreiaError::SuperHidingNotSupported`.
    ///
    /// ### MacOS
    ///
    /// Returns `true` if any component of the path is hidden (`.` prefix) and has the `hidden` flag set.
    ///
    /// ## Windows
    ///
    /// Only the file or folder pointed to by the path is checked.
    /// Returns `true` if the attributes `Hidden` and `System` are set.
    ///
    /// # Errors
    /// 
    /// Errors if the path does not exist or an OS error occurs.
    fn is_super_hidden(&self) -> AreiaResult<bool>;
    /// Super hides the file or directory pointed to by the path.
    /// The path must point to an existing file or directory.
    /// This concept only applies to MacOS and Windows.
    ///
    /// `super hidden` is a file or directory that is hidden, as achieved with using `.hide()` provided by `areia`, and marked with a platform specific attribute or flag.
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// ### Linux
    ///
    /// Always returns `AreiaError::SuperHidingNotSupported`.
    ///
    /// ### MacOS
    ///
    /// Does nothing if the file or directory is already super hidden.
    /// Otherwise, the file or directory is hidden and marked with the `hidden` flag.
    ///
    /// ## Windows
    ///
    /// Does nothing if the file or directory is already super hidden.
    /// Otherwise, the file or directory is hidden and marked with the `Hidden` and `System` attributes.
    ///
    /// # Errors
    /// 
    /// Errors if the path does not exist or an OS error occurs.
    fn super_hide(&mut self) -> AreiaResult<PathBuf>;
    /// Super un-hides the file or directory pointed to by the path.
    /// The reverse of `.super_hide()`. 
    /// The path must point to an existing file or directory. 
    /// This concept only applies to MacOS and Windows.
    ///
    /// `super hidden` is a file or directory that is hidden, as achieved with using `.hide()` provided by `areia`, and marked with a platform specific attribute or flag.
    ///
    /// # Platform specific behaviour
    /// 
    /// ## Unix
    ///
    /// ### Linux
    ///
    /// Always returns `AreiaError::SuperHidingNotSupported`.
    ///
    /// ### MacOS
    ///
    /// Does nothing if the file or directory is not super hidden.
    /// Otherwise, the file or directory is unhidden and unmarked with the `hidden` flag.
    ///
    /// ## Windows
    ///
    /// Does nothing if the file or directory is not super hidden.
    /// Otherwise, the file or directory is unhidden and unmarked with the `Hidden` and `System` attributes.
    ///
    /// # Errors
    /// 
    /// Errors if the path does not exist or an OS error occurs.
    fn super_unhide(&mut self) -> AreiaResult<PathBuf>;
}
