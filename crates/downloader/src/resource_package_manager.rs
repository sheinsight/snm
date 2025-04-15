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
  pub version: String,
}

impl<'a> DownloadResource for DownloadPackageManagerResource<'a> {
  fn get_download_url(&self) -> String {
    let (namespace, artifact) = match (
      self.bin_name.as_str(),
      ver_gt_1(&self.version).unwrap_or(false),
    ) {
      ("yarn" | "yarnpkg", true) => ("@yarnpkg/cli-dist", format!("cli-dist-{}", &self.version)),
      (name, _) => (name, format!("{}-{}", name, &self.version)),
    };

    format!("{}/{namespace}/-/{artifact}.tgz", self.config.npm_registry)
  }

  fn get_extract_path(&self) -> PathBuf {
    self
      .config
      .download_dir
      .join(&self.bin_name)
      .join(&self.version)
      .join(format!("{}-{}.tgz", &self.bin_name, &self.version))
  }

  fn get_timeout_secs(&self) -> Duration {
    Duration::from_secs(self.config.download_timeout_secs)
  }

  fn get_artifact_name(&self) -> String {
    format!("{}-{}.tgz", &self.bin_name, &self.version)
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
    self
      .config
      .node_modules_dir
      .join(&self.bin_name)
      .join(&self.version)
  }

  fn get_expect_shasum<'async_trait>(
    &self,
  ) -> Pin<Box<dyn Future<Output = anyhow::Result<Integrity>> + Send + 'async_trait>>
  where
    Self: 'async_trait,
  {
    let bin_name = self.bin_name.clone();
    let version = self.version.clone();
    let npm_registry = self.config.npm_registry.clone();
    let timeout = self.get_timeout_secs();

    Box::pin(async move {
      let namespace = match (bin_name.as_str(), ver_gt_1(&version).unwrap_or(false)) {
        ("yarn" | "yarnpkg", true) => "@yarnpkg/cli-dist",
        (name, _) => name,
      };

      let url = format!("{}/{namespace}/{}", npm_registry, &version);

      let client = reqwest::Client::builder().timeout(timeout).build()?;

      let resp = client.get(&url).send().await?.json::<NpmResponse>().await?;

      Ok(Integrity::SHA1(resp.dist.shasum))
    })
  }
}
