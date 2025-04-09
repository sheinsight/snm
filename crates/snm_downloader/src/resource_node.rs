use std::{collections::HashMap, path::PathBuf, pin::Pin, time::Duration};

use anyhow::Context;
use robust_downloader::{DownloadItem, Integrity};
use snm_config::snm_config::SnmConfig;
use typed_builder::TypedBuilder;

use crate::DownloadResource;

#[derive(Debug, Clone, TypedBuilder)]
pub struct DownloadNodeResource<'a> {
  pub config: &'a SnmConfig,
  pub bin_name: String,
  pub version: String,
}

impl<'a> DownloadNodeResource<'a> {
  fn parse_shasum(content: &str) -> HashMap<String, String> {
    content
      .lines()
      .filter_map(|line| {
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next()) {
          (Some(sha256), Some(filename)) => Some((filename.to_string(), sha256.to_string())),
          _ => None,
        }
      })
      .collect()
  }
}

impl<'a> DownloadResource for DownloadNodeResource<'a> {
  fn get_extract_path(&self) -> PathBuf {
    let file_name = self.get_artifact_name();
    self
      .config
      .download_dir
      .join("node")
      .join(&self.version)
      .join(file_name)
  }

  fn get_timeout_secs(&self) -> Duration {
    Duration::from_secs(self.config.download_timeout_secs)
  }

  fn get_download_url(&self) -> String {
    format!(
      "{host}/v{version}/{artifact_name}",
      host = self.config.node_dist_url,
      version = &self.version,
      artifact_name = self.get_artifact_name()
    )
  }

  fn get_artifact_name(&self) -> String {
    format!(
      "{bin_name}-v{version}-{os}-{arch}.{ext}",
      bin_name = self.bin_name,
      version = &self.version,
      os = self.config.platform.os,
      arch = self.config.platform.arch,
      ext = self.config.platform.ext
    )
  }

  fn get_download_item(&self, integrity: Option<Integrity>) -> DownloadItem<String, PathBuf> {
    let url = self.get_download_url();
    let target = self.get_extract_path();

    match integrity {
      Some(integrity) => DownloadItem::builder()
        .url(url)
        .target(target)
        .integrity(integrity)
        .build(),
      None => DownloadItem::builder().url(url).target(target).build(),
    }
  }

  fn get_decompress_dir(&self) -> PathBuf {
    self.config.node_bin_dir.join(&self.version)
  }

  fn get_expect_shasum<'async_trait>(
    &self,
  ) -> Pin<Box<dyn Future<Output = anyhow::Result<Integrity>> + Send + 'async_trait>>
  where
    Self: 'async_trait,
  {
    let version = self.version.clone();
    let node_dist_url = self.config.node_dist_url.clone();
    let timeout = self.get_timeout_secs();
    let file_name = self.get_artifact_name();

    Box::pin(async move {
      let version = version
        .trim()
        .to_lowercase()
        .trim_start_matches("v")
        .to_owned();

      let sha256_url = format!(
        "{host}/v{version}/SHASUMS256.txt",
        host = node_dist_url,
        version = &version
      );

      let client = reqwest::Client::builder().timeout(timeout).build()?;

      let sha256_str = client.get(sha256_url).send().await?.text().await?;

      let shasums = Self::parse_shasum(&sha256_str);

      let sha256 = shasums
        .get(&file_name)
        .map(|sha256| sha256.to_owned())
        .with_context(|| "Invalid Node SHASUM line format")?;

      Ok(Integrity::SHA256(sha256))
    })
  }
}
