pub async fn run(version: &String) -> anyhow::Result<()> {
    let exists = gdtk_gvm::online::check_version_exists(version.to_owned()).await?;

    if !exists {
        anyhow::bail!("{version} is an unknown Godot version.");
    }

    let mut local = gdtk_gvm::read_local_versions()?;
    let target_dir = gdtk_gvm::godots_path()?.join(version);

    let old = local.insert(
        version.clone(),
        gdtk_gvm::toml::Value::String(target_dir.display().to_string()),
    );

    if old.is_some() {
        anyhow::bail!("{version} is already installed.");
    }

    gdtk_gvm::ensure_godots()?;

    let source = std::io::Cursor::new(gdtk_gvm::online::download_version_zip(version).await?);

    zip_extract::extract(source, &target_dir, true)?;

    // Enable self-contained mode.
    std::fs::File::create(target_dir.join("._sc_"))?;

    gdtk_gvm::write_local_versions(local)?;

    println!("Installed Godot {}!", version);

    Ok(())
}
