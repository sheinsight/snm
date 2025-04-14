use std::{path::PathBuf, str::FromStr};

use package_json_parser::PackageJsonParser;
use snm_config::snm_config::SnmConfig;
use snm_downloader::{DownloadPackageManagerResource, download_resource};
use up_finder::UpFinder;

use crate::PackageManager;

pub struct PackageManagerSetup {
  pub config: SnmConfig,
}

impl PackageManagerSetup {
  pub fn find_up_package_manager(&self) -> anyhow::Result<Option<PackageManager>> {
    let find_up = UpFinder::builder().cwd(&self.config.workspace).build();

    let files = find_up.find_up("package.json");

    if files.is_empty() {
      return Ok(None);
    }

    let Some(package_manager_raw) = files.iter().find_map(|item| {
      let Ok(package_json) = PackageJsonParser::parse(item) else {
        return None;
      };

      let Some(raw) = package_json.package_manager else {
        return None;
      };

      Some(raw)
    }) else {
      return Ok(None);
    };

    let package_manager = PackageManager::from_str(&package_manager_raw.0)?;

    Ok(Some(package_manager))
  }

  pub async fn ensure_package_manager(
    &self,
    package_manager: &PackageManager,
  ) -> anyhow::Result<PathBuf> {
    let mut dir = self
      .config
      .node_modules_dir
      .join(package_manager.name())
      .join(package_manager.version());
    let file = dir.join("package.json");
    if !file.try_exists()? {
      let resource = DownloadPackageManagerResource::builder()
        .config(&self.config)
        .bin_name(package_manager.name().to_string())
        .version(package_manager.version().to_string())
        .build();

      dir = download_resource(resource).await?;
    }
    Ok(dir)
  }
}
