// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use url::{Host, Origin, Url};

use super::Source;

pub fn source(upstream: &Url) -> Option<Source> {
    if upstream.origin() != Origin::Tuple("https".to_string(), Host::Domain("github.com".to_string()), 443) {
        return None;
    }
    if let Some(segs) = upstream.path_segments() {
        let params = url_parameters(segs)?;
        Some(Source {
            name: params.project.to_lowercase(),
            version: params.version,
            homepage: format!("https://github.com/{}/{}", params.owner, params.project),
        })
    } else {
        None
    }
}

struct UrlParameters {
    owner: String,
    project: String,
    version: String,
}

fn url_parameters(path: std::str::Split<'_, char>) -> Option<UrlParameters> {
    let elements: Vec<&str> = path.collect();

    let owner = elements.first()?;
    let project = elements.get(1)?;
    let intermediate_path = elements.get(2..elements.len() - 2)?;
    if intermediate_path != vec!["archive", "ref", "tags"] && intermediate_path != vec!["releases", "download"] {
        return None;
    }
    let version = elements.last()?.split('.').next()?;
    Some(UrlParameters {
        owner: owner.to_string(),
        project: project.to_string(),
        version: version.to_string(),
    })
}
