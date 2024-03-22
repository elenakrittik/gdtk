use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Versions {
    pub versions: ahash::AHashMap<String, Version>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, tabled::Tabled)]
pub struct Version {
    #[tabled(display_with("Self::display_version", self))]
    #[tabled(rename = "Location")]
    pub path: PathBuf,
}

impl Version {
    pub fn display_version(&self) -> String {
        self.path.display().to_string()
    }
}
