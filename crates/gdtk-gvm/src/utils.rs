use std::fmt::Display;

use crate::{api::ReleaseAsset, types::LocalVersions};

/// Pick the best match for a given ``version`` from ``vers``.
pub fn coerce_version<V: Display>(input: &str, pool: Vec<V>) -> Result<Vec<V>, crate::Error> {
    let matches_ = pool
        .into_iter()
        .filter(|ver| ver.to_string().starts_with(input))
        .collect::<Vec<_>>();

    Ok(matches_)
}

pub fn pick_asset(assets: &[ReleaseAsset], mono: bool) -> Option<&ReleaseAsset> {
    // something something consistency
    // see https://github.com/godotengine/godot-builds/issues/5

    let suffix = match (mono, std::env::consts::OS, std::env::consts::ARCH) {
        (false, "windows", "x86_64") => "win64.exe.zip",
        (false, "windows", "x86") => "win32.exe.zip",
        (false, "windows", "aarch64") => "windows_arm64.exe.zip",
        (false, "linux", "x86_64") => "linux.x86_64.zip",
        (false, "linux", "x86") => "linux.x86_32.zip",
        (false, "linux", "arm") => "linux.arm32.zip",
        (false, "linux", "aarch64") => "linux.arm64.zip",
        (false, "macos", "x86_64" | "aarch64") => "macos.universal.zip",
        // --
        (true, "windows", "x86_64") => "mono_win64.zip",
        (true, "windows", "x86") => "mono_win32.zip",
        (true, "windows", "aarch64") => "mono_windows_arm64.zip",
        (true, "linux", "x86_64") => "mono_linux_x86_64_zip",
        (true, "linux", "x86") => "mono_linux_x86_32_zip",
        (true, "linux", "arm") => "mono_linux_arm32.zip",
        (true, "linux", "aarch64") => "mono_linux_arm64.zip",
        (true, "macos", "x86_64" | "aarch64") => "mono_macos.universal.zip",
        _ => return None,
    };

    assets.iter().find(|asset| asset.name.ends_with(suffix))
}

/// If the given path does not exist, create an empty [crate::types::LocalVersions] instance,
/// write it to that path, and return that instance.
pub fn maybe_create_local_versions(
    path: &std::path::Path,
) -> Result<Option<LocalVersions>, crate::Error> {
    if !path.exists() {
        let data = LocalVersions(vec![]);
        let encoded = rkyv::to_bytes::<rkyv::rancor::Error>(&data)?;

        std::fs::write(path, encoded)?;

        return Ok(Some(data));
    }

    Ok(None)
}
