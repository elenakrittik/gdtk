use std::{io::Error as IOError, path::PathBuf};

/// Returns whether a given Godot version is stable.
pub fn is_stable(ver: &versions::Versioning) -> bool {
    match ver {
        versions::Versioning::Ideal(versions::SemVer {
            pre_rel: Some(versions::Release(vec)),
            ..
        })
        | versions::Versioning::General(versions::Version {
            release: Some(versions::Release(vec)),
            ..
        }) => vec.as_slice() == [versions::Chunk::Alphanum("stable".to_owned())],
        _ => false,
    }
}

pub fn write_local_versions(data: &crate::types::Versions) -> Result<(), crate::Error> {
    let versions_toml = versions_toml_path()?;

    std::fs::write(versions_toml, toml::to_string_pretty(&data)?)?;

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
