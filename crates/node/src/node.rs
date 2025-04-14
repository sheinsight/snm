use std::path::PathBuf;

use anyhow::bail;
use snm_config::snm_config::SnmConfig;
use snm_downloader::{DownloadNodeResource, download_resource};
use snm_utils::consts::NODE_VERSION_FILE_NAME;
use up_finder::UpFinder;

use crate::{NodeVersion, NodeVersionHome};

#[derive(Debug)]
pub struct NodeSetup {
  config: SnmConfig,
}

impl From<SnmConfig> for NodeSetup {
  fn from(config: SnmConfig) -> Self {
    Self { config }
  }
}

impl NodeSetup {
  pub async fn resolve_node_bin_dir(&self) -> anyhow::Result<PathBuf> {
    let nv = self.resolve_node_version()?;

    let node_home_dir = self.config.node_bin_dir.join(&nv.val);

    let node_home_dir = NodeVersionHome(node_home_dir);

    let node_exe = node_home_dir.exe();

    self.ensure_node(&node_exe, &nv).await?;

    let node_bin_dir = node_home_dir.bin_dir();

    Ok(node_bin_dir)
  }
}

impl NodeSetup {
  fn resolve_node_version(&self) -> anyhow::Result<NodeVersion> {
    let nv = if let Some(nv) = self.find_up_node_version()? {
      nv
    } else {
      self.find_default_node_version()?
    };
    Ok(nv)
  }

  async fn ensure_node(&self, node_exe: &PathBuf, nv: &NodeVersion) -> anyhow::Result<()> {
    if !node_exe
      .try_exists()
      .map_err(|e| anyhow::anyhow!("Failed to check if node executable exists: {:?}", e))?
    {
      let resource = DownloadNodeResource::builder()
        .config(&self.config)
        .bin_name(String::from("node"))
        .version(nv.val.clone())
        .build();

      download_resource(resource).await?;
    }
    Ok(())
  }

  fn find_up_node_version(&self) -> anyhow::Result<Option<NodeVersion>> {
    let find_up = UpFinder::builder()
      .cwd(&self.config.workspace) // 从当前目录开始
      .build();

    let files = find_up.find_up(NODE_VERSION_FILE_NAME);

    if files.is_empty() && self.config.strict {
      bail!("In strict mode, a .node-version file must be configured in the current directory.");
    }

    if let Some(file) = files.first() {
      let nv = NodeVersion::try_from(file.to_owned())
        .map_err(|e| anyhow::anyhow!("Failed to parse Node version from file: {:?}", e))?;
      Ok(Some(nv))
    } else {
      Ok(None)
    }
  }

  fn find_default_node_version(&self) -> anyhow::Result<NodeVersion> {
    let default_dir = self.config.node_bin_dir.join("default");

    if !default_dir
      .try_exists()
      .map_err(|e| anyhow::anyhow!("Failed to check if default directory exists: {:?}", e))?
    {
      bail!("No default Node.js version found");
    }

    let v = default_dir
      .read_link()
      .map_err(|e| anyhow::anyhow!("Failed to read symbolic link: {:?}", e))?
      .file_name()
      .map(|s| s.to_string_lossy().into_owned())
      .ok_or_else(|| anyhow::anyhow!("Invalid symbolic link for default Node.js version"))?;

    NodeVersion::try_from(v)
  }
}
