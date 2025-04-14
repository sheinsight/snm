use std::{fs::read_to_string, path::PathBuf};

use anyhow::bail;
use lazy_regex::regex;

#[derive(Debug, Clone)]
pub struct NodeVersion {
  pub raw: Option<String>,
  pub val: String,
}

impl TryFrom<String> for NodeVersion {
  type Error = anyhow::Error;
  fn try_from(raw: String) -> Result<Self, Self::Error> {
    let raw_trim = raw.trim().to_lowercase();

    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

    if !r.is_match(&raw_trim) {
      bail!("Invalid Node.js version format: {}", raw);
    }

    let val = raw_trim.trim_start_matches("v").to_owned();

    Ok(Self {
      raw: Some(raw.to_owned()),
      val,
    })
  }
}

impl TryFrom<PathBuf> for NodeVersion {
  type Error = anyhow::Error;
  fn try_from(file: PathBuf) -> Result<Self, Self::Error> {
    let raw = read_to_string(&file)?;
    Self::try_from(raw)
  }
}

#[cfg(test)]
mod test {
  use anyhow::bail;
  use lazy_regex::regex;

  #[test]
  fn should_xx() -> anyhow::Result<()> {
    let raw = r#"v20.0.0
    
    
    "#;
    let raw = raw.trim().to_lowercase();
    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

    if !r.is_match(&raw) {
      bail!("Invalid Node.js version format: {}", raw);
    }

    Ok(())
  }
}
