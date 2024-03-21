const GODOT_DOWNLOADS_ROOT: &str = "https://godotengine.org/download/archive/";

pub struct VersionDownloadUrls {
    pub android64: String,
    pub windows32: String,
    pub windows64: String,
    pub linux32: String,
    pub linux64: String,
    pub linuxarm: String,
    pub darwin64: String,
    pub darwinarm: String,
    pub darwinany: String,
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

pub async fn version_download_urls(_version: &str) -> Result<VersionDownloadUrls, crate::Error> {
    todo!()
}
