use versions::Versioning;

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

pub const fn arch_os() -> (&'static str, &'static str) {
    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        std::env::consts::ARCH
    };

    (arch, std::env::consts::OS)
}
