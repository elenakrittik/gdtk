use std::{fmt::Display, path::PathBuf};

/// Represents a `versions.toml` file.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VersionsToml {
    pub versions: Vec<DiskVersion>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, tabled::Tabled)]
pub struct DiskVersion {
    #[tabled(rename = "Version")]
    #[tabled(display_with("Self::display_name", self))]
    pub name: String,
    #[tabled(display_with("Self::display_path", self))]
    #[tabled(rename = "Location")]
    pub path: PathBuf,
    #[tabled(skip)]
    pub mono: bool,
}

impl DiskVersion {
    fn display_name(&self) -> String {
        self.to_string()
    }

    fn display_path(&self) -> String {
        self.path.display().to_string()
    }
}

impl Display for DiskVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.mono { " (mono)" } else { "" };

        write!(f, "{}{}", self.name, suffix)
    }
}
