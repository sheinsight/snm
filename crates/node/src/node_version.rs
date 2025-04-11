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
    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");
    if !r.is_match(&raw) {
      bail!("Invalid Node.js version format: {}", raw);
    }
    let val = raw.trim().to_lowercase().trim_start_matches("v").to_owned();
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

// impl NodeVersion {
//   pub async fn try_from_file<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
//     let raw = read_to_string(&file)?;

//     let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

//     if !r.is_match(&raw) {
//       bail!("Invalid Node.js version format: {}", raw);
//     }

//     let val = raw.trim().to_lowercase().trim_start_matches("v").to_owned();

//     Ok(Self {
//       raw: Some(raw.to_owned()),
//       val,
//     })
//   }

//   pub fn try_default<P: AsRef<Path>>(node_bin_dir: P) -> anyhow::Result<Self> {
//     let default_dir = node_bin_dir.as_ref().join("default");

//     if !default_dir.try_exists()? {
//       bail!("No default Node.js version found");
//     }

//     let val = default_dir
//       .read_link()?
//       .file_name()
//       .map(|v| v.to_string_lossy().into_owned())
//       .ok_or_else(|| {
//         anyhow::anyhow!(
//           "Failed to read default Node.js version from symlink: {:?}",
//           default_dir
//         )
//       })?;

//     Ok(Self { raw: None, val })
//   }
// }
