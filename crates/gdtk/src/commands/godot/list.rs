use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Row, Table,
};

pub async fn run(online: &bool, unsupported: &bool, dev: &bool, unsupported_dev: &bool) -> anyhow::Result<()> {
    if *online {
        show_online_versions(*unsupported, *dev, *unsupported_dev).await?;
    } else {
        show_local_versions()?;
    }

    Ok(())
}

fn show_local_versions() -> anyhow::Result<()> {
    gdtk_gvm::ensure_versions()?;

    let versions = gdtk_gvm::get_local_versions()?;

    if versions.is_empty() {
        println!("No versions installed.");
        return Ok(());
    }

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["Version", "Path"])
        .add_rows(
            versions
                .into_iter()
                .map(|(v, p)| Row::from([v, p])),
        );

    println!("{table}");

    Ok(())
}

async fn show_online_versions(unsupported: bool, dev: bool, unsupported_dev: bool) -> anyhow::Result<()> {
    print!("Fetching versions..");
    let versions = gdtk_gvm::online::fetch_versions(gdtk_gvm::online::FetchVersionsOptions { unsupported, dev, unsupported_dev }).await?;

    println!("\rAvailable versions:");

    for ver in gdtk_gvm::utils::sort_versions(versions) {
        println!("  {}", gdtk_gvm::utils::format_version(ver));
    }

    Ok(())
}
