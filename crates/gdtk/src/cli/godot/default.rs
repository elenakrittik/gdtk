use gdtk_gvm::VersionManager;

use crate::cli::{godot::symlink_default_version, utils::prompt_local_version};

pub struct GodotDefaultCommand;

impl tapcli::Command for GodotDefaultCommand {
    type Error = anyhow::Error;

    fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let manager = VersionManager::load()?;

        if manager.is_empty() {
            eprintln!("No versions installed. Install one by running `gdtk godot install`.");
            return Ok(());
        }

        let version = prompt_local_version(&manager)?;

        symlink_default_version(&version.path())?;

        eprintln!("Version {} set as default!", version);

        Ok(())
    }
}
