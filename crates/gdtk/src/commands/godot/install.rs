pub async fn run(version: &String) -> anyhow::Result<()> {
    let exists = gdtk_gvm::online::check_version_exists(version.to_owned()).await?;

    if !exists {
        anyhow::bail!("{version} is an unknown Godot version.");
    }

    let local = gdtk_gvm::get_local_versions()?;

    if local.contains_key(version) {
        anyhow::bail!("{version} is already installed.");
    }

    gdtk_gvm::ensure_godots()?;

    let target_dir = gdtk_gvm::godots_path()?.join(version);
    let source = std::io::Cursor::new(gdtk_gvm::online::download_version_zip(version).await?);

    zip_extract::extract(source, &target_dir, true)?;

    println!("Installed Godot {}!", version);

    Ok(())
}
