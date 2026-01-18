#![doc = include_str!("../README.md")]

mod dirs;
mod hidden;
mod utils;

pub mod error;

pub use dirs::{BaseDirs, UserDirs};
pub use hidden::{Hidden, SuperHidden};
