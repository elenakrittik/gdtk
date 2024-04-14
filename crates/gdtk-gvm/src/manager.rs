use serde::Deserialize;

pub struct VersionManager {
    pub versions: crate::types::Versions,
}

impl VersionManager {
    pub fn load() -> Result<Self, crate::Error> {
        let content = std::fs::read_to_string(gdtk_paths::versions_toml_path()?)?;
        let versions = crate::types::Versions::deserialize(toml::Deserializer::new(&content))?;

        Ok(Self { versions })
    }

    pub fn save(&self) -> Result<(), crate::Error> {
        let contents = toml::to_string_pretty(&self.versions)?;
        let path = gdtk_paths::versions_toml_path()?;

        std::fs::write(path, contents)?;

        Ok(())
    }

    pub fn versionings(&self) -> Vec<versions::Versioning> {
        self.versions
            .versions
            .keys()
            .filter_map(|v| versions::Versioning::new(v))
            .collect()
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
