use serde::Deserialize;

pub struct VersionManager {
    pub versions: crate::types::Versions,
}

impl VersionManager {
    pub fn load() -> Result<Self, crate::Error> {
        let content = std::fs::read_to_string(crate::utils::versions_toml_path()?)?;
        let versions = crate::types::Versions::deserialize(toml::Deserializer::new(&content))?;

        Ok(Self { versions })
    }

    pub fn save(&self) -> Result<(), crate::Error> {
        crate::utils::write_local_versions(&self.versions)?;

        Ok(())
    }

    pub fn add_version(&mut self, version: String, data: crate::types::Version) -> bool {
        self.versions.versions.insert(version, data).is_some()
    }

    pub fn get_version(&self, version: &str) -> Option<&crate::types::Version> {
        self.versions.versions.get(version)
    }

    pub fn is_empty(&self) -> bool {
        self.versions.versions.is_empty()
    }

    pub fn remove_version(&mut self, version: &str) -> Option<crate::types::Version> {
        self.versions.versions.remove(version)
    }
}
