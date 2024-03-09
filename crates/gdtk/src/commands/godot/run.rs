pub async fn run(version: &String) -> anyhow::Result<()> {
    let local = gdtk_gvm::read_local_versions()?;
    let path = local.get(version);

    match path {
        Some(path) => {
            let path = std::path::PathBuf::from(path.as_str().unwrap());
            let path = path.read_dir()?
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
