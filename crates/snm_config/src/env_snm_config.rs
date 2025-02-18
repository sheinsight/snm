use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct EnvSnmConfig {
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
  pub fn parse(prefix: &str) -> anyhow::Result<Self> {
    let config = Config::builder()
      .add_source(Environment::with_prefix(prefix))
      .build()?;

    let config: EnvSnmConfig = config.try_deserialize()?;
    Ok(config)
  }
}

#[cfg(test)]
mod tests {

  use snm_test_utils::SnmTestContext;
  use test_context::test_context;

  use super::*;

  #[test_context(SnmTestContext)]
  #[tokio::test]
  async fn should_parse_env_snm_config(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
    let home_dir = ctx.get_temp_dir().to_string_lossy().to_string();
    let restricted_list = "install";
    let node_dist_url = "https://nodejs.org/dist";
    let node_github_resource_host = "https://raw.githubusercontent.com";
    let node_white_list = "1.1.0,1.2.0";
    let download_timeout_secs = 60;
    let npm_registry = "https://test.npmjs.org";
    let strict = true;

    let envs = [
      (format!("{}_HOME_DIR", ctx.get_id()), home_dir.clone()),
      (
        format!("{}_RESTRICTED_LIST", ctx.get_id()),
        restricted_list.to_string(),
      ),
      (
        format!("{}_NODE_DIST_URL", ctx.get_id()),
        node_dist_url.to_string(),
      ),
      (
        format!("{}_NODE_GITHUB_RESOURCE_HOST", ctx.get_id()),
        node_github_resource_host.to_string(),
      ),
      (
        format!("{}_NODE_WHITE_LIST", ctx.get_id()),
        node_white_list.to_string(),
      ),
      (
        format!("{}_DOWNLOAD_TIMEOUT_SECS", ctx.get_id()),
        download_timeout_secs.to_string(),
      ),
      (
        format!("{}_NPM_REGISTRY", ctx.get_id()),
        npm_registry.to_string(),
      ),
      (format!("{}_STRICT", ctx.get_id()), strict.to_string()),
    ];

    ctx.set_envs(&envs);

    let config = EnvSnmConfig::parse(ctx.get_id())?;

    assert_eq!(config.home_dir, Some(home_dir.clone()));
    assert_eq!(config.restricted_list, Some(restricted_list.to_string()));
    assert_eq!(config.node_dist_url, Some(node_dist_url.to_string()));
    assert_eq!(
      config.node_github_resource_host,
      Some(node_github_resource_host.to_string())
    );
    assert_eq!(config.node_white_list, Some(node_white_list.to_string()));
    assert_eq!(config.download_timeout_secs, Some(download_timeout_secs));
    assert_eq!(config.npm_registry, Some(npm_registry.to_string()));
    assert_eq!(config.strict, Some(strict));

    Ok(())
  }
}
