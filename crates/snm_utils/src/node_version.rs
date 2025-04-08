use std::path::PathBuf;

use anyhow::bail;
use lazy_regex::regex;

#[derive(Debug, Clone)]
pub struct NodeVersion {
  pub raw: String,
  pub val: String,
  pub file: Option<PathBuf>,
}

impl NodeVersion {
  pub async fn try_from_file(file: &PathBuf) -> anyhow::Result<Self> {
    let raw = tokio::fs::read_to_string(file).await?;

    let node_version = Self::try_from_str(&raw)?;

    Ok(node_version)
  }

  pub fn try_from_str(raw: &str) -> anyhow::Result<Self> {
    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

    let val = raw.trim().to_lowercase().trim_start_matches("v").to_owned();

    if !r.is_match(&val) {
      bail!("Invalid Node.js version format: {}", raw);
    }

    Ok(Self {
      raw: raw.to_owned(),
      val,
      file: None,
    })
  }
}
