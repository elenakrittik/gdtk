use std::io::Read;

use crate::types::{LocalVersion, LocalVersions};

pub struct VersionManager {
    inner: LocalVersions,
}

impl VersionManager {
    /// Load the `versions.toml` file.
    pub fn load() -> Result<Self, crate::Error> {
        let mut content = vec![];
        let mut file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(crate::utils::versions_toml_path()?)?;

        file.read_to_end(&mut content)?;

        let versions = rkyv::from_bytes::<_, rkyv::rancor::Error>(&content)?;

        Ok(Self { inner: versions })
    }

    /// Save the `versions.toml` file.
    pub fn save(&self) -> Result<(), crate::Error> {
        let contents = rkyv::to_bytes::<rkyv::rancor::Error>(&self.inner)?;
        let path = crate::utils::versions_toml_path()?;

        std::fs::write(path, contents)?;

        Ok(())
    }

    /// Get all installed versions.
    #[inline]
    pub fn installed(&self) -> &[LocalVersion] {
        &self.inner.0
    }

    /// Try to find an installed version.
    pub fn get_version(&self, name: &str, mono: bool) -> Option<&crate::types::LocalVersion> {
        self.installed()
            .iter()
            .find(|v| v.name == name && v.mono == mono)
    }

    /// Insert a version.
    pub fn add_version(&mut self, data: crate::types::LocalVersion) {
        debug_assert!(self.get_version(&data.name, data.mono).is_none());

        self.inner.0.push(data);
    }

    pub fn is_empty(&self) -> bool {
        self.installed().len() == 0
    }

    pub fn remove_version(&mut self, name: &str, mono: bool) -> Option<crate::types::LocalVersion> {
        let idx = self
            .installed()
            .iter()
            .position(|v| v.name == name && v.mono == mono)?;

        Some(self.inner.0.swap_remove(idx))
    }
}
