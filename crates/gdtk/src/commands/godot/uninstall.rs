pub async fn run(version: &String) -> anyhow::Result<()> {
    let mut local = gdtk_gvm::read_local_versions()?;

    let old = local.remove(version);

    if old.is_none() {
        anyhow::bail!("This version isn't installed.");
    }

    std::fs::remove_dir_all(old.unwrap().as_str().unwrap())?;

    gdtk_gvm::write_local_versions(local)?;

    println!("Godot {} uninstalled!", version);

    Ok(())
}
