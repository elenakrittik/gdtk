#![feature(decl_macro, option_take_if)]

use std::{io::Error as IOError, path::PathBuf};

pub use toml;
use toml::Table;
pub use versions;

pub use crate::error::Error;
pub mod error;
pub mod online;
pub mod utils;

/// Returns versions.toml content as a hash map of version string to relative path.
pub fn read_local_versions() -> Result<Table, crate::Error> {
    let versions_toml = versions_toml_path()?;

    let table = std::fs::read_to_string(versions_toml)?.parse::<Table>()?;

    Ok(table)
}

pub fn write_local_versions(table: Table) -> Result<(), crate::Error> {
    let versions_toml = versions_toml_path()?;

    std::fs::write(versions_toml, table.to_string())?;

    Ok(())
}

pub fn versions_toml_path() -> Result<PathBuf, IOError> {
    let mut conf_dir = gdtk_utils::base_conf_dir()?;

    conf_dir.push("versions.toml");

    gdtk_utils::ensure_path(&conf_dir, false)?;

    Ok(conf_dir)
}

pub fn godots_path() -> Result<PathBuf, IOError> {
    let mut data_dir = gdtk_utils::base_data_dir()?;

    data_dir.push("godots");

    gdtk_utils::ensure_path(&data_dir, true)?;

    Ok(data_dir)
}
