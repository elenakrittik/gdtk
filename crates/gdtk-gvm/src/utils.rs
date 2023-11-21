use rayon::prelude::*;
use versions::Versioning;

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
    fn num(version: &versions::Version, idx: usize) -> u32 {
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

pub fn get_version_archive_name(version: String) -> Result<String, crate::Error> {
    // versions are of form "Godot_v{version}-{release}_{platform}.zip"
    // {release} is "stable" if the version is not unstable

    // i didn't investigate < 4.0 naming semantics yet
    if version.starts_with('4') {
        let platform = match std::env::consts::OS {
            "windows" => match std::env::consts::ARCH {
                "x86" => Ok("win32"),
                "x86_64" => Ok("win64"),
                _ => Err(crate::Error::GodotUnsupportedPlatform(platform())),
            },
            _ => Err(crate::Error::GDTKUnsupportedPlatform(platform())),
        }?;

        return Ok(format!("Godot_v{version}_{platform}.zip"));
    }

    Err(crate::Error::GDTKUnsupportedVersionForInstall(version))
}
