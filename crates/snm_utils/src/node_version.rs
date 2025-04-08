use std::path::Path;

use anyhow::bail;
use lazy_regex::regex;

pub struct NodeVersion<P: AsRef<Path>> {
  pub raw: String,
  pub val: String,
  pub file: P,
}

impl<P: AsRef<Path>> NodeVersion<P> {
  pub async fn parse(file: P) -> anyhow::Result<Self> {
    let file_name = file.as_ref();

    let raw = tokio::fs::read_to_string(file_name).await?;

    let val = raw.trim().to_lowercase().trim_start_matches("v").to_owned();

    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

    if !r.is_match(&val) {
      bail!(
        "Invalid Node.js version format: {} in {:#?}",
        &val,
        file_name
      );
    }

    Ok(Self { raw, val, file })
  }
}
