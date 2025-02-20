use std::{
  fmt::Display,
  fs,
  path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use snm_npmrc::NpmrcReader;

use crate::env_snm_config::EnvSnmConfig;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Serialize)]
pub struct SnmConfig {
  pub node_bin_dir: PathBuf,
  pub download_dir: PathBuf,
  pub node_modules_dir: PathBuf,
  pub node_dist_url: String,
  pub node_github_resource_host: String,
  pub node_white_list: String,
  pub download_timeout_secs: u64,
  pub npm_registry: String,
  pub workspace: PathBuf,
  pub strict: bool,
}

impl Display for SnmConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Ok(json) = serde_json::to_string_pretty(self) {
      return write!(f, "{}", json);
    }
    write!(f, "{:?}", self)
  }
}

impl SnmConfig {
  pub fn from<P: AsRef<Path>>(prefix: &str, workspace: P) -> anyhow::Result<Self> {
    let config = EnvSnmConfig::parse(prefix)?;

    let npm_registry = config
      .npm_registry
      .unwrap_or_else(|| NpmrcReader::from(&workspace).read_registry_with_default());

    let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!("Get home dir error"))?;

    let base_dir = config
      .home_dir
      .map(PathBuf::from)
      .unwrap_or(home_dir)
      .join(".snm");

    let node_bin_dir = base_dir.join(String::from("node_bin"));
    let download_dir = base_dir.join(String::from("downloads"));
    let node_modules_dir = base_dir.join(String::from("node_modules"));

    for dir in [&base_dir, &node_bin_dir, &download_dir, &node_modules_dir] {
      if !dir.try_exists()? {
        fs::create_dir_all(dir)?;
      }
    }

    let node_dist_url = config
      .node_dist_url
      .unwrap_or(String::from("https://nodejs.org/dist"));

    let node_github_resource_host = config
      .node_github_resource_host
      .unwrap_or(String::from("https://raw.githubusercontent.com"));

    let node_white_list = config.node_white_list.unwrap_or(String::from(""));

    let strict = config.strict.unwrap_or(false);

    Ok(Self {
      workspace: workspace.as_ref().to_path_buf(),
      node_bin_dir: node_bin_dir,
      download_dir: download_dir,
      node_modules_dir: node_modules_dir,
      node_dist_url: node_dist_url,
      node_github_resource_host: node_github_resource_host,
      node_white_list: node_white_list,
      download_timeout_secs: config.download_timeout_secs.unwrap_or(30),
      npm_registry: npm_registry,
      strict: strict,
    })
  }
}

#[cfg(test)]
mod tests {
  use snm_test_utils::SnmTestContext;
  use test_context::test_context;

  use super::*;

  #[test_context(SnmTestContext)]
  #[tokio::test]
  async fn should_parse_snm_config(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
    ctx.set_envs(&[(
      format!("{}_HOME_DIR", ctx.get_id()),
      ctx.get_temp_dir().to_string_lossy().to_string(),
    )]);

    let config = SnmConfig::from(ctx.get_id(), ctx.get_temp_dir())?;

    assert_eq!(
      config.node_bin_dir,
      ctx.get_temp_dir().join(".snm/node_bin")
    );
    assert_eq!(
      config.download_dir,
      ctx.get_temp_dir().join(".snm/downloads")
    );
    assert_eq!(
      config.node_modules_dir,
      ctx.get_temp_dir().join(".snm/node_modules")
    );
    assert_eq!(config.node_dist_url, "https://nodejs.org/dist");
    assert_eq!(
      config.node_github_resource_host,
      "https://raw.githubusercontent.com"
    );
    assert_eq!(config.node_white_list, "");
    assert_eq!(config.download_timeout_secs, 30);
    assert_eq!(config.npm_registry, "https://registry.npmjs.org");
    assert_eq!(config.strict, false);

    Ok(())
  }
}
