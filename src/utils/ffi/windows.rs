use std::{ffi::{c_void, OsString}, fmt::Display, path::{PathBuf, Path}, os::windows::ffi::{OsStringExt, OsStrExt}};

use crate::error::{AreiaError, AreiaResult};

pub enum FolderID {
    Profile,
    LocalAppData,
    RoamingAppData,
    Desktop,
    Document,
    Download,
    Music,
    Picture,
    Public,
    Template,
    Video
}

impl Display for FolderID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FolderID::Profile => write!(f, "Profile"),
            FolderID::LocalAppData => write!(f, "LocalAppData"),
            FolderID::RoamingAppData => write!(f, "RoamingAppData"),
            FolderID::Desktop => write!(f, "Desktop"),
            FolderID::Document => write!(f, "Document"),
            FolderID::Download => write!(f, "Download"),
            FolderID::Music => write!(f, "Music"),
            FolderID::Picture => write!(f, "Picture"),
            FolderID::Public => write!(f, "Public"),
            FolderID::Template => write!(f, "Template"),
            FolderID::Video => write!(f, "Video")
        }
    }
}

#[repr(C)]
pub struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8]
}

// Home
const FOLDERID_PROFILE: GUID = GUID {
    data1: 0x5e6c858f,
    data2: 0x0e22,
    data3: 0x4760,
    data4: [0x9A, 0xFE, 0xEA, 0x33, 0x17, 0xB6, 0x71, 0x71]
};

const FOLDERID_LOCALAPPDATA: GUID = GUID {
    data1: 0xf1b32785,
    data2: 0x6fba,
    data3: 0x4fcf,
    data4: [0x9d, 0x55, 0x7b, 0x8e, 0x7f, 0x15, 0x70, 0x91]
};

const FOLDERID_ROAMINGAPPDATA: GUID = GUID {
    data1: 0x3eb685db,
    data2: 0x65f9,
    data3: 0x4cf6,
    data4: [0xa0, 0x3a, 0xe3, 0xef, 0x65, 0x72, 0x9f, 0x3d]
};

const FOLDERID_DESKTOP: GUID = GUID {
    data1: 0xb4bfcc3a,
    data2: 0xdb2c,
    data3: 0x424c,
    data4: [0xb0, 0x29, 0x7f, 0xe9, 0x9a, 0x87, 0xc6, 0x41]
};

const FOLDERID_DOCUMENT: GUID = GUID {
    data1: 0xfdd39ad0,
    data2: 0x238f,
    data3: 0x46af,
    data4: [0xad, 0xb4, 0x6c, 0x85, 0x48, 0x03, 0x69, 0xc7]
};

const FOLDERID_DOWNLOAD: GUID = GUID {
    data1: 0x374de290,
    data2: 0x123f,
    data3: 0x4565,
    data4: [0x91, 0x65, 0x39, 0xc4, 0x92, 0x5e, 0x46, 0x7b]
};

const FOLDERID_MUSIC: GUID = GUID {
    data1: 0x4bd8d571,
    data2: 0x6d19,
    data3: 0x48d3,
    data4: [0xbe, 0x97, 0x42, 0x22, 0x20, 0x08, 0x0e, 0x43]
};

const FOLDERID_PICTURE: GUID = GUID {
    data1: 0x33e28130,
    data2: 0x4e1e,
    data3: 0x4676,
    data4: [0x83, 0x5a, 0x98, 0x39, 0x5c, 0x3b, 0xc3, 0xbb]
};

const FOLDERID_PUBLIC: GUID = GUID {
    data1: 0xdfdf76a2,
    data2: 0xc82a,
    data3: 0x4d63,
    data4: [0x90, 0x6a, 0x56, 0x44, 0xac, 0x45, 0x73, 0x85]
};

const FOLDERID_TEMPLATE: GUID = GUID {
    data1: 0xa63293e8,
    data2: 0x664e,
    data3: 0x48db,
    data4: [0xa0, 0x79, 0xdf, 0x75, 0x9e, 0x05, 0x09, 0xf7]
};

const FOLDERID_VIDEO: GUID = GUID {
    data1: 0x18989b1d,
    data2: 0x99b5,
    data3: 0x455b,
    data4: [0x84, 0x1c, 0xab, 0x7c, 0x74, 0xe4, 0xdd, 0xfc]
};

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x02;
// For "super" hiding like on mac
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x04;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_FILE_ATTRIBUTES: u32 = u32::MAX;

#[link(name = "shell32")]
unsafe extern "system" {
    fn SHGetKnownFolderPath(
        rfid: *const GUID,
        dwFlags: u32,
        hToken: *mut c_void,
        ppszPath: *mut *mut u16
    ) -> i32;
}

#[link(name = "ole32")]
unsafe extern "system" {
    fn CoTaskMemFree(pv: *mut c_void);
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn SetFileAttributesW(lpFileName: *const u16, dwFileAttributes: u32) -> i32;
    fn GetFileAttributesW(lpFileName: *const u16) -> u32;
}

/// Sets the hidden attribute for a file or directory and keeps any existing attributes
pub fn hide(path: &Path) -> AreiaResult<()> {

    unsafe {
        let (wide_path, attrs) = wide_path(path)?;

        let mut new_attrs = attrs | FILE_ATTRIBUTE_HIDDEN;
        new_attrs &= !FILE_ATTRIBUTE_NORMAL;

        let result = SetFileAttributesW(wide_path.as_ptr(), new_attrs);
        if result == 0 {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::WindowsError(format!("Failed to set hidden attribute: {}", err)));
        }
    }
    
    Ok(())
}

