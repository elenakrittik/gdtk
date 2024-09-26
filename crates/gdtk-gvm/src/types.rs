use std::fmt::Display;

use gdtk_paths::camino::Utf8PathBuf;

/// Represents a `versions.toml` file.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct LocalVersions(pub Vec<LocalVersion>);

#[derive(Debug, Clone, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, tabled::Tabled)]
pub struct LocalVersion {
    #[tabled(rename = "Version")]
    #[tabled(display_with("ToString::to_string", self))]
    pub name: String,
    #[tabled(rename = "Location")]
    pub path: String,
    #[tabled(skip)]
    pub mono: bool,
}

impl LocalVersion {
    pub fn path(&self) -> Utf8PathBuf {
        Utf8PathBuf::from(&self.path)
    }
}

impl Display for LocalVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.mono { " (mono)" } else { "" };

        write!(f, "{}{}", self.name, suffix)
    }
}
