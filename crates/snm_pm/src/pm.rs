use std::path::PathBuf;

use anyhow::bail;
use snm_config::snm_config::SnmConfig;
use snm_utils::FindUp;

use crate::{
  downloader::PackageManagerDownloader,
  ops::{
    npm::NpmCommandLine, ops::PackageManagerOps, pnpm::PnpmCommandLine, yarn::YarnCommandLine,
    yarn_berry::YarnBerryCommandLine,
  },
  package_json::PJson,
  pm_metadata::{PackageManagerHash, PackageManagerMetadata},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PM {
  Npm(PackageManagerMetadata),
  Yarn(PackageManagerMetadata),
  YarnBerry(PackageManagerMetadata),
  Pnpm(PackageManagerMetadata),
}

impl PM {
  pub fn get_ops(&self) -> Box<dyn PackageManagerOps> {
    match self {
      Self::Npm(_) => Box::new(NpmCommandLine::new()),
      Self::Yarn(_) => Box::new(YarnCommandLine::new()),
      Self::YarnBerry(_) => Box::new(YarnBerryCommandLine::new()),
      Self::Pnpm(_) => Box::new(PnpmCommandLine::new()),
    }
  }
}

impl PM {
  pub fn metadata(&self) -> &PackageManagerMetadata {
    match self {
      Self::Npm(a) | Self::Yarn(a) | Self::YarnBerry(a) | Self::Pnpm(a) => a,
    }
  }

  pub fn full_name(&self) -> &str {
    self.metadata().full_name.as_str()
  }

  pub fn name(&self) -> &str {
    self.metadata().name.as_str()
  }

  pub fn version(&self) -> &str {
    self.metadata().version.as_str()
  }

  pub fn hash(&self) -> Option<&PackageManagerHash> {
    self.metadata().hash.as_ref()
  }

  pub fn parse(raw: &str) -> anyhow::Result<PM> {
    let pm = PackageManagerMetadata::from_str(&raw)?.into();
    Ok(pm)
  }
}

#[derive(Debug)]
pub struct SPM<'a> {
  pub config: &'a SnmConfig,
  pub pm: PM,
}

impl<'a> SPM<'a> {
  // pub fn try_from(dir: &PathBuf, config: &'a SnmConfig) -> anyhow::Result<Self> {
  //   Self::try_from_env(config).or_else(|_| Self::try_from_config_file(dir, config))
  // }

  // pub fn exists(dir: &PathBuf) -> anyhow::Result<bool> {
  //   let package_json = PJson::from(dir)?;
  //   Ok(package_json.package_manager.is_some())
  // }

  // pub fn try_from_str(raw: &str, config: &'a SnmConfig) -> anyhow::Result<Self> {
  //   let pm = PM::parse(raw)?;
  //   Ok(Self { config, pm })
  // }

  pub fn from_config_file(config: &'a SnmConfig) -> Option<Self> {
    let Ok(vecs) = FindUp::new(&config.workspace).find("package.json") else {
      return None;
    };

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

      let Some(pm) = PM::parse(&raw).ok() else {
        return None;
      };

      Some(pm)
    }) else {
      return None;
    };

    Some(Self { config, pm: spm })
  }

  // pub fn try_from_config_file(dir: &PathBuf, config: &'a SnmConfig) -> anyhow::Result<Self> {
  //   let package_json = PJson::from(dir)?;

  //   if let Some(raw) = package_json.package_manager {
  //     let pm = PM::parse(&raw)?;
  //     return Ok(Self { config, pm });
  //   }
  //   bail!("No package manager found");
  // }

  // fn try_from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
  //   let raw = env::var(ENV_KEY_FOR_SNM_PM)?;
  //   let pm = PM::parse(&raw)?;
  //   Ok(Self { config, pm })
  // }

  pub async fn ensure_bin_dir(&self) -> anyhow::Result<PathBuf> {
    // let spm = SPM::try_from(&self.config.workspace, self.config)?;

    let Some(spm) = SPM::from_config_file(&self.config) else {
      bail!("No package manager found");
    };

    let pm = spm.pm;

    let mut dir = self
      .config
      .node_modules_dir
      .join(pm.full_name())
      .join(pm.version());

    let file = dir.join("package.json");

    if !file.try_exists()? {
      dir = PackageManagerDownloader::new(pm.metadata(), self.config)
        .download_pm(pm.version())
        .await?;
    }

    Ok(dir)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_package_manager_with_pnpm() {
    let pm = PM::parse("pnpm@9.0.0").expect("Should parse PNPM package manager");

    assert!(matches!(pm, PM::Pnpm(_)));

    let info = match pm {
      PM::Pnpm(a) => a,
      _ => panic!("Expected Pnpm variant"),
    };

    assert_eq!(info.full_name, "pnpm");
    assert_eq!(info.version, "9.0.0");
  }

  #[test]
  fn test_parse_package_manager_with_pnpm_and_hash() {
    let pm = PM::parse("pnpm@9.0.0+sha.1234567890").expect("Should parse PNPM package manager");

    assert_eq!(pm.full_name(), "pnpm");
    assert_eq!(pm.version(), "9.0.0");
    assert_eq!(pm.hash().unwrap().method, "sha");
    assert_eq!(pm.hash().unwrap().value, "1234567890");
  }

  #[test]
  fn test_parse_package_manager_with_npm() {
    let pm = PM::parse("npm@10.0.0").expect("Should parse NPM package manager");

    assert_eq!(pm.full_name(), "npm");
    assert_eq!(pm.version(), "10.0.0");
  }
}
