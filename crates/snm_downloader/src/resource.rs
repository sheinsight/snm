use std::{path::PathBuf, pin::Pin, time::Duration};

use robust_downloader::{DownloadItem, Integrity};

pub trait DownloadResource {
  /// 获取下载项配置
  fn get_download_url(&self) -> String;

  /// 获取解压目标路径
  fn get_extract_path(&self) -> PathBuf;

  /// 获取下载超时时间（秒）
  fn get_timeout_secs(&self) -> Duration;

  fn get_artifact_name(&self) -> String;

  fn get_download_item(&self, integrity: Option<Integrity>) -> DownloadItem<String, PathBuf>;

  fn get_decompress_dir(&self) -> PathBuf;

  fn get_expect_shasum<'async_trait>(
    &self,
  ) -> Pin<Box<dyn Future<Output = anyhow::Result<Integrity>> + Send + 'async_trait>>
  where
    Self: 'async_trait;
}
