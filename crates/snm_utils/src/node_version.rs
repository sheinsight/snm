use std::path::Path;

use anyhow::bail;
use lazy_regex::regex;

pub struct NodeVersion {
  pub raw: String,
  pub val: String,
  pub file: String,
}

impl NodeVersion {
  pub async fn parse<P: AsRef<Path>>(raw: String, file: P) -> anyhow::Result<Self> {
    let raw = tokio::fs::read_to_string(file).await?;

    let val = raw.trim().to_lowercase().trim_start_matches("v").to_owned();

    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

    if !r.is_match(&val) {
      bail!("Invalid Node.js version format: {} in {:#?}", &val, file);
    }

    Ok(Self {
      raw,
      val,
      file: file.as_ref().to_string_lossy().into_owned(),
    })
  }
}
