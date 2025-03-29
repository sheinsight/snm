use std::{
  fs::read_to_string,
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use downloader::NodeDownloader;
use snm_config::snm_config::SnmConfig;
use snm_utils::{consts::NODE_VERSION_FILE_NAME, FindUp};
pub mod factory;
pub use factory::*;
pub mod downloader;

#[derive(Debug, Clone)]
pub struct SNode {
  pub version: Option<String>,
  pub config: SnmConfig,
}

impl SNode {
  pub fn find_up(config: SnmConfig) -> anyhow::Result<Self> {
    let node_version_file = FindUp::new(&config.workspace).find(".node-version")?;

    let v = node_version_file
      .iter()
      .find_map(|item| Self::read_version_file(item));

    if let Some(v) = v {
      return Ok(Self {
        version: Some(v),
        config,
      });
    } else {
      if config.strict {
        bail!("In strict mode, a .node-version file must be configured in the current directory.");
      }

      Self::from_default(config)
    }
  }

  pub fn from_config_file(config: SnmConfig) -> anyhow::Result<Self> {
    let file_path = config.workspace.join(NODE_VERSION_FILE_NAME);
    let version =
      Self::read_version_file(&file_path).with_context(|| "Invalid node version file")?;

    Ok(Self {
      version: Some(version),
      config,
    })
  }

  fn from_default(config: SnmConfig) -> anyhow::Result<Self> {
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
    Some(version)
  }
}

impl SNode {
  fn get_version(&self) -> anyhow::Result<String> {
    let v = self
      .version
      .as_deref()
      .context("No valid node version found")?;
    self.check_v(&v)?;
    Ok(v.to_string())
  }

  pub async fn get_node_modules_dir(&self) -> anyhow::Result<PathBuf> {
    let node_home_dir = self.ensure_node().await?;

    #[cfg(windows)]
    let node_modules_dir = node_home_dir.join("node_modules");

    #[cfg(not(windows))]
    let node_modules_dir = node_home_dir.join("lib").join("node_modules");

    Ok(node_modules_dir)
  }

  pub async fn get_bin_dir(&self) -> anyhow::Result<PathBuf> {
    let node_home_dir = self.ensure_node().await?;

    #[cfg(windows)]
    let bin_dir = node_home_dir;

    #[cfg(not(windows))]
    let bin_dir = node_home_dir.join("bin");

    Ok(bin_dir)
  }

  pub async fn get_home_dir(&self) -> anyhow::Result<PathBuf> {
    self.ensure_node().await
  }

  async fn ensure_node(&self) -> anyhow::Result<PathBuf> {
    let version = self.get_version()?;
    let node_home_dir = self.config.node_bin_dir.join(&version);

    // let (node_bin_dir, exe_name) = if cfg!(windows) {
    //   (node_dir.clone(), "node.exe")
    // } else {
    //   (node_dir.join("bin"), "node")
    // };

    let node_exe = if cfg!(windows) {
      node_home_dir.join("node.exe")
    } else {
      node_home_dir.join("bin").join("node")
    };

    // 优化: 提前计算最终返回值
    // let node_exe = node_bin_dir.join(exe_name);
    // 优化: 只在文件不存在时下载
    if !node_exe.try_exists()? {
      NodeDownloader::new(&self.config).download(&version).await?;
    }

    Ok(node_home_dir)
  }

  // pub async fn ensure_node_and_return_dir(&self) -> anyhow::Result<PathBuf> {
  //   let version = self.get_version()?;

  //   self.check_v(&version)?;

  //   let node_dir = self.config.node_bin_dir.join(&version);

  //   let (node_bin_dir, exe_name) = if cfg!(windows) {
  //     (node_dir.clone(), "node.exe")
  //   } else {
  //     (node_dir.join("bin"), "node")
  //   };

  //   // 优化: 提前计算最终返回值
  //   let node_exe = node_bin_dir.join(exe_name);

  //   // 优化: 只在文件不存在时下载
  //   if !node_exe.try_exists()? {
  //     NodeDownloader::new(self.config).download(&version).await?;
  //   }

  //   Ok(node_dir)
  // }

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
