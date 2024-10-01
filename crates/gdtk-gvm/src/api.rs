use ureq::http::Uri;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Release {
    pub id: u32,
    pub tag_name: String,
    pub prerelease: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}

pub fn fetch_releases_url() -> Uri {
    Uri::from_static("https://api.github.com/repos/godotengine/godot-builds/releases?first=100")
}

pub fn fetch_assets_url(release_id: u32) -> Uri {
    format!("https://api.github.com/repos/godotengine/godot-builds/releases/{release_id}/assets?first=100")
        .parse()
        .unwrap()
}

// `get_links` borrowed from octocrab under the MIT license
// https://github.com/XAMPPRocky/octocrab/blob/9fbf59c42a4c267204980955a1c70b80ee919964/LICENCE-MIT
// https://github.com/XAMPPRocky/octocrab/blob/9fbf59c42a4c267204980955a1c70b80ee919964/src/page.rs#L229-L284
pub(crate) mod octocrab {
    use std::str::FromStr;

    use ureq::http::{header::HeaderMap, Uri};

    pub(crate) fn get_next_link(headers: &HeaderMap) -> Option<Uri> {
        let mut next = None;

        if let Some(link) = headers.get("Link") {
            let links = link.to_str().unwrap();

            for url_with_params in links.split(',') {
                let mut url_and_params = url_with_params.split(';');
                let url = url_and_params
                    .next()
                    .expect("url to be present")
                    .trim()
                    .trim_start_matches('<')
                    .trim_end_matches('>');

                for param in url_and_params {
                    if let Some((name, value)) = param.trim().split_once('=') {
                        let value = value.trim_matches('\"');

                        if name == "rel" && value == "next" {
                            next = Some(Uri::from_str(url).unwrap())
                        }
                    }
                }
            }
        }

        next
    }
}
