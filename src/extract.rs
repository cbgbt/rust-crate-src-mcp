use snafu::Snafu;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum ExtractError {
    #[snafu(display("failed to download crate '{crate_name}' version '{version}'"))]
    Download {
        crate_name: String,
        version: String,
        source: reqwest::Error,
    },
    #[snafu(display("failed to decompress crate archive"))]
    Decompress {
        source: std::io::Error,
    },
    #[snafu(display("failed to untar crate archive"))]
    Untar {
        source: std::io::Error,
    },
    #[snafu(display("failed to access cache directory"))]
    CacheDir {
        source: std::io::Error,
    },
}

pub async fn extract_crate(
    crate_name: &str,
    version: &str,
) -> Result<PathBuf, ExtractError> {
    todo!()
}
