use std::{path::PathBuf, pin::Pin, time::Duration};

use robust_downloader::{DownloadItem, Integrity};

pub trait DownloadResource {
  /// 获取下载项配置
  fn get_download_url(&self, version: &str) -> String;

  /// 获取解压目标路径
  fn get_extract_path(&self, version: &str) -> PathBuf;

  /// 获取下载超时时间（秒）
  fn get_timeout_secs(&self) -> Duration;

  fn get_artifact_name(&self, version: &str) -> String;

  fn get_download_item(
    &self,
    version: &str,
    integrity: Option<Integrity>,
  ) -> DownloadItem<String, PathBuf>;

  fn get_decompress_dir(&self, version: &str) -> PathBuf;

  fn get_expect_shasum<'life0, 'async_trait>(
    &'life0 self,
    version: &'life0 str,
  ) -> Pin<Box<dyn Future<Output = anyhow::Result<Integrity>> + Send + 'async_trait>>
  where
    Self: 'async_trait,
    'life0: 'async_trait;
}
