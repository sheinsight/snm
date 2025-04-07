use std::{collections::HashMap, fs, ops::Not, path::PathBuf, time::Duration};

use colored::Colorize;
use dialoguer::Confirm;
use itertools::Itertools;
use semver::Version;
use serde::Serialize;
use snm_config::snm_config::SnmConfig;
use snm_downloader::{download_resource, DownloadNodeResource};
use tracing::trace;

use crate::node::metadata::NodeMetadata;

use super::metadata::ScheduleMetadata;

#[derive(Debug, clap::Args, Serialize)]
pub struct DefaultArgs {
  #[arg(help = "Node version")]
  pub version: String,
}

#[derive(Debug, clap::Args, Serialize)]
pub struct ListArgs {
  #[arg(long, help = "List remote node", default_value = "false")]
  pub remote: bool,

  #[arg(long, help = "Compact mode", default_value = "false")]
  pub compact: bool,
}

impl Default for ListArgs {
  fn default() -> Self {
    Self {
      remote: false,
      compact: true,
    }
  }
}

#[derive(Debug, clap::Args, Serialize)]
pub struct UninstallArgs {
  #[arg(help = "Node version")]
  pub version: String,
}

#[derive(Debug, clap::Args, Serialize)]
pub struct InstallArgs {
  #[arg(help = "Node version")]
  pub version: String,
}

pub struct NodeFactory<'a> {
  config: &'a SnmConfig,
  default_dir: PathBuf,
}

impl<'a> NodeFactory<'a> {
  pub fn new(config: &'a SnmConfig) -> Self {
    Self {
      config,
      default_dir: config.node_bin_dir.join("default"),
    }
  }

  fn get_node_binary(&self, node_dir: &PathBuf) -> anyhow::Result<(PathBuf, bool)> {
    #[cfg(target_os = "windows")]
    let binary = node_dir.join("node.exe");
    #[cfg(not(target_os = "windows"))]
    let binary = node_dir.join("bin").join("node");

    let binary_exists = binary.try_exists()?;

    trace!("Binary: {:?} ( exists: {} )", binary, binary_exists);

    Ok((binary, binary_exists))
  }

  fn symlink_default(&self, source_dir: &PathBuf, target_dir: &PathBuf) -> anyhow::Result<()> {
    let default_dir_exists = self.has_default()?;

    if default_dir_exists {
      fs::remove_dir_all(&self.default_dir)?;
    }

    trace!(
      r#"Creating symlink: 
{:?} -> {:?}"#,
      &source_dir,
      &target_dir
    );

    #[cfg(unix)]
    std::os::unix::fs::symlink(&source_dir, &target_dir)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&source_dir, &target_dir)?;

