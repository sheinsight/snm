use std::{
  env::{current_dir, Args},
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use lazy_regex::regex;
use snm_config::snm_config::SnmConfig;
use snm_downloader::{download_resource, DownloadNodeResource};
use snm_utils::consts::{NODE_VERSION_FILE_NAME, SNM_PREFIX};
use tokio::fs::read_to_string;
use tracing::trace;
use up_finder::UpFinder;

use crate::{node_shim::NodeShim, pm_shim::PmShim};

pub enum CommandShim {
  Node(NodeShim),
  Pm(PmShim),
}

impl CommandShim {
  pub async fn proxy<T: AsRef<Path>>(&self, cwd: &T) -> anyhow::Result<()> {
    let snm_config = SnmConfig::from(SNM_PREFIX, cwd)?;
    trace!(r#"{:#?}"#, snm_config);
    match self {
      CommandShim::Node(node_shim) => node_shim.proxy().await?,
      CommandShim::Pm(pm_shim) => pm_shim.proxy().await?,
    }
    Ok(())
  }

  pub async fn from_args(args: Args) -> anyhow::Result<Self> {
    let args = args.collect::<Vec<String>>();

    let Some(actual_bin_name) = args.first() else {
      bail!("No binary name provided in arguments {:#?}", args);
    };

    trace!(r#"try_from args: {:#?}"#, args);

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(SNM_PREFIX, &cwd)?;

    let bin_dir = Self::find_node_bin_dir(&snm_config).await?;

    let paths = vec![bin_dir.to_string_lossy().into_owned()];

    if actual_bin_name == "node" {
      Ok(CommandShim::Node(NodeShim::new(args, paths)))
    } else {
      Ok(CommandShim::Pm(PmShim::new(args, paths, snm_config)))
    }
  }

  async fn find_node_bin_dir(config: &SnmConfig) -> anyhow::Result<PathBuf> {
    let find_up = UpFinder::builder()
      .cwd(&config.workspace) // 从当前目录开始
      .build();

    let files = find_up.find_up(NODE_VERSION_FILE_NAME);

    if files.is_empty() && config.strict {
      bail!("In strict mode, a .node-version file must be configured in the current directory.");
    }

    let version = if let Some(file) = files.first() {
      let raw_version = read_to_string(file).await?.trim().to_string();
      Self::parse_node_version(raw_version, Some(file)).await?
    } else {
      Self::get_default_version(config).await?
    };

    let node_home_dir = config.node_bin_dir.join(&version);

    let node_exe = if cfg!(windows) {
      node_home_dir.join("node.exe")
    } else {
      node_home_dir.join("bin").join("node")
    };

    if !node_exe.try_exists()? {
      let resource = DownloadNodeResource::builder()
        .config(config)
        .bin_name(String::from("node"))
        .build();

      download_resource(resource, &version).await?;
    }

    let node_bin_dir = if cfg!(windows) {
      node_home_dir
    } else {
      node_home_dir.join("bin")
    };

    Ok(node_bin_dir)
  }

  async fn parse_node_version(
    raw_version: String,
    file_path: Option<&Path>,
  ) -> anyhow::Result<String> {
    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");
    if !r.is_match(&raw_version) {
      bail!(
        "Invalid Node.js version format: {:#?}{}",
        &raw_version,
        file_path.map_or(String::new(), |p| format!(" in {:#?}", p))
      );
    }
    Ok(
      raw_version
        .to_lowercase()
        .trim_start_matches("v")
        .trim()
        .to_string(),
    )
  }

  async fn get_default_version(config: &SnmConfig) -> anyhow::Result<String> {
    let default_dir = config.node_bin_dir.join("default");

    if !default_dir.try_exists()? {
      bail!("No default Node.js version found");
    }

    default_dir
      .read_link()?
      .file_name()
      .map(|s| s.to_string_lossy().into_owned())
      .with_context(|| "Invalid symbolic link for default Node.js version")
  }
}
