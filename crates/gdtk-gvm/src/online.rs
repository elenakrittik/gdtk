use ureq::http::{
    header::{ACCEPT, AUTHORIZATION},
    Uri,
};

use crate::{
    api::{fetch_assets_url, fetch_releases_url, octocrab::get_next_link, Release, ReleaseAsset},
    version::OnlineVersion,
};

pub fn fetch_versions() -> Result<Vec<OnlineVersion>, crate::Error> {
    let mut output = vec![];
    let mut link = fetch_releases_url();

    loop {
        let (releases, new_link) = send_api_request::<Vec<Release>>(link)?;

        output.extend(releases.into_iter().map(OnlineVersion::from));

        if let Some(new_link) = new_link {
            link = new_link;
        } else {
            break;
        }
    }

    output.sort_unstable_by(|v1, v2| v2.as_ordered().cmp(v1.as_ordered()));

    Ok(output)
}

pub fn fetch_version_assets(release_id: u32) -> Result<Vec<ReleaseAsset>, crate::Error> {
    let mut output = vec![];
    let mut link = fetch_assets_url(release_id);

    loop {
        let (assets, new_link) = send_api_request::<Vec<ReleaseAsset>>(link)?;

        output.extend(assets);

        if let Some(new_link) = new_link {
            link = new_link;
        } else {
            break;
        }
    }

    Ok(output)
}

fn retrieve_token() -> Option<String> {
    let command = std::process::Command::new("gh")
        .arg("auth")
        .arg("token")
        .output()
        .ok()?;

    let std::process::Output {
        status,
        stdout: output,
        ..
    } = command;

    if status.success() {
        String::from_utf8(output.trim_ascii().to_owned()).ok()
    } else {
        None
    }
}

fn send_api_request<T>(url: Uri) -> Result<(T, Option<Uri>), crate::Error>
where
    T: serde::de::DeserializeOwned,
{
    let mut request = ureq::get(url)
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");

    if let Some(token) = retrieve_token() {
        request = request.header(AUTHORIZATION, format!("Bearer {token}"));
    }

    let response = request.call()?;

    let next_link = get_next_link(response.headers());
    let data = response.into_body().read_json::<T>()?;

    Ok((data, next_link))
}
