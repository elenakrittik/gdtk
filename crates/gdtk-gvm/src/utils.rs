use rayon::prelude::*;
use versions::{Versioning, Version};

pub fn sort_versions(vers: Vec<String>) -> Vec<String> {
    let mut itermediate = vers
        .into_iter()
        .map(|ver| Versioning::new(ver.as_str()).unwrap())
        .collect::<Vec<_>>();

    itermediate.par_sort_unstable();

    itermediate.into_iter().map(|ver| ver.to_string()).collect()
}

pub fn format_version(ver: String) -> String {
    let ver = Versioning::new(ver.as_str()).unwrap();

    #[inline]
    fn num(version: &Version, idx: usize) -> u32 {
        version.chunks.0[idx].single_digit().unwrap()
    }

    match ver {
        Versioning::Ideal(semver) => format!(
            "{}.{} ({}.{}.{})",
            semver.major, semver.minor, semver.major, semver.minor, semver.patch
        ),
        Versioning::General(version) => {
            if version.chunks.0.len() >= 3 {
                format!(
                    "{}.{} ({})",
                    num(&version, 0),
                    num(&version, 1),
                    version.to_string()
                )
            } else {
                version.to_string()
            }
        }
        Versioning::Complex(mess) => mess.to_string(),
    }
}

#[inline]
fn platform() -> String {
    format!("{} on {}", std::env::consts::ARCH, std::env::consts::OS)
}

pub fn get_version_archive_name(version: String, release: Option<String>) -> Result<String, crate::Error> {
    // versions are of form "Godot_v{version}-{release}_{platform}.zip"
    // {release} is "stable" if the version is not unstable

    let release = release.unwrap_or("stable".to_string());
    let platform = match &version.chars().nth(0) {
        Some(major) => match major {
            '4' => get_godot4_platform(),
            _ => Err(crate::Error::GDTKUnsupportedVersionForInstall(version.clone())),
        },
        None => unreachable!(),
    }?;

    Ok(format!("Godot_v{version}-{release}_{platform}.zip"))
}

fn get_godot4_platform() -> Result<String, crate::Error> {
    match std::env::consts::OS {
        "windows" => match std::env::consts::ARCH {
            "x86" => Ok("win32.exe"),
            "x86_64" => Ok("win64.exe"),
            _ => Err(crate::Error::GodotUnsupportedPlatform(platform())),
        },
        "linux" => match std::env::consts::ARCH {
            "x86" => Ok("linux.x86_32"),
            "x86_64" => Ok("linux.x86_64"),
            "aarch64" => Ok("linux.x86_64"), // TODO: remove once done testing
            _ => Err(crate::Error::GodotUnsupportedPlatform(platform())),
        },
        "macos" => match std::env::consts::ARCH {
            "x86_64" | "aarch64" => Ok("macos.universal"),
            _ => Err(crate::Error::GodotUnsupportedPlatform(platform())),
        },
        _ => Err(crate::Error::GDTKUnsupportedPlatform(platform())),
    }.map(|v| v.into())
}
