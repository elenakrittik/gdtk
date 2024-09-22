#[cynic::schema("github")]
mod schema {}
pub mod release_assets;
pub mod releases;

pub const GITHUB_GRAPHQL_API: &str = "https://api.github.com/graphql";
