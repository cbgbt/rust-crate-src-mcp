pub mod version;
pub mod extract;

use bon::Builder;
use snafu::Snafu;
use std::path::PathBuf;

#[derive(Debug, Clone, Builder)]
#[non_exhaustive]
#[builder(on(_, into))]
pub struct CrateSource {
    pub crate_name: String,
    pub version: String,
    pub checkout_path: PathBuf,
    pub message: String,
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum GetCrateSourceError {
    #[snafu(display("failed to resolve version"))]
    ResolveVersion {
        source: version::ResolveVersionError,
    },
    #[snafu(display("failed to extract crate"))]
    Extract {
        source: extract::ExtractError,
    },
}

pub async fn get_crate_source(
    crate_name: &str,
    version_req: Option<&str>,
) -> Result<CrateSource, GetCrateSourceError> {
    todo!()
}
