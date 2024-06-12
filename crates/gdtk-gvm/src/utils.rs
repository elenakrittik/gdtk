use versions::{Release, SemVer, Version, Versioning};

fn get_stable_chunk() -> versions::Chunk {
    versions::Chunk::Alphanum("stable".to_string())
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

/// Pick the best match for a given ``version`` from ``vers``.
pub fn coerce_version(
    version: Versioning,
    vers: Vec<Versioning>,
) -> Result<Vec<Versioning>, crate::Error> {
    let matches_ = vers
        .into_iter()
        .filter(|ver| ver.to_string().starts_with(&version.to_string()))
        .filter(|ver| ver >= &version)
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
            }) => Versioning::Ideal(SemVer {
                major,
                minor,
                patch,
                pre_rel: None,
                meta,
            }),
            Versioning::General(Version {
                epoch,
                chunks,
                release: _,
                meta,
            }) => Versioning::General(Version {
                epoch,
                chunks,
                release: None,
                meta,
            }),
            _ => unreachable!(), // godot's versions are never Versioning::Complex
        }
    } else {
        ver
    }
}

pub const fn arch_os() -> (&'static str, &'static str) {
    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        std::env::consts::ARCH
    };

    (arch, std::env::consts::OS)
}
