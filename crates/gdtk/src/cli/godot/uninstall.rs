use gdtk_gvm::{versions::Versioning, VersionManager};

use crate::cli::utils::ParserExt;

pub struct GodotUninstallCommand {
    pub version: Versioning,
    pub manager: VersionManager,
}

impl tapcli::Command for GodotUninstallCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let manager = VersionManager::load()?;
        let version = parser.next_godot_version(manager.versionings())?;

        Ok(Self { version, manager })
    }

    async fn run(mut self) -> Result<Self::Output, Self::Error> {
        let version_string = self.version.to_string();

        let previous = match self.manager.remove_version(&version_string) {
            Some(previous) => previous,
            None => anyhow::bail!("Godot {version_string} isn't installed."),
        };

        std::fs::remove_dir_all(previous.path)?;

        if let Some(default) = &self.manager.inner.default
            && default == &version_string
        {
            self.manager.inner.default = None;
            std::fs::remove_file(gdtk_paths::default_godot_path()?)?;
        }

        self.manager.save()?;

        println!("Godot {version_string} uninstalled!");

        Ok(())
    }
}
