use octocrab::models::repos::Release;
use versions::Version as SemanticVersion;

#[derive(PartialEq)]
pub struct Version {
    semantic: SemanticVersion,
    data: Release, // TODO: this thing is *FAT*. we should use graphql api via cynic and only include what we need
}

impl Version {
    pub fn is_dev(&self) -> bool {
        self.data.prerelease
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        &self.data.tag_name
    }
}

impl From<Release> for Version {
    fn from(value: Release) -> Self {
        Self {
            semantic: SemanticVersion::new(&value.tag_name).expect("a valid version"),
            data: value,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.semantic.partial_cmp(&other.semantic)
    }
}
