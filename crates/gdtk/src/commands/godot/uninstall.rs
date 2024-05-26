pub async fn run(version: Option<String>) -> anyhow::Result<()> {
    let mut version_manager = gdtk_gvm::VersionManager::load()?;

    let version = match version {
        Some(v) => {
            let versioning = gdtk_gvm::versions::Versioning::new(&v)
                .ok_or(anyhow::anyhow!("Invalid version: {v}"))?;

            let versions =
                gdtk_gvm::utils::coerce_version(versioning, version_manager.versionings())?;
            let idx =
                crate::commands::godot::select_version(&versions, "Select version to uninstall")?;

            versions[idx].to_string()
        }
        None => prompt_version(version_manager.versionings())?,
    };

    let previous = match version_manager.remove_version(&version) {
        Some(previous) => previous,
        None => anyhow::bail!("Godot {version} isn't installed."),
    };

    std::fs::remove_dir_all(previous.path)?;

    if let Some(default) = &version_manager.versions.default
        && default == &version
    {
        version_manager.versions.default = None;
        std::fs::remove_file(gdtk_paths::default_godot_path()?)?;
    }

    version_manager.save()?;

    println!("Godot {version} uninstalled!");

    Ok(())
}

fn prompt_version(versions: Vec<gdtk_gvm::versions::Versioning>) -> anyhow::Result<String> {
    if versions.is_empty() {
        anyhow::bail!("No versions installed.");
    } else if versions.len() == 1 {
        Ok(versions[0].to_string())
    } else {
        let idx = gdtk_dialoguer::FuzzySelect::new()
            .with_theme(&gdtk_dialoguer::theme::ColorfulTheme::default())
            .add_items(&versions)
            .highlight_matches(true)
            .with_default(0)
            .with_prompt("Select version to uninstall")
            .interact()?;

        Ok(versions[idx].to_string())
    }
}
