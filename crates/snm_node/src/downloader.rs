use std::{collections::HashMap, path::PathBuf, time::Duration};

use anyhow::{bail, Context};
use hashery::Hashery;
use robust_downloader::RobustDownloader;
use snm_config::snm_config::SnmConfig;
use snm_utils::tarball::ArchiveExtension;
use tracing::trace;

pub struct NodeDownloader<'a> {
  config: &'a SnmConfig,
}

impl<'a> NodeDownloader<'a> {
  pub fn new(config: &'a SnmConfig) -> Self {
    Self { config }
  }

  pub async fn download(&self, version: &str) -> anyhow::Result<PathBuf> {
    let download_url = self.get_download_url(version);
    let downloaded_file = self.get_downloaded_path_buf(version);

    let downloader = RobustDownloader::builder()
      .connect_timeout(Duration::from_secs(self.config.download_timeout_secs))
      .max_concurrent(2)
      .build();

    downloader
      .download(vec![(download_url, &downloaded_file)])
      .await?;

    self.verify_shasum(&downloaded_file, &version).await?;

    let node_dir = self.config.node_bin_dir.join(&version);

    ArchiveExtension::from_path(downloaded_file)?.decompress(&node_dir)?;

    Ok(node_dir)
  }

  async fn verify_shasum(&self, file_path: &PathBuf, version: &str) -> anyhow::Result<()> {
    let hasher = Hashery::builder()
      .algorithm(hashery::Algorithm::SHA256)
      .build();

    let actual = hasher.digest(file_path).await?;

    let expected = self.get_expect_shasum(version).await?;

    trace!("Verify shasum: expect: {}, actual: {}", expected, actual);

    if actual != expected {
      bail!("Node binary shasum mismatch");
    }

    Ok(())
  }

  async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
    let sha256_url = format!(
      "{host}/v{version}/SHASUMS256.txt",
      host = self.config.node_dist_url,
      version = version
    );

    let file_name = self.get_artifact_name(version);

    let client = reqwest::Client::builder()
      .timeout(Duration::from_secs(self.config.download_timeout_secs))
      .build()?;

    let sha256_str = client.get(&sha256_url).send().await?.text().await?;

    let shasums = self.parse_shasum(&sha256_str);

    shasums
      .get(&file_name)
      .map(|sha256| sha256.to_owned())
      .with_context(|| "Invalid Node SHASUM line format")
  }

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

  fn get_downloaded_path_buf(&self, version: &str) -> PathBuf {
    let file_name = self.get_artifact_name(version);
    self
      .config
      .download_dir
      .join("node")
      .join(version)
      .join(file_name)
  }

  fn get_artifact_name(&self, version: &str) -> String {
    format!(
      "node-v{version}-{os}-{arch}.{ext}",
      version = version,
      os = self.config.platform.os,
      arch = self.config.platform.arch,
      ext = self.config.platform.ext
    )
  }

  fn get_download_url(&self, version: &str) -> String {
    format!(
      "{host}/v{version}/{artifact_name}",
      host = self.config.node_dist_url,
      version = version,
      artifact_name = self.get_artifact_name(version)
    )
  }
}
