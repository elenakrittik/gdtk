pub async fn run(version: String) -> anyhow::Result<()> {
    let version_manager = gdtk_gvm::VersionManager::load()?;
    let path = version_manager.get_version(&version);

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
