#![feature(decl_macro)]

use std::{io::Error as IOError, path::PathBuf};

use ahash::AHashMap;
use gdtk_macros::unwrap_enum;
use toml::{de::Error as TOMLDeError, Table, Value};
use versions::Version;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to find config path.")]
    PathNotFound,

    #[error("I/O error: {0:?}")]
    IOError(std::io::Error),

    #[error("TOML deserialization error: {0:?}")]
    TOMLDeserializationError(toml::de::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(reqwest::Error),

    #[error("Custom builds are not supported yet.")]
    CustomBuildsUnsupported,

    #[error("Invalid version: {0}")]
    InvalidVersion(String),

    #[error("Invalid Godot version: {0}")]
    InvalidGodotVersion(Version),

    #[error("Installing Godot version {0} is not yet supported.")]
    GDTKUnsupportedVersionForInstall(String),

    #[error("Your platform ({0}) is not supported by Godot at the moment.")]
    GodotUnsupportedPlatform(String),

    #[error("Your platform ({0}) is not supported by GDTK at the moment.")]
    GDTKUnsupportedPlatform(String),
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IOError(value)
    }
}

impl From<TOMLDeError> for Error {
    fn from(value: TOMLDeError) -> Self {
        Self::TOMLDeserializationError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

pub fn ensure_versions() -> Result<(), IOError> {
    gdtk_utils::ensure_path(gdtk_utils::base_conf_dir()?, true)?;
    gdtk_utils::ensure_path(versions_toml_path()?, false)?;

    Ok(())
}

/// Returns versions.toml content as a hash map of project path to Godot
/// version string.
pub fn get_local_versions() -> Result<AHashMap<String, String>, Error> {
    let versions_toml = versions_toml_path()?;

    let table = std::fs::read_to_string(versions_toml)?.parse::<Table>()?;

    let tab = table
        .into_iter()
        .filter(|(_, v)| matches!(v, Value::String(_)))
        .map(|(k, v)| (k, unwrap_enum!(v, Value::String(s), s)))
        .collect::<AHashMap<_, _>>();

    Ok(tab)
}

pub fn versions_toml_path() -> Result<PathBuf, IOError> {
    let mut conf_dir = gdtk_utils::base_conf_dir()?;

    conf_dir.push("versions.toml");

    Ok(conf_dir)
}

pub mod online;
pub mod utils;
