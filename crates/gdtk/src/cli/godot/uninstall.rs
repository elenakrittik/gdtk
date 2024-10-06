use gdtk_gvm::VersionManager;

use crate::cli::utils::prompt_local_version;

pub struct GodotUninstallCommand;

impl tapcli::Command for GodotUninstallCommand {
    type Error = anyhow::Error;

    fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let mut manager = VersionManager::load()?;

        if manager.is_empty() {
            eprintln!("No versions installed.");
            return Ok(());
        }

        let version = prompt_local_version(&manager)?.clone();

        let Some(previous) = manager.remove_version(&version.name, version.mono) else {
            anyhow::bail!("Godot {} isn't installed.", &version)
        };

        std::fs::remove_dir_all(previous.path)?;

        manager.save()?;

        println!("Godot {} uninstalled!", &version);

        Ok(())
    }
}
