pub mod extract;
pub mod version;

use bon::Builder;
use snafu::{ResultExt, Snafu};
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

pub async fn get_crate_source(
    crate_name: &str,
    version_req: Option<&str>,
) -> Result<CrateSource, GetCrateSourceError> {
    use get_crate_source_error::*;

    let version = version::resolve_version(crate_name, version_req)
        .await
        .context(ResolveVersionSnafu)?;
    let checkout_path = extract::extract_crate(crate_name, &version)
        .await
        .context(ExtractSnafu)?;
    let message = format!(
        "Crate '{}' version {} extracted to {}",
        crate_name,
        version,
        checkout_path.display()
    );

    Ok(CrateSource::builder()
        .crate_name(crate_name)
        .version(version)
        .checkout_path(checkout_path)
        .message(message)
        .build())
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum GetCrateSourceError {
    #[snafu(display("failed to resolve version"))]
    ResolveVersion {
        source: version::ResolveVersionError,
    },
    #[snafu(display("failed to extract crate"))]
    Extract { source: extract::ExtractError },
}
