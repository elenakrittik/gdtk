use std::fmt::Display;

use versions::Version as SemanticVersion;

use crate::queries::releases::Release as GraphQLRelease;

#[derive(PartialEq, Debug)]
pub struct Version {
    semantic: SemanticVersion,
    data: GraphQLRelease,
}

impl Version {
    pub fn name(&self) -> &str {
        &self.data.tag_name
    }

    pub fn is_dev(&self) -> bool {
        self.data.is_prerelease
    }

    pub fn as_ordered(&self) -> &impl Ord {
        &self.semantic
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.data.tag_name)
    }
}

impl From<GraphQLRelease> for Version {
    fn from(value: GraphQLRelease) -> Self {
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
