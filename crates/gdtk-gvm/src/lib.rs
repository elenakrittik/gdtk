#![feature(decl_macro)]

pub use versions;

pub use crate::error::Error;
pub use crate::manager::VersionManager;
pub mod error;
pub mod manager;
pub mod online;
pub mod queries;
pub mod types;
pub mod utils;
pub mod version;
