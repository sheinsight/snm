use std::{
  env,
  fmt::Display,
  fs,
  path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use snm_npmrc::NpmrcReader;

use crate::env_snm_config::EnvSnmConfig;

const SNM_HOME_DIR_KEY: &str = "SNM_HOME_DIR";

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
  pub lang: String,
  pub restricted_list: Vec<String>,
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
  fn get_home_dir() -> anyhow::Result<PathBuf> {
    match env::var(SNM_HOME_DIR_KEY) {
      Ok(dir) => {
        if dir.is_empty() {
          return dirs::home_dir().ok_or(anyhow::anyhow!("Get home dir error"));
        }
        Ok(PathBuf::from(dir))
      }
      Err(_) => dirs::home_dir().ok_or(anyhow::anyhow!("Get home dir error")),
    }
  }

  pub fn from<P: AsRef<Path>>(prefix: &str, workspace: P) -> anyhow::Result<Self> {
    let config = EnvSnmConfig::parse(prefix)?;

    let npm_registry = config
      .npm_registry
      .unwrap_or_else(|| NpmrcReader::from(&workspace).read_registry_with_default());

    let base_dir = Self::get_home_dir()?.join(".snm");

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

    let restricted_list = {
      let map =
        |v: String| -> Vec<String> { v.split(',').map(|s| s.to_string()).collect::<Vec<String>>() };
      config.restricted_list.map(map).unwrap_or(Vec::new())
    };

    let strict = config.strict.unwrap_or(false);

    Ok(Self {
      restricted_list,
      workspace: workspace.as_ref().to_path_buf(),
      node_bin_dir: node_bin_dir,
      download_dir: download_dir,
      lang: config.lang.unwrap_or("en".to_string()),
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
  use test_context::{test_context, AsyncTestContext};

  use super::*;

  struct EnvTestContext {}

  impl AsyncTestContext for EnvTestContext {
    fn teardown(self) -> impl std::future::Future<Output = ()> + Send {
      async {
        env::remove_var(SNM_HOME_DIR_KEY);
      }
    }

    fn setup() -> impl std::future::Future<Output = Self> + Send {
      async {
        let home = dirs::home_dir().unwrap();
        env::set_var(SNM_HOME_DIR_KEY, home.to_string_lossy().to_string());
        Self {}
      }
    }
  }

  #[test_context(EnvTestContext)]
  #[tokio::test]
  async fn should_parse_snm_config(_ctx: &mut EnvTestContext) -> anyhow::Result<()> {
    let config = SnmConfig::from("SNM1", ".")?;

    let home = dirs::home_dir().unwrap();

    // let e = env::var(SNM_HOME_DIR_KEY)?;

    // println!("config: {:?}", config);
    // println!("e: {:?}", e);

    assert_eq!(config.node_bin_dir, home.join(".snm/node_bin"));
    assert_eq!(config.download_dir, home.join(".snm/downloads"));
    assert_eq!(config.node_modules_dir, home.join(".snm/node_modules"));
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
