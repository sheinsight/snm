use std::{path::PathBuf, str::FromStr};

use anyhow::bail;
use package_json_parser::PackageJsonParser;
use snm_config::snm_config::SnmConfig;
use snm_downloader::{DownloadPackageManagerResource, download_resource};
use up_finder::UpFinder;

use crate::PackageManager;

pub struct PackageManagerResolver {
  pub config: SnmConfig,
}

impl From<SnmConfig> for PackageManagerResolver {
  fn from(config: SnmConfig) -> Self {
    Self { config }
  }
}

impl PackageManagerResolver {
  pub fn find_up_package_manager(&self) -> anyhow::Result<PackageManager> {
    let find_up = UpFinder::builder().cwd(&self.config.workspace).build();

    let files = find_up.find_up("package.json");

    if files.is_empty() {
      if self.config.strict {
        bail!("You have not correctly configured packageManager in package.json");
      }
    }

    let package_manager =
      files
        .iter()
        .try_fold(None, |acc, item| -> anyhow::Result<Option<_>> {
          if acc.is_some() {
            return Ok(acc);
          }
          let package_json = PackageJsonParser::parse(item)?;

          Ok(package_json.package_manager)
        })?;

    if let Some(raw) = package_manager {
      let package_manager = PackageManager::from_str(&raw.0)?;

      Ok(package_manager)
    } else {
      if self.config.strict {
        bail!("You have not correctly configured packageManager in package.json");
      } else {
        bail!("You have not correctly configured packageManager in package.json");
      }
    }

    // let Some(package_manager_raw) = files.iter().find_map(|item| {
    //   let Ok(package_json) = PackageJsonParser::parse(item) else {
    //     return None;
    //   };

    //   let Some(raw) = package_json.package_manager else {
    //     return None;
    //   };

    //   Some(raw)
    // }) else {
    //   // TODO 未来可能在非严格模式下会尝试存在默认的包管理器
    //   if self.config.strict {
    //     bail!("You have not correctly configured packageManager in package.json");
    //   } else {
    //     bail!("You have not correctly configured packageManager in package.json");
    //   }
    // };
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
