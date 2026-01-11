
#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod unix;

#[cfg(target_os = "windows")]
pub mod windows;
