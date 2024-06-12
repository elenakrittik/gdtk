use gdtk_gvm::{versions::Versioning, VersionManager};

use crate::cli::utils::ParserExt;

pub struct GodotRunCommand {
    pub version: Versioning,
    pub manager: VersionManager,
}

impl tapcli::Command for GodotRunCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let manager = VersionManager::load()?;
        let version = parser.next_godot_version(manager.versionings())?;

        Ok(Self { version, manager })
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let Some(version) = self.manager.get_version(&self.version.to_string()) else {
            eprintln!("Godot {} is not installed.", self.version);
            return Ok(());
        };

        let mut child = std::process::Command::new(version.path.join("godot")).spawn()?;

        child.wait()?;

        Ok(())
    }
}
