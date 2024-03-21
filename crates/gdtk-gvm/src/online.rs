const GODOT_DOWNLOADS_ROOT: &str = "https://godotengine.org/download/archive/";

#[extend::ext]
impl<T, U> Option<T> {
    fn combine(self, other: Option<U>) -> Option<(T, U)> {
        match (self, other) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

pub async fn fetch_versions() -> Result<Vec<versions::Versioning>, crate::Error> {
    let text = reqwest::get(GODOT_DOWNLOADS_ROOT).await?.text().await?;
    let html = scraper::Html::parse_document(&text);
    let selector = scraper::Selector::parse(".archive-version > h4").unwrap();

    let versions = html
        .select(&selector)
        .filter_map(|elem| elem.attr("id"))
        .filter_map(|ver| versions::Versioning::new(ver))
        .collect();

    Ok(versions)
}

pub async fn version_download_urls(version: &str) -> Result<VersionDownloadUrls, crate::Error> {
    let url = GODOT_DOWNLOADS_ROOT.to_owned() + version;
    let text = reqwest::get(url).await?.text().await?;
    let html = scraper::Html::parse_document(&text);
    let selector = scraper::Selector::parse(".download > a").unwrap();

    let downloads = html
        .select(&selector)
        .filter_map(|elem| elem.attr("href"))
        .filter_map(|href| {
            match map_href_to_arch(href).ok() {
                Some(arch) => Some((arch, href)),
                None => None,
            }
        })
        .collect::<Vec<_>>();

    todo!()
}

fn map_href_to_arch(href: &str) -> Result<(&'static str, &'static str), crate::Error> {
    if href.contains("mono") {
        return Err(crate::Error::MonoUnsupported);
    }

    // hardcoding :(
    match href {
        href if href.ends_with("android_editor.apk") => Ok(("aarch64", "android")),
        href if href.ends_with("linux.x86_64.zip") => Ok(("x86_64", "linux")),
        href if href.ends_with("macos.universal.zip") => Ok(("x86_64", "macos")),
        href if href.ends_with("windows.7z") => Ok(("x86_64", "windows")),
        _ => Err(crate::Error::InvalidDownloadUrl(href.into())),
    }
}
