use std::ffi::{CString, OsString, c_void};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use crate::error::{AreiaError, AreiaResult};

type CFRef = *const c_void;
type CFURLRef = CFRef;
type CFStringRef = CFRef;

const UF_HIDDEN: u32 = 0x8000;

#[repr(C)]
struct Stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atime: i64,
    st_atime_nsec: i64,
    st_mtime: i64,
    st_mtime_nsec: i64,
    st_ctime: i64,
    st_ctime_nsec: i64,
    st_birthtime: i64,
    st_birthtime_nsec: i64,
    st_size: i64,
    st_blocks: i64,
    st_blksize: i32,
    pub st_flags: u32,
    st_gen: u32,
    st_lspare: i32,
    st_qspare: [i64; 2],
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFCopyHomeDirectoryURL() -> CFURLRef;
    fn CFURLCopyFileSystemPath(an_url: CFURLRef, pathstyle: i64) -> CFStringRef;
    fn CFStringGetLength(the_string: CFStringRef) -> isize;
    fn CFStringGetCString(
        the_string: CFStringRef,
        buffer: *mut u8,
        buffer_size: isize,
        encoding: u32,
    ) -> bool;
    fn CFRelease(the_object: CFRef);
}

#[link(name = "c")]
unsafe extern "C" {
    fn chflags(path: *const i8, flags: u32) -> i32;
    fn stat(path: *const i8, buf: *mut Stat) -> i32;
}

pub fn get_mac_home_fallback() -> Option<PathBuf> {
    unsafe {
        let url_ref = CFCopyHomeDirectoryURL();
        if url_ref.is_null() {
            return None;
        }
        let path_ref = CFURLCopyFileSystemPath(url_ref, 0);
        CFRelease(url_ref);

        if path_ref.is_null() {
            return None;
        }

        let path_len = CFStringGetLength(path_ref);
        let mut buf = vec![0u8; (path_len * 4) as usize + 1];
        let success =
            CFStringGetCString(path_ref, buf.as_mut_ptr(), buf.len() as isize, 0x08000100);
        CFRelease(path_ref);
        if success {
            let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            buf.truncate(end);
            Some(PathBuf::from(OsString::from_vec(buf)))
        } else {
            None
        }
    }
}

pub fn set_hidden_flag(path: &Path) -> AreiaResult<()> {
    set_flag(path, UF_HIDDEN, true)
}

pub fn remove_hidden_flag(path: &Path) -> AreiaResult<()> {
    set_flag(path, UF_HIDDEN, false)
}

pub fn has_hidden_flag(path: &Path) -> AreiaResult<bool> {
    Ok(get_flags(path)? & UF_HIDDEN != 0)
}

fn get_flags(path: &Path) -> AreiaResult<u32> {
    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| AreiaError::MacError(format!("Invalid path encoding: {}", e)))?;

    unsafe {
        let mut buf: Stat = std::mem::zeroed();
        let retr = stat(c_path.as_ptr(), &mut buf);

        if retr != 0 {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::MacError(format!(
                "Failed to stat file: {}",
                err
            )));
        }

        Ok(buf.st_flags)
    }
}

/// Sets the hidden attribute for a file or directory
///
/// If `enable` is true, the hidden attribute is set, otherwise it is removed - but only if it
/// makes sense to do so
fn set_flag(path: &Path, flag: u32, enable: bool) -> AreiaResult<()> {
    let cur_flags = get_flags(path)?;

    let new_flags = if enable {
        cur_flags | flag
    } else {
        cur_flags & !flag
    };

    if cur_flags == new_flags {
        return Ok(());
    }

    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| AreiaError::MacError(format!("Invalid path encoding: {}", e)))?;

    unsafe {
        let res = chflags(c_path.as_ptr(), new_flags);
        if res != 0 {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::MacError(format!(
                "Failed to set hidden attribute: {}",
                err
            )));
        }
    }
    Ok(())
}
