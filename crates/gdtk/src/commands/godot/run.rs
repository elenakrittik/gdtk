pub async fn run(version: Option<String>) -> anyhow::Result<()> {
    let version_manager = gdtk_gvm::VersionManager::load()?;

    // Case 1. A version was specified
    let version = if let Some(version) = version {
        // Construct a Versioning from the version string
        let version = gdtk_gvm::versions::Versioning::new(&version).unwrap();
        // Get the best match from installed versions
        let mut versions = gdtk_gvm::utils::coerce_version(version, version_manager.versionings())?;
        // Allow the user to select the desired version if there are multiple matches
        let idx = crate::commands::godot::select_version(&versions, "Select version to run")?;

        versions.swap_remove(idx)

    // Case 2. No version was specified, and there is no default
    } else {
        let mut versions = version_manager.versionings();
        let idx = crate::commands::godot::select_version(&versions, "Select version to run")?;

        versions.swap_remove(idx)
    };

    let path = version_manager.get_version(&version.to_string());

    match path {
        Some(version) => {
            let path = version.path.read_dir()?
                .filter_map(|p| p.ok())
                .filter(|p| p.file_name().to_str().unwrap().contains("Godot"))
                .map(|p| p.path()).next()
                .ok_or(anyhow::anyhow!("This Godot installation appears to be broken. Try uninstalling and installing again."))?;

            let mut child = std::process::Command::new(path).spawn()?;

            child.wait()?;
        }
        None => eprintln!("Godot {version} is not installed."),
    }

    Ok(())
}
