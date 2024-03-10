#![feature(decl_macro, option_take_if)]

use std::{io::Error as IOError, path::PathBuf};

pub use versions;
pub use toml;
use toml::{de::Error as TOMLDeError, map::Map, Table, Value};
use versions::Version;

pub mod online;
pub mod utils;

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

/// Returns versions.toml content as a hash map of version string to relative path.
pub fn read_local_versions() -> Result<Map<String, Value>, Error> {
    let versions_toml = versions_toml_path()?;

    let table = std::fs::read_to_string(versions_toml)?.parse::<Table>()?;

    Ok(table)
}

pub fn write_local_versions(table: Map<String, Value>) -> Result<(), Error> {
    let versions_toml = versions_toml_path()?;

    std::fs::write(versions_toml, table.to_string())?;

    Ok(())
}

pub fn versions_toml_path() -> Result<PathBuf, IOError> {
    let mut conf_dir = gdtk_utils::base_conf_dir()?;

    conf_dir.push("versions.toml");

    Ok(conf_dir)
}

pub fn godots_path() -> Result<PathBuf, IOError> {
    let mut data_dir = gdtk_utils::base_data_dir()?;

    data_dir.push("godots");

    Ok(data_dir)
}

pub fn ensure_godots() -> Result<(), IOError> {
    gdtk_utils::ensure_path(gdtk_utils::base_data_dir()?, true)?;
    gdtk_utils::ensure_path(godots_path()?, true)?;

    Ok(())
}

pub fn is_stable(ver: &versions::Versioning) -> bool {
    match ver {
        versions::Versioning::Ideal(
            versions::SemVer {
                pre_rel: Some(
                    versions::Release(vec)
                ),
                ..
            }
        ) |
        versions::Versioning::General(
            versions::Version {
                release: Some(
                    versions::Release(vec)
                ),
                ..
            }
        ) => vec.as_slice() == [versions::Chunk::Alphanum("stable".to_owned())],
        _ => false,
    }
}
