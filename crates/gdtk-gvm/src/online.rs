use ahash::AHashMap;
use futures::future::join_all;
use scraper::Html;
use scraper::Selector;
use serde::Deserialize;
use versions::Version;
use versions::Versioning;

use crate::Error;

const GODOT_DOWNLOADS_ROOT: &str = "http://download.tuxfamily.org/godotengine/";
const IGNORED_IDENTIFIERS: &[&str] = &[
    "Parent Directory",
    "media",
    "patreon",
    "testing",
    "toolchains",
];
const GODOT_UNSTABLE_TAGS: &[&str] = &["alpha", "beta", "dev", "rc"];
const GODOT_SUPPORT_DATA: &str =
    "https://raw.githubusercontent.com/elenakrittik/gdtk/master/godot_support.json";

#[derive(Debug)]
pub struct FetchVersionsOptions {
    // Fetch versions that are no longer supported. See https://github.com/godotengine/godot-docs/blob/master/about/release_policy.rst
    pub unsupported: bool,
    // Fetch development snapshots (dev, alpha, beta and rc).
    pub dev: bool,
    /// Fetch development snapshots for unsupported versions.
    pub unsupported_dev: bool,
}

#[derive(Debug, Deserialize)]
pub struct GodotVersion {
    version: String,
    /// 0 - in development, 1 - stable
    status: u8,
}

pub async fn fetch_versions(opt: FetchVersionsOptions) -> Result<Vec<String>, Error> {
    let html = reqwest::get(GODOT_DOWNLOADS_ROOT).await?.text().await?;
    let html = Html::parse_document(&html);
    let selector = Selector::parse("tr > td > a").unwrap();

    let mut versions = Vec::new();

    // Collect all theoretically-available versions first.
    for elem in html.select(&selector) {
        let ver_text: String = elem.text().collect();

        // Ignore non-versions like "Parent directory"
        if IGNORED_IDENTIFIERS.contains(&ver_text.as_ref()) {
            continue;
        }

        versions.push(ver_text);
    }

    let godots = reqwest::get(GODOT_SUPPORT_DATA)
        .await?
        .json::<Vec<GodotVersion>>()
        .await?;

    let stables: Vec<&str> = godots
        .iter()
        .filter(|v| v.status == 1)
        .map(|v| v.version.as_str())
        .collect::<Vec<_>>();

    let mut unstables: Vec<&str> = godots
        .iter()
        .filter(|v| v.status == 0)
        .map(|v| v.version.as_str())
        .collect::<Vec<_>>();

    if !opt.unsupported {
        // A mapping of "major.minor" to a struct representing "major.minor[.patch]"
        let mut latest_versions: AHashMap<&str, Versioning> = AHashMap::new();

        // Not the code i am proud of, i must admit, but at least it works.

        // For each Godot version fetched from tuxfamily
        for godot_version in versions {
            for stable_godot_version in &stables {
                // We check if it is one of the versions marked as stable in our
                // custom data set
                if godot_version.starts_with(stable_godot_version) {
                    // If yes, we check if we already have a version set as
                    // "latest" in our hashmap.
                    let latest: Option<&Versioning> = latest_versions.get(stable_godot_version);

                    match latest {
                        Some(latest) => {
                            // If yes, we compare our currently-iterated-over version
                            // with the one in the hashmap, and if it is greater, we
                            // set it as the latest
                            let godot_version = Versioning::new(godot_version.as_str()).unwrap();

                            if &godot_version > latest {
                                latest_versions.insert(stable_godot_version, godot_version);
                            }
                        }
                        // If not, we simply set it as the latest.
                        None => {
                            latest_versions.insert(
                                stable_godot_version,
                                Versioning::new(godot_version.as_str()).unwrap(),
                            );
                        }
                    }
                }
            }
        }

        versions = latest_versions.values().map(|v| v.to_string()).collect();
    } else {
        versions.retain(|v| !unstables.contains(&v.as_str()));
    }

    if opt.dev {
        if opt.unsupported_dev {
            unstables.extend(versions.iter().map(|v| v.as_str()));
        }

        let unstables = join_all(unstables.iter().map(|v| fetch_unstable_versions(v)))
            .await
            .into_iter()
            .flat_map(|v| v.unwrap_or_default())
            .collect::<Vec<_>>();

        versions.extend(unstables);
    }

    Ok(versions)
}

pub async fn fetch_unstable_versions(ver: &str) -> Result<Vec<String>, Error> {
    let url = GODOT_DOWNLOADS_ROOT.to_string() + ver;
    let html = reqwest::get(url).await?.text().await?;
    let html = Html::parse_document(&html);
    let selector = Selector::parse("tr > td > a").unwrap();

    let versions = html
        .select(&selector)
        .map(|v| v.text().collect::<String>())
        .filter(|v| GODOT_UNSTABLE_TAGS.iter().any(|tag| v.starts_with(tag)))
        .filter(|v| !IGNORED_IDENTIFIERS.contains(&v.as_ref()))
        .map(|v| ver.to_owned() + "-" + &v)
        .collect();

    Ok(versions)
}

pub fn get_version_download_url(version: String) -> Result<String, Error> {
    let url = GODOT_DOWNLOADS_ROOT.to_string();
    let url = url + &version.replace('-', "/");

    let majorminor_ver = version.chars().take(3).collect();
    let release = Version::new(&version)
        .unwrap()
        .release
        .map(|v| v.to_string());

    let url = url + "/" + crate::utils::get_version_archive_name(majorminor_ver, release)?.as_str();

    Ok(url)
}

pub async fn check_version_exists(version: String) -> Result<bool, Error> {
    let url = get_version_download_url(version)?;
    dbg!(&url);
    let client = reqwest::Client::new();

    let status = client.head(url).send().await?.status();

    Ok(status.is_success())
}
