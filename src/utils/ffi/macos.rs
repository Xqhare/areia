use std::ffi::{c_void, OsString};
use std::path::PathBuf;
use std::os::unix::ffi::OsStringExt;

type CFRef = *const c_void;
type CFURLRef = CFRef;
type CFStringRef = CFRef;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFCopyHomeDirectoryURL() -> CFURLRef;
    fn CFURLCopyFileSystemPath(an_url: CFURLRef, pathstyle: i64) -> CFStringRef;
    fn CFStringGetLength(the_string: CFStringRef) -> isize;
    fn CFStringGetCString(
        the_string: CFStringRef, 
        buffer: *mut u8, 
        buffer_size: isize, 
        encoding: u32
    ) -> bool;
    fn CFRelease(the_object: CFRef);
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
        let success = CFStringGetCString(path_ref, buf.as_mut_ptr(), buf.len() as isize, 0x08000100);
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
