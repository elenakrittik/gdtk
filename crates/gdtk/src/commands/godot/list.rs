use tabled::settings::{format::Format, object::Cell, Style};

pub fn run() -> anyhow::Result<()> {
    let mut version_manager = gdtk_gvm::VersionManager::load()?;

    if version_manager.is_empty() {
        eprintln!("No versions installed.");
        return Ok(());
    }

    if let Some(default) = version_manager.versions.default {
        let def_ver = version_manager.versions.versions.remove(&default).unwrap();
        version_manager
            .versions
            .versions
            .insert(default + " (default)", def_ver);
    }

    let mut table = tabled::Table::new(version_manager.versions.versions);
    table.with(Style::modern_rounded());
    table.modify(Cell::new(0, 0), Format::content(|_| "Version".to_owned()));

    eprintln!("{table}");

    Ok(())
}
