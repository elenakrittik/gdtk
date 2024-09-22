use serde::Deserialize;

pub struct VersionManager {
    pub inner: crate::types::VersionsToml,
}

impl VersionManager {
    /// Load the `versions.toml` file.
    pub fn load() -> Result<Self, crate::Error> {
        let content = std::fs::read_to_string(gdtk_paths::versions_toml_path()?)?;
        let versions = crate::types::VersionsToml::deserialize(toml::Deserializer::new(&content))?;

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
    pub fn versions(&self) -> Vec<&str> {
        self.inner.versions.keys().map(AsRef::as_ref).collect()
    }

    /// Inserts a version and returns whether this version was previously in there.
    pub fn add_version(&mut self, version: &str, data: crate::types::Version) -> bool {
        self.inner
            .versions
            .insert(version.to_string(), data)
            .is_some()
    }

    pub fn get_version(&self, version: &str) -> Option<&crate::types::Version> {
        self.inner.versions.get(version)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.versions.is_empty()
    }

    pub fn remove_version(&mut self, version: &str) -> Option<crate::types::Version> {
        self.inner.versions.remove(version)
    }
}
