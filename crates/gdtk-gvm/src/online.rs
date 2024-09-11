use futures_util::StreamExt;

use crate::version::Version;

pub async fn fetch_versions() -> Result<Vec<Version>, crate::Error> {
    let github = octocrab::Octocrab::builder().build()?;

    let releases = github
        .repos("godotengine", "godot-builds")
        .releases()
        .list()
        .per_page(100)
        .send()
        .await?
        .into_stream(&github)
        .filter_map(|r| async { r.ok() })
        .map(Version::from)
        .collect::<Vec<_>>()
        .await;

    Ok(releases)
}

// pub async fn fetch_versions() -> Result<Vec<Version>, crate::Error> {
//     let text = reqwest::get(GODOT_DOWNLOADS_ROOT).await?.text().await?;
//     let html = scraper::Html::parse_document(&text);
//     let selector = scraper::Selector::parse(".archive-version > h4").unwrap();

//     let versions = html
//         .select(&selector)
//         .filter_map(|elem| elem.attr("id"))
//         .filter_map(versions::Version::new)
//         .map(crate::utils::strip_stable_postfix)
//         .map(|v| Version::new(v, false))
//         .collect();

//     Ok(versions)
// }

// pub async fn version_download_urls(
//     version: &versions::Version,
//     version_string: &str,
// ) -> Result<AHashMap<(&'static str, &'static str), url::Url>, crate::Error> {
//     let mut url = GODOT_DOWNLOADS_ROOT.to_owned() + version_string;

//     if crate::utils::is_stable(version) {
//         url += "-stable";
//     }

//     let text = reqwest::get(url).await?.text().await?;
//     let html = scraper::Html::parse_document(&text);
//     let selector = scraper::Selector::parse(".download > span > a").unwrap();

//     let downloads = html
//         .select(&selector)
//         .filter_map(|elem| elem.attr("href"))
//         .filter_map(|href| map_href_to_arch(href).ok().zip(url::Url::parse(href).ok()))
//         .collect();

//     Ok(downloads)
// }

// fn map_href_to_arch(href: &str) -> Result<(&'static str, &'static str), crate::Error> {
//     if href.contains("mono") {
//         return Err(crate::Error::MonoUnsupported);
//     }

//     // TODO: use a regex instead?

//     // hardcoding :(
//     match href {
//         href if href.ends_with("android_editor.apk") => Ok(("arm64", "android")),
//         href if href.ends_with("linux.x86_64.zip") => Ok(("x86_64", "linux")),
//         href if href.ends_with("linux.x86_32.zip") => Ok(("x86", "linux")),
//         href if href.ends_with("linux.arm64.zip") => Ok(("arm64", "linux")),
//         href if href.ends_with("linux.arm32.zip") => Ok(("arm32", "linux")),
//         href if href.ends_with("macos.universal.zip") => Ok(("darwinany", "macos")),
//         href if href.ends_with("win64.exe.zip") => Ok(("x86_64", "windows")),
//         href if href.ends_with("win32.exe.zip") => Ok(("x86", "windows")),
//         _ => Err(crate::Error::UnknownDownloadUrl(href.into())),
//     }
// }
