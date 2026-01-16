use std::path::PathBuf;

use crate::error::{AreiaError, AreiaResult};

use super::SuperHidden;


impl SuperHidden for PathBuf {
    fn is_super_hidden(&self) -> AreiaResult<bool> {
        if cfg!(target_os = "linux") {
            Err(AreiaError::SuperHidingNotSupported("No super hiding on Linux".to_string()))
        } else {
            if !self.exists() {
                return Err(AreiaError::SuperHidingRequiresExistingPath(self.clone()));
            }
            todo!()
        }
    }

    fn super_hide(&mut self) -> AreiaResult<PathBuf> {
        if cfg!(target_os = "linux") {
            Err(AreiaError::SuperHidingNotSupported("No super hiding on Linux".to_string()))
        } else {
            if !self.exists() {
                return Err(AreiaError::SuperHidingRequiresExistingPath(self.clone()));
            }
            if self.is_super_hidden()? {
                return Ok(self.clone());
            }
            todo!()
        }
    }

    fn super_unhide(&mut self) -> AreiaResult<PathBuf> {
        if cfg!(target_os = "linux") {
            Err(AreiaError::SuperHidingNotSupported("No super hiding on Linux".to_string()))
        } else {
            if !self.exists() {
                return Err(AreiaError::SuperHidingRequiresExistingPath(self.clone()));
            }
            if !self.is_super_hidden()? {
                return Ok(self.clone());
            }
            todo!()
        }
    }
}
