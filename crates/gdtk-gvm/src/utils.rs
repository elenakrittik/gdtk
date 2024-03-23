use std::{io::Error as IOError, path::PathBuf};

use versions::{Release, SemVer, Version, Versioning};

fn get_stable_chunk() -> versions::Chunk {
    versions::Chunk::Alphanum("stable".to_owned())
}

/// Returns whether a given Godot version is stable.
pub fn is_stable(ver: &Versioning) -> bool {
    match ver {
        Versioning::Ideal(SemVer {
            pre_rel: Some(Release(vec)),
            ..
        })
        | Versioning::General(Version {
            release: Some(Release(vec)),
            ..
        }) => vec.as_slice() == [get_stable_chunk()],
        Versioning::Ideal(SemVer { pre_rel: None, .. })
        | Versioning::General(Version { release: None, .. }) => true,
        _ => false,
    }
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

pub fn coerce_version(version: Versioning, vers: Vec<Versioning>) -> Result<Vec<Versioning>, crate::Error> {
    let matches_ = vers
        .into_iter()
        .filter(|ver| ver.to_string().starts_with(&version.to_string()))
        .filter(|ver| ver >= &&version)
        .collect::<Vec<_>>();

    Ok(matches_)
}

pub(crate) fn strip_stable_postfix(ver: Versioning) -> Versioning {
    if is_stable(&ver) {
        match ver {
            Versioning::Ideal(SemVer {
                major,
                minor,
                patch,
                pre_rel: _,
                meta,
            }) => Versioning::Ideal(SemVer { major, minor, patch, pre_rel: None, meta }),
            Versioning::General(Version {
                epoch,
                chunks,
                release: _,
                meta,
            }) => Versioning::General(Version { epoch, chunks, release: None, meta }),
            _ => unreachable!(), // godot's versions are never Versioning::Complex
        }
    } else {
        ver
    }
}
