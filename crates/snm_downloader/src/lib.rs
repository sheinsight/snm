use robust_downloader::RobustDownloader;
use std::path::PathBuf;
use tarball::ArchiveExtension;

mod resource;
mod resource_node;
mod resource_package_manager;
mod tarball;

pub use resource::DownloadResource;
pub use resource_node::DownloadNodeResource;
pub use resource_package_manager::DownloadPackageManagerResource;

pub async fn download_resource<R>(resource: R, version: &str) -> anyhow::Result<PathBuf>
where
  R: DownloadResource,
{
  let integrity = resource.get_expect_shasum(version).await?;

  let download_item = resource.get_download_item(version, Some(integrity));

  let downloader = RobustDownloader::builder().max_concurrent(2).build();

  downloader.download(vec![download_item.clone()]).await?;

  let decompress_dir = resource.get_decompress_dir(version);

  ArchiveExtension::from_path(download_item.target)?.decompress(&decompress_dir)?;

  Ok(decompress_dir)
}
