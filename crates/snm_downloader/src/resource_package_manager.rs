use std::{path::PathBuf, pin::Pin, time::Duration};

use robust_downloader::{DownloadItem, Integrity};
use snm_config::snm_config::SnmConfig;
use snm_utils::ver::ver_gt_1;
use typed_builder::TypedBuilder;

use crate::resource::DownloadResource;

#[derive(serde::Deserialize)]
struct NpmResponse {
  dist: Dist,
}

#[derive(serde::Deserialize)]
struct Dist {
  shasum: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct DownloadPackageManagerResource<'a> {
  pub config: &'a SnmConfig,
  pub bin_name: String,
}

impl<'a> DownloadResource for DownloadPackageManagerResource<'a> {
  fn get_download_url(&self, version: &str) -> String {
    let (namespace, artifact) = match (self.bin_name.as_str(), ver_gt_1(version).unwrap_or(false)) {
      ("yarn" | "yarnpkg", true) => ("@yarnpkg/cli-dist", format!("cli-dist-{version}")),
      (name, _) => (name, format!("{name}-{version}")),
    };

    format!("{}/{namespace}/-/{artifact}.tgz", self.config.npm_registry)
  }

  fn get_extract_path(&self, version: &str) -> PathBuf {
    self
      .config
      .download_dir
      .join(&self.bin_name)
      .join(version)
      .join(format!("{}-{}.tgz", &self.bin_name, version))
  }

  fn get_timeout_secs(&self) -> Duration {
    Duration::from_secs(self.config.download_timeout_secs)
  }

  fn get_artifact_name(&self, version: &str) -> String {
    format!("{}-{}.tgz", &self.bin_name, version)
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
    self
      .config
      .node_modules_dir
      .join(&self.bin_name)
      .join(version)
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
      let namespace = match (self.bin_name.as_str(), ver_gt_1(version).unwrap_or(false)) {
        ("yarn" | "yarnpkg", true) => "@yarnpkg/cli-dist",
        (name, _) => name,
      };

      let url = format!("{}/{namespace}/{version}", self.config.npm_registry);

      let client = reqwest::Client::builder()
        .timeout(self.get_timeout_secs())
        .build()?;

      let resp = client.get(&url).send().await?.json::<NpmResponse>().await?;

      Ok(Integrity::SHA1(resp.dist.shasum))
    })
  }
}
