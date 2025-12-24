use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum ResolveVersionError {
    #[snafu(display("failed to query crates.io for crate '{crate_name}'"))]
    CratesIoQuery {
        crate_name: String,
        source: reqwest::Error,
    },
    #[snafu(display("failed to parse semver requirement '{requirement}'"))]
    SemverParse {
        requirement: String,
        source: semver::Error,
    },
    #[snafu(display("no version matching '{requirement}' found for crate '{crate_name}'"))]
    NoMatchingVersion {
        crate_name: String,
        requirement: String,
    },
}

pub async fn resolve_version(
    crate_name: &str,
    version_req: Option<&str>,
) -> Result<String, ResolveVersionError> {
    use resolve_version_error::*;
    use snafu::ResultExt;

    let client = reqwest::Client::new();
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let resp = client
        .get(&url)
        .header("User-Agent", "rust-crate-src")
        .send()
        .await
        .context(CratesIoQuerySnafu { crate_name })?
        .json::<CrateResponse>()
        .await
        .context(CratesIoQuerySnafu { crate_name })?;

    match version_req {
        None => Ok(resp.crate_info.max_version),
        Some(req_str) => {
            let req = semver::VersionReq::parse(req_str).context(SemverParseSnafu {
                requirement: req_str,
            })?;
            resp.versions
                .iter()
                .filter_map(|v| semver::Version::parse(&v.num).ok())
                .filter(|v| req.matches(v))
                .max()
                .map(|v| v.to_string())
                .ok_or_else(|| {
                    NoMatchingVersionSnafu {
                        crate_name,
                        requirement: req_str,
                    }
                    .build()
                })
        }
    }
}

#[derive(serde::Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
    versions: Vec<VersionInfo>,
}

#[derive(serde::Deserialize)]
struct CrateInfo {
    max_version: String,
}

#[derive(serde::Deserialize)]
struct VersionInfo {
    num: String,
}
