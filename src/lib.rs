#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![doc = include_str!("../README.md")]

mod autos;
mod dirs;
mod hidden;
mod utils;

pub mod error;

// API

// Directories
pub use dirs::{BaseDirs, UserDirs};
// Traits
pub use hidden::{Hidden, SuperHidden};
// Utilities
pub use autos::{auto_creator, auto_deletor};
