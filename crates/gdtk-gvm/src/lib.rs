#![feature(decl_macro, min_specialization)]

pub use versions;

pub use crate::error::Error;
pub use crate::manager::VersionManager;

pub(crate) mod api;
pub mod error;
pub mod manager;
pub mod online;
pub mod types;
pub mod utils;
pub mod version;
