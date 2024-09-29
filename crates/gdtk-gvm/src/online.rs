use cynic::QueryBuilder;

use crate::{
    queries::{
        release_assets::{ReleaseAsset, ReleaseAssetsQuery, ReleaseAssetsQueryVariables},
        releases::{ReleasesQuery, ReleasesQueryVariables},
        GITHUB_GRAPHQL_API,
    },
    version::OnlineVersion,
};

pub fn fetch_versions() -> Result<Vec<OnlineVersion>, crate::Error> {
    let mut output = vec![];
    let mut cursor_end = None;

    loop {
        let op = ReleasesQuery::build(ReleasesQueryVariables {
            after: cursor_end.as_deref(),
        });

        let response = send_graphql_request(op)?.repository.unwrap().releases;

        output.extend(
            response
                .nodes
                .unwrap()
                .into_iter()
                .flatten()
                .map(OnlineVersion::from),
        );

        if response.page_info.has_next_page {
            cursor_end = response.page_info.end_cursor;
        } else {
            break;
        }
    }

    output.sort_unstable_by(|v1, v2| v2.as_ordered().cmp(v1.as_ordered()));

    Ok(output)
}

pub fn fetch_version_assets(tag_name: &str) -> Result<Vec<ReleaseAsset>, crate::Error> {
    let mut output = vec![];
    let mut cursor_end = None;

    loop {
        let op = ReleaseAssetsQuery::build(ReleaseAssetsQueryVariables {
            after: cursor_end.as_deref(),
            tag_name,
        });

        let response = send_graphql_request(op)?
            .repository
            .unwrap()
            .release
            .unwrap()
            .release_assets;

        output.extend(response.nodes.unwrap().into_iter().flatten());

        if response.page_info.has_next_page {
            cursor_end = response.page_info.end_cursor;
        } else {
            break;
        }
    }

    Ok(output)
}

fn retrieve_token() -> Result<String, crate::Error> {
    let command = std::process::Command::new("gh")
        .arg("auth")
        .arg("token")
        .output()?;

    let std::process::Output {
        status,
        stdout: output,
        ..
    } = command;

    if status.success() {
        Ok(String::from_utf8(output.trim_ascii().to_owned())
            .map_err(|_| crate::Error::TokenRetrievalError)?)
    } else {
        Err(crate::Error::TokenRetrievalError)
    }
}

fn send_graphql_request<Q, V>(op: cynic::Operation<Q, V>) -> Result<Q, crate::Error>
where
    Q: serde::de::DeserializeOwned,
    V: serde::Serialize,
{
    let token = format!("Bearer {}", retrieve_token()?);

    let data = ureq::post(GITHUB_GRAPHQL_API)
        .header(ureq::http::header::AUTHORIZATION, &token)
        .send_json(op)?
        .into_body()
        .read_json::<cynic::GraphQlResponse<Q>>()?
        .data
        .unwrap();

    Ok(data)
}
