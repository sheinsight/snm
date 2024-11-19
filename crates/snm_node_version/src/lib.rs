use std::{env, fs::read_to_string, path::Path};

use anyhow::{bail, Context};
use downloader::NodeDownloader;
use snm_config::SnmConfig;

use snm_utils::snm_error::SnmError;
mod archive_extension;
pub mod manager;
pub use manager::*;
mod downloader;

const FILE_NAME: &str = ".node-version";

const SNM_NODE_VERSION_ENV_KEY: &str = "SNM_NODE_VERSION";

#[derive(Debug)]
pub struct SNode<'a> {
    pub version: Option<String>,
    pub config: &'a SnmConfig,
}

impl<'a> SNode<'a> {
    pub fn try_from(config: &'a SnmConfig) -> anyhow::Result<Self> {
        Self::from_env(config)
            .or_else(|_| Self::from(config))
            .or_else(|_| Self::from_default(config))
    }

    fn from(config: &'a SnmConfig) -> anyhow::Result<Self> {
        let file_path = config.workspace.join(FILE_NAME);

        let version =
            Self::read_version_file(&file_path).with_context(|| "Invalid node version file")?;

        Ok(Self {
            version: Some(version),
            config,
        })
    }

    fn from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
        let version = env::var(SNM_NODE_VERSION_ENV_KEY)?;
        Ok(Self {
            version: Some(version),
            config,
        })
    }

    fn from_default(config: &'a SnmConfig) -> anyhow::Result<Self> {
        if !config.node_bin_dir.try_exists()? {
            bail!("Node binary directory does not exist");
        }

        let default_dir = config.node_bin_dir.join("default");

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
        env::set_var(SNM_NODE_VERSION_ENV_KEY, &version);
        Some(version)
    }
}

impl<'a> SNode<'a> {
    pub async fn get_bin(&self) -> anyhow::Result<String> {
        let version = self
            .version
            .as_deref()
            .context("No valid node version found")?;

        self.check_v(&version)?;

        let node_dir = self.config.node_bin_dir.join(&version);

        let node_bin_dir = node_dir.join("bin");

        let node_bin_file = node_bin_dir.join("node");

        if node_bin_file.try_exists()? {
            return Ok(node_bin_dir.to_string_lossy().into_owned());
        }

        NodeDownloader::new(self.config).download(version).await?;

        Ok(node_bin_dir.to_string_lossy().into_owned())
    }

    fn check_v(&self, version: &str) -> anyhow::Result<()> {
        if self.config.node_white_list.is_empty() {
            return Ok(());
        }
        if self.config.node_white_list.contains(&version) {
            return Ok(());
        }
        bail!(SnmError::UnsupportedNodeVersionError {
            version: version.to_owned(),
            node_white_list: self
                .config
                .node_white_list
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        })
    }
}
