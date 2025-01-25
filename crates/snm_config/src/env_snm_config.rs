use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct EnvSnmConfig {
  pub lang: Option<String>,

  pub home_dir: Option<String>,

  pub restricted_list: Option<String>,

  pub node_dist_url: Option<String>,

  pub node_github_resource_host: Option<String>,

  pub node_white_list: Option<String>,

  pub download_timeout_secs: Option<u64>,

  pub npm_registry: Option<String>,

  pub strict: Option<bool>,
}

impl EnvSnmConfig {
  pub fn parse() -> anyhow::Result<Self> {
    let config = Config::builder()
      .add_source(Environment::with_prefix("SNM"))
      .build()?;

    let config: EnvSnmConfig = config.try_deserialize()?;
    Ok(config)
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use super::*;

  #[tokio::test]
  async fn should_parse_env_snm_config() -> anyhow::Result<()> {
    env::set_var("SNM_LANG", "en");
    env::set_var("SNM_HOME_DIR", "");
    env::set_var("SNM_RESTRICTED_LIST", "install");
    env::set_var("SNM_NODE_DIST_URL", "https://nodejs.org/dist");
    env::set_var(
      "SNM_NODE_GITHUB_RESOURCE_HOST",
      "https://raw.githubusercontent.com",
    );
    env::set_var("SNM_NODE_WHITE_LIST", "1.1.0,1.2.0");
    env::set_var("SNM_DOWNLOAD_TIMEOUT_SECS", "60");
    env::set_var("SNM_NPM_REGISTRY", "https://test.npmjs.org");
    env::set_var("SNM_STRICT", "true");

    let config = EnvSnmConfig::parse().unwrap();

    assert_eq!(config.lang, Some("en".to_string()));
    assert_eq!(config.home_dir, Some("".to_string()));
    assert_eq!(config.restricted_list, Some("install".to_string()));
    assert_eq!(
      config.node_dist_url,
      Some("https://nodejs.org/dist".to_string())
    );
    assert_eq!(
      config.node_github_resource_host,
      Some("https://raw.githubusercontent.com".to_string())
    );
    assert_eq!(config.node_white_list, Some("1.1.0,1.2.0".to_string()));
    assert_eq!(config.download_timeout_secs, Some(60));
    assert_eq!(
      config.npm_registry,
      Some("https://test.npmjs.org".to_string())
    );
    assert_eq!(config.strict, Some(true));

    Ok(())
  }
}
