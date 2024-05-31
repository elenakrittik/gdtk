use crate::cli::{godot::select_version, utils::ParserExt};

pub struct GodotRunCommand {
    pub version: Option<String>,
}

impl tapcli::Command for GodotRunCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let version = parser.next_value_maybe()?;

        Ok(Self { version })
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let version_manager = gdtk_gvm::VersionManager::load()?;

        // Case 1. A version was specified
        let version = if let Some(version) = self.version {
            // Construct a Versioning from the version string
            let version = gdtk_gvm::versions::Versioning::new(&version).unwrap();
            // Get the best match from installed versions
            let mut versions =
                gdtk_gvm::utils::coerce_version(version, version_manager.versionings())?;
            // Allow the user to select the desired version if there are multiple matches
            let idx = select_version(&versions, "Select version to run")?;

            versions.swap_remove(idx)

        // Case 2. No version was specified, and there is no default
        } else {
            let mut versions = version_manager.versionings();
            let idx = select_version(&versions, "Select version to run")?;

            versions.swap_remove(idx)
        };

        let Some(version) = version_manager.get_version(&version.to_string()) else {
            eprintln!("Godot {version} is not installed.");
            return Ok(());
        };

        let path = if version.path.join("godot").exists() {
            // New-style installations
            version.path.join("godot")
        } else {
            // Old-style installations
            version.path.read_dir()?
                .filter_map(|p| p.ok())
                .filter(|p| p.file_name().to_str().unwrap().contains("Godot"))
                .map(|p| p.path()).next()
                .ok_or(anyhow::anyhow!("This Godot installation appears to be broken. Try uninstalling and installing again."))?
        };

        let mut child = std::process::Command::new(path).spawn()?;

        child.wait()?;

        Ok(())
    }
}