    Ok(())
  }

  fn get_node_dir(&self, version: &str) -> PathBuf {
    self.config.node_bin_dir.join(version)
  }

  fn has_default(&self) -> anyhow::Result<bool> {
    let (_, binary_exists) = self.get_node_binary(&self.default_dir)?;

    Ok(binary_exists)
  }

  pub async fn set_default(&self, args: DefaultArgs) -> anyhow::Result<()> {
    trace!(r#"Start set default node , args: {:#?}"#, args);

    let node_dir = self.get_node_dir(&args.version);

    trace!("Directory for Node {}: {:?}", args.version, &node_dir);

    let (_, binary_exists) = self.get_node_binary(&node_dir)?;

    if !binary_exists {
      let confirmed = Confirm::new()
        .with_prompt(format!(
          "🤔 v{} is not installed, do you want to install it ?",
          &args.version
        ))
        .interact()?;
      if confirmed {
        self
          .install(InstallArgs {
            version: args.version.clone(),
          })
          .await?;
      }
    }

    self.symlink_default(&node_dir, &self.default_dir)?;

    println!("🎉 Node v{} is now default", &args.version.bright_green());

    Ok(())
  }

  pub async fn install(&self, args: InstallArgs) -> anyhow::Result<()> {
    let node_dir = self.get_node_dir(&args.version);

    let (_, binary_exists) = self.get_node_binary(&node_dir)?;

    if binary_exists {
      let confirm = Confirm::new()
        .with_prompt(format!(
          "🤔 v{} is already installed, do you want to reinstall it ?",
          &args.version
        ))
        .interact()?;

      if confirm {
        fs::remove_dir_all(&node_dir)?;
      } else {
        return Ok(());
      }
    }

    let resource = DownloadNodeResource::builder()
      .config(self.config)
      .bin_name(String::from("node"))
      .build();

    download_resource(resource, &args.version).await?;

    println!("🎉 Node v{} is installed", &args.version.bright_green());

    Ok(())
  }

  pub async fn uninstall(&self, args: UninstallArgs) -> anyhow::Result<()> {
    let node_dir = self.get_node_dir(&args.version);

    let (_, binary_exists) = self.get_node_binary(&node_dir)?;

    if !binary_exists {
      println!("🤔 v{} is not installed", &args.version.bright_green());
      return Ok(());
    }

    let default_dir_exists = self.has_default()?;

    if default_dir_exists {
      let link = self.default_dir.read_link()?;

      let eq = link.eq(&node_dir);

      trace!(
        r#"Symlink Relation: 
{:?} -> {:?}"#,
        &self.default_dir,
        &link
      );

      if eq {
        fs::remove_dir_all(&node_dir)?;
        fs::remove_dir_all(&self.default_dir)?;
        println!(
          "🎉 Node v{} is uninstalled , Now there is no default node .",
          &args.version.bright_green()
        );
      }
    } else {
      fs::remove_dir_all(&node_dir)?;
      println!("🎉 Node v{} is uninstalled", &args.version.bright_green());
    }

    Ok(())
  }

  pub async fn list(&self, args: ListArgs) -> anyhow::Result<()> {
    trace!(r#"Start show node list , args: {:#?}"#, args);

    if args.remote {
      let remote_node_list = self.get_remote_node().await?;
      remote_node_list.into_iter().for_each(|node| {
        println!("{}", node);
      });
      return Ok(());
    }

    let default_version = self
      .config
      .node_bin_dir
      .join("default")
      .read_link()
      .ok()
      .and_then(|p| p.file_name().map(|n| n.to_owned()))
      .map(|name| name.to_string_lossy().into_owned());

    trace!("default_version---->: {:?}", &default_version);

    let local_node_list = self
      .config
      .node_bin_dir
      .read_dir()
      .ok()
      .and_then(|dir| {
        let responses = dir
          .filter_map(|dir| dir.ok())
          .map(|dir| dir.path())
          .filter(|path| path.is_dir())
          .filter_map(|path| {
            path
              .file_name()
              .map(|name| name.to_string_lossy().into_owned())
          })
          .filter(|v| v.eq("default").not())
          .sorted_by_cached_key(|v| Version::parse(v).ok())
          .collect::<Vec<String>>();

        if responses.is_empty() {
          println!("😿 Local node list is empty");
          return None;
        }

        Some(responses)
      })
      .unwrap_or_default();

    trace!("local_node_list---->: {:?}", &local_node_list);

    if args.compact {
      local_node_list.into_iter().for_each(|v| {
        let is_default = default_version.as_ref().map_or(false, |d_v| v.eq(d_v));
        let prefix = if is_default { "->" } else { "" }.bright_green();
        println!("{:<2} {}", prefix, v);
      });
      return Ok(());
    }

    let mut remote_node_map = self
      .get_remote_node()
      .await?
      .into_iter()
      .map(|node| (node.version[1..].to_string(), node))
      .collect::<HashMap<String, NodeMetadata>>();

    trace!(r#"Node remote info{:#?}"#, remote_node_map);

    local_node_list
      .into_iter()
      .filter_map(|v| remote_node_map.remove(&v))
      .for_each(|e| {
        println!("{}", e);
      });

    Ok(())
  }

  async fn get_remote_node(&self) -> anyhow::Result<Vec<NodeMetadata>> {
    let default_version = self
      .config
      .node_bin_dir
      .join("default")
      .read_link()
      .ok()
      .and_then(|p| p.file_name().map(|n| n.to_owned()))
      .map(|name| name.to_string_lossy().into_owned());

    let x = ScheduleMetadata::fetch(self.config).await?;

    let node_list_url = format!("{host}/index.json", host = self.config.node_dist_url);

    let client = reqwest::Client::new();

    let node_vec: Vec<NodeMetadata> = client
      .get(&node_list_url)
      .timeout(Duration::from_secs(10))
      .send()
      .await?
      .json::<Vec<NodeMetadata>>()
      .await?
      .into_iter()
      .filter_map(|node| {
        node
          .version
          .to_owned()
          .split_once('.')
          .and_then(|(major, _)| {
            (major != "v0").then(|| NodeMetadata {
              default: default_version.as_ref().map(|v| v.eq(&node.version[1..])),
              schedule: x.get(major).map(|s| s.clone()),
              ..node
            })
          })
      })
      .sorted_by_cached_key(|node| Version::parse(&node.version[1..]).ok())
      .collect();

    Ok(node_vec)
  }
}
