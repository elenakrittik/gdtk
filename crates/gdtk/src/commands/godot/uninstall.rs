pub async fn run(version: String) -> anyhow::Result<()> {
    let mut version_manager = gdtk_gvm::VersionManager::load()?;
    let versioning = gdtk_gvm::versions::Versioning::new(&version)
        .ok_or(anyhow::anyhow!("Invalid version: {version}"))?;

    let versions = gdtk_gvm::utils::coerce_version(versioning, version_manager.versionings())?;
    let idx = crate::commands::godot::select_version(&versions, "Select version to uninstall")?;

    let version = versions[idx].to_string();

    let previous = match version_manager.remove_version(&version) {
        Some(previous) => previous,
        None => anyhow::bail!("Godot {version} isn't installed."),
    };

    std::fs::remove_dir_all(previous.path)?;

    version_manager.save()?;

    println!("Godot {version} uninstalled!");

    Ok(())
}
