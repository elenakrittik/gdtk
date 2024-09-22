use super::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct ReleasesQueryVariables<'a> {
    pub after: Option<&'a str>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ReleasesQueryVariables")]
pub struct ReleasesQuery {
    #[arguments(owner: "godotengine", name: "godot-builds")]
    pub repository: Option<Repository>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "ReleasesQueryVariables")]
pub struct Repository {
    #[arguments(first: 100, after: $after)]
    pub releases: ReleaseConnection,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ReleaseConnection {
    pub nodes: Option<Vec<Option<Release>>>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug, PartialEq)]
pub struct Release {
    pub tag_name: String,
    pub is_prerelease: bool,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}
