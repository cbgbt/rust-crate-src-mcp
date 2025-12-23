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
    todo!()
}
