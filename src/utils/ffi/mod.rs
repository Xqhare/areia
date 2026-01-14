#[cfg(any(target_os = "macos", doc))]
pub mod macos;

#[cfg(any(unix, doc))]
pub mod unix;

#[cfg(any(target_os = "windows", doc))]
pub mod windows;
