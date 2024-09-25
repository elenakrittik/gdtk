use std::process::Command;

use gdtk_gvm::VersionManager;

use crate::cli::utils::prompt_local_version;

pub struct GodotRunCommand;

impl tapcli::Command for GodotRunCommand {
    type Error = anyhow::Error;

    async fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let manager = VersionManager::load()?;
        let version = prompt_local_version(&manager)?;

        let Some(version) = manager.get_version(&version.name, version.mono) else {
            eprintln!("Godot {} is not installed.", version);
            return Ok(());
        };

        let mut child = Command::new(version.path().join("godot")).spawn()?;

        child.wait()?;

        Ok(())
    }
}
