use gdtk_gvm::VersionManager;

use crate::cli::utils::prompt_local_version;

pub struct GodotUninstallCommand;

impl tapcli::Command for GodotUninstallCommand {
    type Error = anyhow::Error;

    async fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let mut manager = VersionManager::load()?;
        let version = prompt_local_version(&manager)?.to_owned();

        let Some(previous) = manager.remove_version(&version) else {
            anyhow::bail!("Godot {} isn't installed.", &version)
        };

        std::fs::remove_dir_all(previous.path)?;

        if let Some(default) = &manager.inner.default
            && default == &version
        {
            manager.inner.default = None;
            std::fs::remove_file(gdtk_paths::default_godot_path()?)?;
        }

        manager.save()?;

        println!("Godot {} uninstalled!", &version);

        Ok(())
    }
}
