use std::fmt::Display;

/// Represents a `versions.toml` file.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct VersionsToml(pub Vec<DiskVersion>);

#[derive(Debug, Clone, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, tabled::Tabled)]
pub struct DiskVersion {
    #[tabled(rename = "Version")]
    #[tabled(display_with("ToString::to_string", self))]
    pub name: String,
    #[tabled(rename = "Location")]
    pub path: String,
    #[tabled(skip)]
    pub mono: bool,
}

impl Display for DiskVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.mono { " (mono)" } else { "" };

        write!(f, "{}{}", self.name, suffix)
    }
}
