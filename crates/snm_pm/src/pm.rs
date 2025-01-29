use std::{env, path::PathBuf};

use anyhow::bail;
use snm_config::snm_config::SnmConfig;
use snm_utils::consts::ENV_KEY_FOR_SNM_PM;

use crate::{
  ops::{
    npm::NpmCommandLine, ops::PackageManagerOps, pnpm::PnpmCommandLine, yarn::YarnCommandLine,
    yarn_berry::YarnBerryCommandLine,
  },
  package_json::PackageJson,
  pm_metadata::{PackageManagerHash, PackageManagerMetadata},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PackageManager {
  Npm(PackageManagerMetadata),
  Yarn(PackageManagerMetadata),
  YarnBerry(PackageManagerMetadata),
  Pnpm(PackageManagerMetadata),
}

impl PackageManager {
  // fn execute<F, T>(&self, callback: F) -> T
  // where
  //   F: Fn(&dyn PackageManagerOps) -> T,
  // {
  //   match self {
  //     Self::Npm(metadata) => callback(&NpmCommandLine::new(metadata)),
  //     Self::Yarn(metadata) => callback(&YarnCommandLine::new(metadata)),
  //     Self::YarnBerry(metadata) => callback(&YarnBerryCommandLine::new(metadata)),
  //     Self::Pnpm(metadata) => callback(&PnpmCommandLine::new(metadata)),
  //   }
  // }

  pub fn get_ops(&self) -> Box<dyn PackageManagerOps> {
    match self {
      Self::Npm(_) => Box::new(NpmCommandLine::new()),
      Self::Yarn(_) => Box::new(YarnCommandLine::new()),
      Self::YarnBerry(_) => Box::new(YarnBerryCommandLine::new()),
      Self::Pnpm(_) => Box::new(PnpmCommandLine::new()),
    }
  }

  // pub fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
  //   self.execute(|pm| pm.install(args.clone()))
  // }

  // pub fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
  //   self.execute(|pm| pm.remove(args.clone()))
  // }

  // pub fn run(&self, args: RunArgs) -> anyhow::Result<Vec<String>> {
  //   self.execute(|pm| pm.run(args.clone()))
  // }
}

impl PackageManager {
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

  pub fn try_from_env(config: &SnmConfig) -> anyhow::Result<Self> {
    let pm = Self::from_env();

    if pm.is_ok() {
      return pm;
    }

    let pm = Self::from(&config.workspace)?;

    Ok(pm)

    // PackageJson::from(&config.workspace)
    //   .ok()
    //   .and_then(|json| json.package_manager)
    //   .and_then(|raw| Self::parse(&raw, config).ok())
    //   .with_context(|| "Failed to determine package manager")
  }

  pub fn from_env() -> anyhow::Result<Self> {
    let raw = env::var(ENV_KEY_FOR_SNM_PM)?;
    Self::parse(&raw)
  }

  pub fn from(dir: &PathBuf) -> anyhow::Result<Self> {
    let package_json = PackageJson::from(dir)?;

    if let Some(raw) = package_json.package_manager {
      return Self::parse(&raw);
    }

    bail!("No package manager found");
  }

  pub fn from_str(raw: &str) -> anyhow::Result<Self> {
    Self::parse(raw)
  }

  pub fn parse(raw: &str) -> anyhow::Result<Self> {
    let pm = PackageManagerMetadata::from_str(&raw)?.into();

    Ok(pm)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_package_manager_with_pnpm() {
    let pm = PackageManager::parse("pnpm@9.0.0").expect("Should parse PNPM package manager");

    assert!(matches!(pm, PackageManager::Pnpm(_)));

    let info = match pm {
      PackageManager::Pnpm(a) => a,
      _ => panic!("Expected Pnpm variant"),
    };

    assert_eq!(info.full_name, "pnpm");
    assert_eq!(info.version, "9.0.0");
  }

  #[test]
  fn test_parse_package_manager_with_pnpm_and_hash() {
    let pm = PackageManager::parse("pnpm@9.0.0+sha.1234567890")
      .expect("Should parse PNPM package manager");

    assert_eq!(pm.full_name(), "pnpm");
    assert_eq!(pm.version(), "9.0.0");
    assert_eq!(pm.hash().unwrap().method, "sha");
    assert_eq!(pm.hash().unwrap().value, "1234567890");
  }

  #[test]
  fn test_parse_package_manager_with_npm() {
    let pm = PackageManager::parse("npm@10.0.0").expect("Should parse NPM package manager");

    assert_eq!(pm.full_name(), "npm");
    assert_eq!(pm.version(), "10.0.0");
  }
}
