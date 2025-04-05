use std::{
  path::{Path, PathBuf},
  time::Duration,
};

use anyhow::bail;
use hashery::Hashery;
use robust_downloader::RobustDownloader;
use snm_config::snm_config::SnmConfig;
use snm_utils::tarball::ArchiveExtension;
use tracing::trace;

use crate::pm_metadata::PackageManagerMetadata;

#[derive(serde::Deserialize)]
struct NpmResponse {
  dist: Dist,
}

#[derive(serde::Deserialize)]
struct Dist {
  shasum: String,
}

pub struct PackageManagerDownloader<'a> {
  metadata: &'a PackageManagerMetadata,
  snm_config: &'a SnmConfig,
}

impl<'a> PackageManagerDownloader<'a> {
  pub fn new(metadata: &'a PackageManagerMetadata, snm_config: &'a SnmConfig) -> Self {
    Self {
      metadata,
      snm_config,
    }
  }

  pub async fn download_pm(&self, version: &str) -> anyhow::Result<PathBuf> {
    let downloaded_file_path_buf = self.download(version).await?;

    self
      .verify_shasum(&downloaded_file_path_buf, version)
      .await?;

    let decompressed_dir_path_buf = self
      .snm_config
      .node_modules_dir
      .join(&self.metadata.full_name)
      .join(version);

    ArchiveExtension::from_path(downloaded_file_path_buf)?
      .decompress(&decompressed_dir_path_buf)?;

    Ok(decompressed_dir_path_buf)
  }

  async fn download(&self, version: &str) -> anyhow::Result<PathBuf> {
    let metadata = &self.metadata;

    let download_url = self.get_download_url(version);

    trace!("Start download from: {}", download_url);

    let downloaded_file = self
      .snm_config
      .download_dir
      .join(&metadata.full_name)
      .join(version)
      .join(format!("{}-{}.tgz", &metadata.full_name, version));

    let downloader = RobustDownloader::builder()
      .connect_timeout(Duration::from_secs(self.snm_config.download_timeout_secs))
      .max_concurrent(2)
      .build();

    downloader
      .download(vec![(download_url, &downloaded_file)])
      .await?;

    Ok(downloaded_file)
  }

  async fn verify_shasum<T: AsRef<Path>>(&self, file_path: T, version: &str) -> anyhow::Result<()> {
    let expect_shasum = self.get_expect_shasum(version).await?;

    let hasher = Hashery::builder()
      .algorithm(hashery::Algorithm::SHA1)
      .build();
    let actual_shasum = hasher.digest(file_path).await?;

    trace!(
      "Verify shasum: expect: {}, actual: {}",
      expect_shasum, actual_shasum
    );

    if expect_shasum != actual_shasum {
      bail!("SHASUM mismatch");
    }

    Ok(())
  }

  async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
    let url = self.get_expect_shasum_url(version);

    let resp = reqwest::get(&url).await?.json::<NpmResponse>().await?;

    trace!("Get expect shasum from: {} is {}", url, resp.dist.shasum);

    Ok(resp.dist.shasum)
  }

  fn get_expect_shasum_url(&self, version: &str) -> String {
    let metadata = &self.metadata;
    // https://registry.npmjs.org/react/0.0.1
    // https://registry.npmjs.org/@yarnpkg/cli-dist/2.4.1
    format!(
      "{host}/{library_name}/{version}",
      host = &self.snm_config.npm_registry,
      library_name = &metadata.full_name,
      version = version
    )
  }

  fn get_download_url(&self, version: &str) -> String {
    let metadata = &self.metadata;
    let npm_registry = &self.snm_config.npm_registry;

    // 使用 rsplit_once 直接获取最后一个部分，避免创建 Vec
    let file_name = metadata
      .full_name
      .rsplit_once('/')
      .map_or(metadata.full_name.clone(), |(_, name)| name.to_owned());

    format!(
      "{host}/{name}/-/{file}-{version}.tgz",
      host = npm_registry,
      name = metadata.full_name,
      file = file_name
    )
  }
}
