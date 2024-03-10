use crate::Error;

// TODO: switch to godotengine/godot-builds
const GODOT_DOWNLOADS_ROOT: &str = "https://godotengine.org/download/archive/";

pub async fn fetch_versions() -> Result<Vec<versions::Versioning>, Error> {
    let text = reqwest::get(GODOT_DOWNLOADS_ROOT).await?.text().await?;
    let html = scraper::Html::parse_document(&text);
    let selector = scraper::Selector::parse(".archive-version > h4").unwrap();

    let versions = html
        .select(&selector)
        .filter_map(|elem| elem.attr("id"))
        .filter_map(|ver| versions::Versioning::new(ver))
        .rev()
        .collect();

    Ok(versions)
}
