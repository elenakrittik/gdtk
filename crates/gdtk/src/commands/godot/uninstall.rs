pub async fn run(version: String) -> anyhow::Result<()> {
    let mut version_manager = gdtk_gvm::VersionManager::load()?;

    let previous = match version_manager.remove_version(&version) {
        Some(previous) => previous,
        None => anyhow::bail!("Godot {version} isn't installed."),
    };

    std::fs::remove_dir_all(previous.path)?;

    version_manager.save()?;

    println!("Godot {version} uninstalled!");

    Ok(())
}
