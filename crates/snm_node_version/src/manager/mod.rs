use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    ops::Not,
    path::{Path, PathBuf},
    process::{exit, ExitCode},
    time::Duration,
};

use anyhow::{bail, Context};
use colored::*;
use dialoguer::Confirm;
use flate2::read::GzDecoder;
use itertools::Itertools;
use metadata::NodeMetadata;
use schedule::Schedule;
use semver::Version;
use sha2::{Digest, Sha256};
use snm_config::SnmConfig;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::{archive_extension::ArchiveExtension, downloader::NodeDownloader};

pub mod lts;
pub mod metadata;
pub mod schedule;

#[derive(Debug, clap::Args)]
pub struct ListArgs {
    #[arg(long, help = "List remote node", default_value = "false")]
    pub remote: bool,

    #[arg(long, help = "Compact mode", default_value = "false")]
    pub compact: bool,
}

pub struct NodeManager<'a> {
    config: &'a SnmConfig,
}

impl<'a> NodeManager<'a> {
    pub fn new(config: &'a SnmConfig) -> Self {
        Self { config }
    }

    pub async fn install(&self, version: &str) -> anyhow::Result<()> {
        let node_dir = self.config.node_bin_dir.join(version);

        let node_bin_file = node_dir.join("bin").join("node");

        if node_bin_file.try_exists()? {
            let confirm = Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is already installed, do you want to reinstall it ?",
                    &version
                ))
                .interact()?;

            if confirm {
                fs::remove_dir_all(&node_dir)?;
            } else {
                exit(1);
            }
        }

        NodeDownloader::new(self.config).download(version).await?;

        Ok(())
    }

    pub async fn list(&self, args: ListArgs) -> anyhow::Result<()> {
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

        let local_node_list = self
            .config
            .node_bin_dir
            .read_dir()?
            .filter_map(|dir| dir.ok())
            .map(|dir| dir.path())
            .filter(|path| path.is_dir())
            .filter_map(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .filter(|v| v.eq("default").not())
            .sorted_by_cached_key(|v| Version::parse(v).ok())
            .collect::<Vec<String>>();

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

        let x = Schedule::new(self.config).await?;

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
                // if let Some((major, _)) = node.version.split_once('.') {
                //     if major == "v0" {
                //         return None;
                //     }
                //     let schedule = x.get(major);
                //     return Some(NodeMetadata { schedule, ..node });
                // }
                // None

                node.version
                    .to_owned()
                    .split_once('.')
                    .and_then(|(major, _)| {
                        (major != "v0").then(|| NodeMetadata {
                            default: default_version.as_ref().map(|v| v.eq(&node.version[1..])),
                            schedule: x.get(major),
                            ..node
                        })
                    })

                // if let Some((major, _)) = node.version.clone().split_once('.') {
                //     return (major != "v0").then(|| NodeMetadata {
                //         schedule: x.get(major),
                //         ..node
                //     });
                // }

                // None
            })
            .sorted_by_cached_key(|node| Version::parse(&node.version[1..]).ok())
            .collect();

        Ok(node_vec)
    }
}
