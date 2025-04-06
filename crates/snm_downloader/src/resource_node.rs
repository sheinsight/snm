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
}

impl<'a> DownloadNodeResource<'a> {
  fn parse_shasum(&self, content: &str) -> HashMap<String, String> {
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
  fn get_extract_path(&self, version: &str) -> PathBuf {
    let file_name = self.get_artifact_name(version);
    self
      .config
      .download_dir
      .join("node")
      .join(version)
      .join(file_name)
  }

  fn get_timeout_secs(&self) -> Duration {
    Duration::from_secs(self.config.download_timeout_secs)
  }

  fn get_download_url(&self, version: &str) -> String {
    format!(
      "{host}/v{version}/{artifact_name}",
      host = self.config.node_dist_url,
      version = version,
      artifact_name = self.get_artifact_name(version)
    )
  }

  fn get_artifact_name(&self, version: &str) -> String {
    format!(
      "{bin_name}-v{version}-{os}-{arch}.{ext}",
      bin_name = self.bin_name,
      version = version,
      os = self.config.platform.os,
      arch = self.config.platform.arch,
      ext = self.config.platform.ext
    )
  }

  fn get_download_item(
    &self,
    version: &str,
    integrity: Option<Integrity>,
  ) -> DownloadItem<String, PathBuf> {
    let url = self.get_download_url(version);
    let target = self.get_extract_path(version);

    match integrity {
      Some(integrity) => DownloadItem::builder()
        .url(url)
        .target(target)
        .integrity(integrity)
        .build(),
      None => DownloadItem::builder().url(url).target(target).build(),
    }
  }

  fn get_decompress_dir(&self, version: &str) -> PathBuf {
    self.config.node_bin_dir.join(&version)
  }

  fn get_expect_shasum<'life0, 'async_trait>(
    &'life0 self,
    version: &'life0 str,
  ) -> Pin<Box<dyn Future<Output = anyhow::Result<Integrity>> + Send + 'async_trait>>
  where
    Self: 'async_trait,
    'life0: 'async_trait,
  {
    Box::pin(async move {
      let sha256_url = format!(
        "{host}/v{version}/SHASUMS256.txt",
        host = self.config.node_dist_url,
        version = version
      );

      let client = reqwest::Client::builder()
        .timeout(self.get_timeout_secs())
        .build()?;

      let sha256_str = client.get(sha256_url).send().await?.text().await?;

      let shasums = self.parse_shasum(&sha256_str);

      let file_name = self.get_artifact_name(version);

      let sha256 = shasums
        .get(&file_name)
        .map(|sha256| sha256.to_owned())
        .with_context(|| "Invalid Node SHASUM line format")?;

      Ok(Integrity::SHA256(sha256))
    })
  }
}
