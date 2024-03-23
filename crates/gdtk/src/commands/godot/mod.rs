pub mod install;
pub mod list;
pub mod run;
pub mod uninstall;

pub(crate) fn select_version(versions: &[gdtk_gvm::versions::Versioning], prompt: &str) -> anyhow::Result<usize> {
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
