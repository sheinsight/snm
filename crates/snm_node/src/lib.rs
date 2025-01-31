use std::{
  env,
  fs::read_to_string,
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use downloader::NodeDownloader;
use snm_config::snm_config::SnmConfig;
use snm_utils::consts::{ENV_KEY_FOR_SNM_NODE, NODE_VERSION_FILE_NAME};
pub mod factory;
pub use factory::*;
pub mod downloader;

#[derive(Debug)]
pub struct SNode<'a> {
  pub version: Option<String>,
  pub config: &'a SnmConfig,
}

impl<'a> SNode<'a> {
  pub fn try_from(config: &'a SnmConfig) -> anyhow::Result<Self> {
    Self::from_env(config)
      .or_else(|_| Self::from_config_file(config))
      .or_else(|_| {
        if config.strict {
          bail!(
            "In strict mode, a .node-version file must be configured in the current directory."
          );
        }
        Self::from_default(config)
      })
  }

  fn from_config_file(config: &'a SnmConfig) -> anyhow::Result<Self> {
    let file_path = config.workspace.join(NODE_VERSION_FILE_NAME);
    let version =
      Self::read_version_file(&file_path).with_context(|| "Invalid node version file")?;

    Ok(Self {
      version: Some(version),
      config,
    })
  }

  fn from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
    let version = env::var(ENV_KEY_FOR_SNM_NODE)?;
    Ok(Self {
      version: Some(version),
      config,
    })
  }

  fn from_default(config: &'a SnmConfig) -> anyhow::Result<Self> {
    let default_dir = config.node_bin_dir.join("default");

    if !default_dir.try_exists()? {
      bail!("No default Node.js version found");
    }

    let version = default_dir
      .read_link()?
      .file_name()
      .map(|s| s.to_string_lossy().into_owned())
      .with_context(|| "Invalid default version link")?;

    Ok(Self {
      version: Some(version),
      config,
    })
  }

  fn read_version_file<T: AsRef<Path>>(version_path: T) -> Option<String> {
    let file_path = version_path.as_ref();
    if !file_path.exists() {
      return None;
    }
    let raw = read_to_string(file_path).ok()?;
    let version = raw.to_lowercase().trim_start_matches('v').trim().to_owned();
    let version_parts: Vec<_> = version.split('.').collect();
    if version_parts.len() != 3 || version_parts.iter().any(|s| s.parse::<u32>().is_err()) {
      return None;
    }
    env::set_var(ENV_KEY_FOR_SNM_NODE, &version);
    Some(version)
  }
}

impl<'a> SNode<'a> {
  pub async fn get_bin(&self) -> anyhow::Result<PathBuf> {
    let version = self
      .version
      .as_deref()
      .context("No valid node version found")?;

    self.check_v(&version)?;

    let node_dir = self.config.node_bin_dir.join(&version);

    #[cfg(target_os = "windows")]
    let node_bin_dir = {
      let node_bin_file = node_dir.join("node.exe");
      if node_bin_file.try_exists()? {
        return Ok(node_dir);
      }
      node_dir
    };

    #[cfg(not(target_os = "windows"))]
    let node_bin_dir = {
      let node_bin_dir = node_dir.join("bin");
      let node_bin_file = node_bin_dir.join("node");
      if node_bin_file.try_exists()? {
        return Ok(node_bin_dir);
      }
      node_bin_dir
    };

    NodeDownloader::new(self.config).download(version).await?;

    Ok(node_bin_dir)
  }

  fn check_v(&self, version: &str) -> anyhow::Result<()> {
    if self.config.node_white_list.is_empty() {
      return Ok(());
    }
    if self.config.node_white_list.contains(&version) {
      return Ok(());
    }

    bail!(
      r"Unsupported node version: {version}, supported versions: {}",
      self.config.node_white_list
    );
  }
}
