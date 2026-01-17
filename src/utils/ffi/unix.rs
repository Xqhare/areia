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
            None
        }
    }
}
