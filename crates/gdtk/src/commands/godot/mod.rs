pub mod install;
pub mod list;
pub mod run;
pub mod uninstall;

pub(crate) fn select_version(
    versions: &[gdtk_gvm::versions::Versioning],
    prompt: &str,
) -> anyhow::Result<usize> {
    if versions.is_empty() {
        anyhow::bail!("No matching versions found.")
    } else if versions.len() == 1 {
        Ok(0)
    } else {
        Ok(gdtk_dialoguer::FuzzySelect::new()
            .with_theme(&gdtk_dialoguer::theme::ColorfulTheme::default())
            .add_items(versions)
            .highlight_matches(true)
            .with_default(0)
            .with_prompt(prompt)
            .interact()?)
    }
}

fn symlink_default_version(version_folder: &std::path::Path) -> std::io::Result<()> {
    let original = version_folder.join("godot");
    let link = gdtk_paths::default_godot_path()?;

    if link.exists() {
        std::fs::remove_file(&link)?;
    }

    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(original, link)?;
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(original, link)?;
    }

    Ok(())
}
