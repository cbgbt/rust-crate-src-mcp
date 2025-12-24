use snafu::{ResultExt, Snafu};
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
    Decompress { source: std::io::Error },
    #[snafu(display("failed to untar crate archive"))]
    Untar { source: std::io::Error },
    #[snafu(display("failed to access cache directory"))]
    CacheDir { source: std::io::Error },
}

pub async fn extract_crate(crate_name: &str, version: &str) -> Result<PathBuf, ExtractError> {
    use extract_error::*;

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cache_base = PathBuf::from(home).join(".cargo/registry/src");
    let cache_dir = std::fs::read_dir(&cache_base)
        .ok()
        .and_then(|mut entries| {
            entries.find_map(|e| {
                e.ok().and_then(|e| {
                    let name = e.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with("index.crates.io-") {
                        Some(e.path())
                    } else {
                        None
                    }
                })
            })
        })
        .unwrap_or_else(|| cache_base.join("index.crates.io-6f17d22bba15001f"));

    let extract_path = cache_dir.join(format!("{}-{}", crate_name, version));

    if extract_path.exists() {
        return Ok(extract_path);
    }

    let url = format!(
        "https://static.crates.io/crates/{}/{}-{}.crate",
        crate_name, crate_name, version
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-crate-src")
        .send()
        .await
        .context(DownloadSnafu {
            crate_name,
            version,
        })?
        .bytes()
        .await
        .context(DownloadSnafu {
            crate_name,
            version,
        })?;

    let decoder = flate2::read::GzDecoder::new(&response[..]);
    let mut archive = tar::Archive::new(decoder);

    std::fs::create_dir_all(&cache_dir).context(CacheDirSnafu)?;
    archive.unpack(&cache_dir).context(UntarSnafu)?;

    Ok(extract_path)
}
