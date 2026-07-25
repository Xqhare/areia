use std::ffi::{CStr, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

#[repr(C)]
struct Passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: u32,
    pub pw_gid: u32,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}

#[link(name = "c")]
unsafe extern "C" {
    fn getuid() -> u32;
    fn getpwuid_r(
        uid: u32,
        pwd: *mut Passwd,
        buf: *mut i8,
        buflen: usize,
        result: *mut *mut Passwd,
    ) -> i32;
    fn getcwd(buf: *mut i8, size: usize) -> *mut i8;
}

pub fn get_unix_working_dir_fallback() -> Option<PathBuf> {
    unsafe {
        // Way to large a buffer, but fallback of a fallback
        let mut buf = vec![0i8; 8192];
        let ret = getcwd(buf.as_mut_ptr(), buf.len());
        if ret.is_null() {
            // Could call to get the error here

            // To make this redundant, lets have rusts stdlib do the literal same call
            // again!
            // Why not use it in the first place? Where is the fun in that?
            if let Ok(path) = std::env::current_dir() {
                Some(path)
            } else {
                None
            }
        } else {
            Some(PathBuf::from(OsString::from_vec(
                CStr::from_ptr(ret).to_bytes().to_vec(),
            )))
        }
    }
}

pub fn get_unix_home_fallback() -> Option<PathBuf> {
    unsafe {
        let uid = getuid();
        let mut pwd = std::mem::zeroed::<Passwd>();
        let mut res = std::ptr::null_mut();
        // Some systems define sysconf(_SC_GETPW_R_SIZE_MAX) to something different - too bad!
        let mut buf = vec![0i8; 1024];

        let ret = getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.len(), &mut res);

        if ret == 0 && !res.is_null() {
            Some(PathBuf::from(OsString::from_vec(
                CStr::from_ptr(pwd.pw_dir).to_bytes().to_vec(),
            )))
        } else {
            if let Some(fallback_working_dir) = get_unix_working_dir_fallback() {
                if fallback_working_dir.is_absolute() {
                    // Also catched later, here for performance
                    // No need to deconstruct and check
                    let pattern = {
                        #[cfg(target_os = "macos")]
                        {
                            "/Users"
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            "/home"
                        }
                    };
                    if fallback_working_dir == PathBuf::from(pattern) {
                        return Some(fallback_working_dir);
                    }
                    let components = fallback_working_dir
                        .components()
                        .map(|c| c.as_os_str().to_os_string())
                        .collect::<Vec<OsString>>();
                    let pattern = pattern.strip_prefix("/").expect(
                        "Unable to strip prefix from pattern definitly containing said pattern",
                    );
                    let home_index = components
                        .iter()
                        .position(|c| c == &OsString::from(pattern));
                    if let Some(home_index) = home_index {
                        if components.len() > home_index + 1 {
                            return Some(components[0..=home_index + 1].iter().collect());
                        } else {
                            // Should be unreachable because of the very first if block
                            // `if fallback_working_dir == PathBuf::from("/home")`
                            return Some(components[0..=home_index].iter().collect());
                        }
                    }
                }
            }
            None
        }
    }
}
