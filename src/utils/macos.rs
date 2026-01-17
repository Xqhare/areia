use std::path::PathBuf;

use crate::{error::AreiaResult, Hidden};

use super::ffi::macos::{has_hidden_flag, set_hidden_flag, remove_hidden_flag};

/// Checks if any component of the path is hidden and has the hidden flag set.
pub fn is_superhidden(path: &PathBuf) -> AreiaResult<bool> {
    let mut new_path = PathBuf::new();
    for component in path.components() {
        new_path.push(component);
        if new_path.is_hidden()? {
            return has_hidden_flag(&new_path);
        }
    }
    return Ok(false);
}

pub fn superhide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    let hide = path.hide()?;
    set_hidden_flag(&hide)?;
    Ok(hide)
}

pub fn super_unhide(path: &mut PathBuf) -> AreiaResult<PathBuf> {
    let unhide = path.unhide()?;
    remove_hidden_flag(&unhide)?;
    Ok(unhide)
}
