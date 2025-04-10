use std::{path::PathBuf, str::FromStr};

use anyhow::bail;
use lazy_regex::regex_captures;
use snm_config::snm_config::SnmConfig;
use snm_downloader::{DownloadPackageManagerResource, download_resource};
use snm_utils::ver::ver_gt_1;
use strum::EnumString;
use up_finder::UpFinder;

use crate::{
  ops::{
    npm::NpmCommandLine, ops::PackageManagerOps, pnpm::PnpmCommandLine, yarn::YarnCommandLine,
    yarn_berry::YarnBerryCommandLine,
  },
  package_json::PJson,
};

#[derive(Debug, PartialEq, Eq, Clone, strum::Display, EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum PackageManagerKind {
  Npm,
  Yarn,
  Pnpm,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageManager {
  kind: PackageManagerKind,
  version: String,
}

impl PackageManager {
  pub fn get_ops(&self) -> Box<dyn PackageManagerOps> {
    match self.kind {
      PackageManagerKind::Npm => Box::new(NpmCommandLine::new()),
      PackageManagerKind::Yarn => {
        if ver_gt_1(&self.version).unwrap_or(false) {
          Box::new(YarnBerryCommandLine::new())
        } else {
          Box::new(YarnCommandLine::new())
        }
      }
      PackageManagerKind::Pnpm => Box::new(PnpmCommandLine::new()),
    }
  }
}

impl PackageManager {
  pub fn new(kind: PackageManagerKind, version: String) -> Self {
    Self { kind, version }
  }

  pub fn name(&self) -> &str {
    self.kind.as_ref()
  }

  pub fn version(&self) -> &str {
    &self.version
  }
}

impl FromStr for PackageManager {
  type Err = anyhow::Error;

  fn from_str(raw: &str) -> Result<Self, Self::Err> {
    let Some((_, name, version)) = regex_captures!(
      r#"^(?P<name>npm|yarn|pnpm)@(?P<version>[^+]+)(?:\+.+)?$"#,
      raw
    ) else {
      bail!("Failed to capture package manager: {}", raw);
    };

    let kind = PackageManagerKind::try_from(name)
      .map_err(|_| anyhow::anyhow!("Unsupported package manager: {}, Raw: {}", name, raw))?;

    Ok(PackageManager::new(kind, version.to_string()))
  }
}

#[derive(Debug)]
pub struct SPM<'a> {
  pub config: &'a SnmConfig,
  pub pm: PackageManager,
}

impl<'a> SPM<'a> {
  pub fn from_config_file(config: &'a SnmConfig) -> Option<Self> {
    let find_up = UpFinder::builder()
      .cwd(&config.workspace) // 从当前目录开始
      .build();

    let vecs = find_up.find_up("package.json");

    if vecs.is_empty() {
      return None;
    }

    let Some(spm) = vecs.iter().find_map(|item| {
      let Some(dir) = item.parent() else {
        return None;
      };

      let Some(package_json) = PJson::from(dir).ok() else {
        return None;
      };

      let Some(raw) = package_json.package_manager else {
        return None;
      };

      let Some(pm) = PackageManager::from_str(&raw).ok() else {
        return None;
      };

      Some(pm)
    }) else {
      return None;
    };

    Some(Self { config, pm: spm })
  }

  pub async fn ensure_bin_dir(&self) -> anyhow::Result<PathBuf> {
    let mut dir = self
      .config
      .node_modules_dir
      .join(self.pm.name())
      .join(self.pm.version());

    let file = dir.join("package.json");

    if !file.try_exists()? {
      let resource = DownloadPackageManagerResource::builder()
        .config(self.config)
        .bin_name(self.pm.name().to_string())
        .version(self.pm.version().to_string())
        .build();

      dir = download_resource(resource).await?;
    }

    Ok(dir)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_package_manager_with_pnpm() {
    let pm = PackageManager::from_str("pnpm@9.0.0").expect("Should parse PNPM package manager");

    assert!(matches!(
      pm,
      PackageManager {
        kind: PackageManagerKind::Pnpm,
        ..
      }
    ));

    assert_eq!(pm.name(), "pnpm");
    assert_eq!(pm.version(), "9.0.0");
  }

  #[test]
  fn test_parse_package_manager_with_pnpm_and_hash() {
    let pm = PackageManager::from_str("pnpm@9.0.0+sha.1234567890")
      .expect("Should parse PNPM package manager");

    assert_eq!(pm.name(), "pnpm");
    assert_eq!(pm.version(), "9.0.0");
  }

  #[test]
  fn test_parse_package_manager_with_npm() {
    let pm = PackageManager::from_str("npm@10.0.0").expect("Should parse NPM package manager");

    assert_eq!(pm.name(), "npm");
    assert_eq!(pm.version(), "10.0.0");
  }
}