pub fn super_hide(path: &Path) -> AreiaResult<()> {

    unsafe {
        let (wide_path, attrs) = wide_path(path)?;

        let mut new_attrs = attrs | FILE_ATTRIBUTE_HIDDEN;
        new_attrs |= FILE_ATTRIBUTE_SYSTEM;
        new_attrs &= !FILE_ATTRIBUTE_NORMAL;

        let result = SetFileAttributesW(wide_path.as_ptr(), new_attrs);
        if result == 0 {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::WindowsError(format!("Failed to set super hidden attributes: {}", err)));
        }
    }
    
    Ok(())
}

/// Removes the hidden attribute for a file or directory and keeps any existing attributes
pub fn super_unhide(path: &Path) -> AreiaResult<()> {

    unsafe {
        let (wide_path, attrs) = wide_path(path)?;

        let mut new_attrs = attrs & !FILE_ATTRIBUTE_HIDDEN;
        new_attrs &= !FILE_ATTRIBUTE_SYSTEM;
        if new_attrs == 0 {
            new_attrs = FILE_ATTRIBUTE_NORMAL;
        }

        let result = SetFileAttributesW(wide_path.as_ptr(), new_attrs);
        if result == 0 {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::WindowsError(format!("Failed to remove super hidden attributes: {}", err)));
        }
    }
    
    Ok(())
}

/// Removes the hidden attribute for a file or directory and keeps any existing attributes
pub fn unhide(path: &Path) -> AreiaResult<()> {
    unsafe {
        let (wide_path, attrs) = wide_path(path)?;

        let mut new_attrs = attrs & !FILE_ATTRIBUTE_HIDDEN;
        if new_attrs == 0 {
            new_attrs = FILE_ATTRIBUTE_NORMAL;
        }

        let result = SetFileAttributesW(wide_path.as_ptr(), new_attrs);
        if result == 0 {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::WindowsError(format!("Failed to remove hidden attribute: {}", err)));
        }
    }
    
    Ok(())
}

/// Checks if the hidden attribute is set
pub fn is_hidden(path: &Path) -> AreiaResult<bool> {
    // All checks are done in `wide_path`
    unsafe {
        let (_, attrs) = wide_path(path)?;
        Ok((attrs & FILE_ATTRIBUTE_HIDDEN) != 0)
    }
}

/// Checks if the hidden and system attribute is set
pub fn is_super_hidden(path: &Path) -> AreiaResult<bool> {
    // All checks are done in `wide_path`
    unsafe {
        let (_, attrs) = wide_path(path)?;
        Ok((attrs & FILE_ATTRIBUTE_HIDDEN) != 0 && (attrs & FILE_ATTRIBUTE_SYSTEM) != 0)
    }
}

/// Returns the path to a known folder
///
/// # Arguments
/// * `folder` - The folder to get the path for - see `FolderID` enum
pub fn get_path(folder: FolderID) -> AreiaResult<PathBuf> {
    let mut path_ptr: *mut u16 = std::ptr::null_mut();
    let rfid = match folder {
        FolderID::Profile => &FOLDERID_PROFILE,
        FolderID::LocalAppData => &FOLDERID_LOCALAPPDATA,
        FolderID::RoamingAppData => &FOLDERID_ROAMINGAPPDATA,
        FolderID::Desktop => &FOLDERID_DESKTOP,
        FolderID::Document => &FOLDERID_DOCUMENT,
        FolderID::Download => &FOLDERID_DOWNLOAD,
        FolderID::Music => &FOLDERID_MUSIC,
        FolderID::Picture => &FOLDERID_PICTURE,
        FolderID::Public => &FOLDERID_PUBLIC,
        FolderID::Template => &FOLDERID_TEMPLATE,
        FolderID::Video => &FOLDERID_VIDEO
    };
    let result = unsafe {
        SHGetKnownFolderPath(rfid, 0, std::ptr::null_mut(), &mut path_ptr)
    };
    if result == 0 && !path_ptr.is_null() {
        let mut len = 0;
        unsafe {
            while *path_ptr.add(len) != 0 { len += 1; }
            let slice = std::slice::from_raw_parts(path_ptr, len);
            let path = PathBuf::from(OsString::from_wide(slice));
            CoTaskMemFree(path_ptr as *mut c_void);
            Ok(path)
        }
    } else {
        Err(AreiaError::WindowsError(format!("Failed to get {}. HRESULT: 0x{:x}", folder, result)))
    }

}

/// Takes a path and returns the wide path needed for the Windows API
///
/// Returns the wide path as `.0` and the file attributes as `.1`
fn wide_path(path: &Path) -> AreiaResult<(Vec<u16>, u32)> {
    unsafe {
        let wide_path: Vec<u16> = path.as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let attrs = GetFileAttributesW(wide_path.as_ptr());
        if attrs == INVALID_FILE_ATTRIBUTES {
            let err = std::io::Error::last_os_error();
            return Err(AreiaError::WindowsIoError(err));
        }

        Ok((wide_path, attrs))
    }
}
