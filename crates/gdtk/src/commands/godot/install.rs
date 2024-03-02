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

    let _data_dir = gdtk_gvm::godots_path()?;

    println!("{}", _data_dir.display());

    let url = gdtk_gvm::online::get_version_download_url(version)?;

    dbg!(url);

    Ok(())
}
