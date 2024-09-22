use super::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct ReleaseAssetsQueryVariables<'a> {
    pub after: Option<&'a str>,
    pub tag_name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ReleaseAssetsQueryVariables")]
pub struct ReleaseAssetsQuery {
    #[arguments(owner: "godotengine", name: "godot-builds")]
    pub repository: Option<Repository>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "ReleaseAssetsQueryVariables")]
pub struct Repository {
    #[arguments(tagName: $tag_name)]
    pub release: Option<Release>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "ReleaseAssetsQueryVariables")]
pub struct Release {
    #[arguments(first: 100, after: $after)]
    pub release_assets: ReleaseAssetConnection,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ReleaseAssetConnection {
    pub nodes: Option<Vec<Option<ReleaseAsset>>>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ReleaseAsset {
    pub name: String,
    pub download_url: Uri,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "URI")]
pub struct Uri(pub String);
