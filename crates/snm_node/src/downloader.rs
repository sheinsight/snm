use std::{
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use sha2::{Digest, Sha256};
use snm_config::snm_config::SnmConfig;
use snm_utils::{
  download::{DownloadBuilder, WriteStrategy},
  tarball::ArchiveExtension,
  trace_if,
};
use tracing::trace;

pub struct NodeDownloader<'a> {
  config: &'a SnmConfig,
}

impl<'a> NodeDownloader<'a> {
  pub fn new(config: &'a SnmConfig) -> Self {
    Self { config }
  }

  pub async fn download(&self, version: &str) -> anyhow::Result<PathBuf> {
    let downloaded_file = self.download_node(&version).await?;

    self.verify_shasum(&downloaded_file, &version).await?;

    let node_dir = self.config.node_bin_dir.join(&version);

    ArchiveExtension::from_path(downloaded_file)?.decompress(&node_dir)?;

    Ok(node_dir)
  }

  async fn verify_shasum(&self, file_path: &PathBuf, version: &str) -> anyhow::Result<()> {
    let actual = self.get_actual_shasum(file_path).await?;
    let expected = self.get_expect_shasum(version).await?;

    trace_if!(|| {
      trace!("Verify shasum: expect: {}, actual: {}", expected, actual);
    });

    if actual != expected {
      bail!("Node binary shasum mismatch");
    }

    Ok(())
  }

  async fn get_actual_shasum<T: AsRef<Path>>(
    &self,
    downloaded_file_path_buf: T,
  ) -> anyhow::Result<String> {
    let file = File::open(downloaded_file_path_buf)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    std::io::copy(&mut reader, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
  }

  async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
    let sha256_url = format!(
      "{host}/v{version}/SHASUMS256.txt",
      host = self.config.node_dist_url,
      version = version
    );

    let file_name = self.get_file_name(version);

    let sha256_str = reqwest::get(&sha256_url).await?.text().await?;

    sha256_str
      .lines()
      .find_map(|line| {
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next()) {
          (Some(sha256), Some(file)) if file == file_name => Some(sha256.to_owned()),
          _ => None,
        }
      })
      .with_context(|| "Invalid Node SHASUM line format")
  }

  async fn download_node(&self, version: &str) -> anyhow::Result<PathBuf> {
    let download_url = self.get_download_url(version);
    let downloaded_file_path_buf = self.get_downloaded_path_buf(version);

    DownloadBuilder::new()?
      .retries(3)
      .timeout(self.config.download_timeout_secs)
      .write_strategy(WriteStrategy::WriteAfterDelete)
      .download(&download_url, &downloaded_file_path_buf)
      .await?;

    Ok(downloaded_file_path_buf)
  }

  fn get_downloaded_path_buf(&self, version: &str) -> PathBuf {
    let file_name = self.get_file_name(version);
    self
      .config
      .download_dir
      .join("node")
      .join(version)
      .join(file_name)
  }

  fn get_file_name(&self, version: &str) -> String {
    format!(
      "node-v{version}-{os}-{arch}.{ext}",
      version = version,
      os = snm_utils::consts::os(),
      arch = snm_utils::consts::arch(),
      ext = snm_utils::consts::ext()
    )
  }

  fn get_download_url(&self, version: &str) -> String {
    format!(
      "{host}/v{version}/node-v{version}-{os}-{arch}.{ext}",
      host = self.config.node_dist_url,
      version = version,
      os = snm_utils::consts::os(),
      arch = snm_utils::consts::arch(),
      ext = snm_utils::consts::ext()
    )
  }
}
