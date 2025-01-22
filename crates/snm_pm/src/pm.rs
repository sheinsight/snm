use std::{env, path::PathBuf};

use anyhow::{bail, Context};
use colored::Colorize;
use snm_config::SnmConfig;
use snm_utils::consts::ENV_KEY_FOR_SNM_PM;

use crate::{
  downloader::PackageManagerDownloader,
  ops::{
    npm::NpmCommandLine,
    ops::{AddArgs, InstallArgs, PackageManagerOps, RemoveArgs},
    pnpm::PnpmCommandLine,
    yarn::YarnCommandLine,
    yarn_berry::YarnBerryCommandLine,
  },
  package_json::PackageJson,
  pm_metadata::{PackageManagerHash, PackageManagerMetadata},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PackageManager<'a> {
  Npm(PackageManagerMetadata<'a>),
  Yarn(PackageManagerMetadata<'a>),
  YarnBerry(PackageManagerMetadata<'a>),
  Pnpm(PackageManagerMetadata<'a>),
}

impl<'a> From<PackageManagerMetadata<'a>> for PackageManager<'a> {
  fn from(metadata: PackageManagerMetadata<'a>) -> Self {
    match metadata.library_name.as_str() {
      "npm" => Self::Npm(metadata),
      "yarn" => Self::Yarn(metadata),
      "@yarnpkg/cli-dist" => Self::YarnBerry(metadata),
      "pnpm" => Self::Pnpm(metadata),
      _ => unreachable!(),
    }
  }
}

impl<'a> PackageManager<'a> {
  fn execute<F, T>(&self, callback: F) -> T
  where
    F: Fn(&dyn PackageManagerOps) -> T,
  {
    match self {
      Self::Npm(metadata) => callback(&NpmCommandLine::new(metadata)),
      Self::Yarn(metadata) => callback(&YarnCommandLine::new(metadata)),
      Self::YarnBerry(metadata) => callback(&YarnBerryCommandLine::new(metadata)),
      Self::Pnpm(metadata) => callback(&PnpmCommandLine::new(metadata)),
    }
  }

  pub fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
    self.execute(|pm| pm.install(args.clone()))
  }

  pub fn add(&self, args: AddArgs) -> anyhow::Result<Vec<String>> {
    self.execute(|pm| pm.add(args.clone()))
  }

  pub fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
    self.execute(|pm| pm.remove(args.clone()))
  }
}

impl<'a> PackageManager<'a> {
  fn metadata(&self) -> &PackageManagerMetadata<'a> {
    match self {
      Self::Npm(a) | Self::Yarn(a) | Self::YarnBerry(a) | Self::Pnpm(a) => a,
    }
  }

  pub fn library_name(&self) -> &str {
    self.metadata().library_name.as_str()
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

  pub fn try_from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
    let pm = Self::from_env(&config);

    if pm.is_ok() {
      return pm;
    }

    PackageJson::from(&config.workspace)
      .ok()
      .and_then(|json| json.package_manager)
      .and_then(|raw| Self::parse(&raw, config).ok())
      .with_context(|| "Failed to determine package manager")
  }

  pub fn from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
    let raw = env::var(ENV_KEY_FOR_SNM_PM)?;
    Self::parse(&raw, config)
  }

  pub fn from_str(raw: &str, config: &'a SnmConfig) -> anyhow::Result<Self> {
    Self::parse(raw, config)
  }

  pub fn parse(raw: &str, config: &'a SnmConfig) -> anyhow::Result<Self> {
    let metadata = PackageManagerMetadata::from_str(&raw, config)?;

    let package_manager = Self::from(metadata);

    Ok(package_manager)
  }
}

impl<'a> PackageManager<'a> {
  pub async fn get_bin(&self, args: &Vec<String>) -> anyhow::Result<PathBuf> {
    let actual_bin_name = args.get(0).context("bin name not found")?;

    let actual_bin_name = if actual_bin_name == "pnpx" {
      "pnpm"
    } else if actual_bin_name == "npx" {
      "npm"
    } else {
      actual_bin_name
    };

    let command = args.get(1).context("command not found")?;

    let metadata = self.metadata();

    let version = self.version();

    if metadata.config.restricted_list.contains(command) {
      bail!(
        "Package manager mismatch, expect: {}, actual: {} . Restricted list: {}",
        self.library_name().green(),
        actual_bin_name.red(),
        self.metadata().config.restricted_list.join(", ").black()
      );
    }

    if self.name() != actual_bin_name {
      bail!(
        "Package manager mismatch, expect: {}, actual: {}",
        self.library_name().green(),
        actual_bin_name.red()
      );
    }

    let pkg_dir = metadata
      .config
      .node_modules_dir
      .join(metadata.library_name.clone())
      .join(version);

    let pkg = pkg_dir.join("package.json");

    if pkg.try_exists()? {
      let json = PackageJson::from(pkg_dir)?;
      let bin_path_buf = json.get_bin_with_name(actual_bin_name)?;

      return Ok(bin_path_buf);
    };

    let decompressed_dir_path_buf = PackageManagerDownloader::new(metadata)
      .download_pm(version)
      .await?;

    let json = PackageJson::from(decompressed_dir_path_buf)?;

    let file = json.get_bin_with_name(actual_bin_name)?;

    Ok(file)
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use super::*;

  #[test]
  fn test_parse_package_manager_with_pnpm() {
    let config = SnmConfig::from(PathBuf::from(".")).unwrap();

    let pm =
      PackageManager::parse("pnpm@9.0.0", &config).expect("Should parse PNPM package manager");

    assert!(matches!(pm, PackageManager::Pnpm(_)));

    let info = match pm {
      PackageManager::Pnpm(a) => a,
      _ => panic!("Expected Pnpm variant"),
    };

    assert_eq!(info.library_name, "pnpm");
    assert_eq!(info.version, "9.0.0");
  }

  #[test]
  fn test_parse_package_manager_with_pnpm_and_hash() {
    let config = SnmConfig::from(PathBuf::from(".")).unwrap();

    let pm = PackageManager::parse("pnpm@9.0.0+sha.1234567890", &config)
      .expect("Should parse PNPM package manager");

    assert_eq!(pm.library_name(), "pnpm");
    assert_eq!(pm.version(), "9.0.0");
    assert_eq!(pm.hash().unwrap().method, "sha");
    assert_eq!(pm.hash().unwrap().value, "1234567890");
  }

  #[test]
  fn test_parse_package_manager_with_npm() {
    let config = SnmConfig::from(PathBuf::from(".")).unwrap();

    let pm =
      PackageManager::parse("npm@10.0.0", &config).expect("Should parse NPM package manager");

    assert_eq!(pm.library_name(), "npm");
    assert_eq!(pm.version(), "10.0.0");
  }
}
