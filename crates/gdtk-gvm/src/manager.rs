use crate::types::{DiskVersion, VersionsToml};

pub struct VersionManager {
    inner: VersionsToml,
}

impl VersionManager {
    /// Load the `versions.toml` file.
    pub fn load() -> Result<Self, crate::Error> {
        let content = std::fs::read_to_string(gdtk_paths::versions_toml_path()?)?;
        let versions = toml::from_str(&content)?;

        Ok(Self { inner: versions })
    }

    /// Save the `versions.toml` file.
    pub fn save(&self) -> Result<(), crate::Error> {
        let contents = toml::to_string_pretty(&self.inner)?;
        let path = gdtk_paths::versions_toml_path()?;

        std::fs::write(path, contents)?;

        Ok(())
    }

    /// Get all installed versions.
    #[inline]
    pub fn installed(&self) -> &[DiskVersion] {
        &self.inner.versions
    }

    /// Try to find an installed version.
    pub fn get_version(&self, name: &str, mono: bool) -> Option<&crate::types::DiskVersion> {
        self.installed()
            .iter()
            .find(|v| v.name == name && v.mono == mono)
    }

    /// Insert a version.
    pub fn add_version(&mut self, data: crate::types::DiskVersion) {
        debug_assert!(self.get_version(&data.name, data.mono).is_none());

        self.inner.versions.push(data);
    }

    pub fn is_empty(&self) -> bool {
        self.installed().len() == 0
    }

    pub fn remove_version(&mut self, name: &str, mono: bool) -> Option<crate::types::DiskVersion> {
        let idx = self
            .installed()
            .iter()
            .position(|v| v.name == name && v.mono == mono)?;

        Some(self.inner.versions.swap_remove(idx))
    }
}
