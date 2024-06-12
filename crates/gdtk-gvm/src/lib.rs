#![feature(decl_macro)]

pub use versions;

pub use crate::error::Error;
pub use crate::manager::VersionManager;
pub mod error;
pub mod manager;
pub mod online;
pub mod types;
pub mod utils;
