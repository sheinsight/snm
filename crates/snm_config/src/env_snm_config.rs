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
  use std::env;

  use test_context::{test_context, AsyncTestContext};

  use super::*;

  pub const SNM_PREFIX: &str = "ENV_SNM_CONFIG_SNM";

  struct EnvTestContext {}

  impl AsyncTestContext for EnvTestContext {
    fn teardown(self) -> impl std::future::Future<Output = ()> + Send {
      async {
        env::remove_var(format!("{}_LANG", SNM_PREFIX));
        env::remove_var(format!("{}_HOME_DIR", SNM_PREFIX));
        env::remove_var(format!("{}_RESTRICTED_LIST", SNM_PREFIX));
        env::remove_var(format!("{}_NODE_DIST_URL", SNM_PREFIX));
        env::remove_var(format!("{}_NODE_GITHUB_RESOURCE_HOST", SNM_PREFIX));
        env::remove_var(format!("{}_NODE_WHITE_LIST", SNM_PREFIX));
        env::remove_var(format!("{}_DOWNLOAD_TIMEOUT_SECS", SNM_PREFIX));
        env::remove_var(format!("{}_NPM_REGISTRY", SNM_PREFIX));
        env::remove_var(format!("{}_STRICT", SNM_PREFIX));
      }
    }

    fn setup() -> impl std::future::Future<Output = Self> + Send {
      async {
        let home = dirs::home_dir().unwrap();
        env::set_var(format!("{}_LANG", SNM_PREFIX), "en");
        env::set_var(
          format!("{}_HOME_DIR", SNM_PREFIX),
          home.to_string_lossy().to_string(),
        );
        env::set_var(format!("{}_RESTRICTED_LIST", SNM_PREFIX), "install");
        env::set_var(
          format!("{}_NODE_DIST_URL", SNM_PREFIX),
          "https://nodejs.org/dist",
        );
        env::set_var(
          format!("{}_NODE_GITHUB_RESOURCE_HOST", SNM_PREFIX),
          "https://raw.githubusercontent.com",
        );
        env::set_var(format!("{}_NODE_WHITE_LIST", SNM_PREFIX), "1.1.0,1.2.0");
        env::set_var(format!("{}_DOWNLOAD_TIMEOUT_SECS", SNM_PREFIX), "60");
        env::set_var(
          format!("{}_NPM_REGISTRY", SNM_PREFIX),
          "https://test.npmjs.org",
        );
        env::set_var(format!("{}_STRICT", SNM_PREFIX), "true");
        Self {}
      }
    }
  }

  #[test_context(EnvTestContext)]
  #[tokio::test]
  async fn should_parse_env_snm_config(_ctx: &mut EnvTestContext) -> anyhow::Result<()> {
    let config = EnvSnmConfig::parse(SNM_PREFIX).unwrap();
    let home = dirs::home_dir().unwrap();
    assert_eq!(config.lang, Some("en".to_string()));
    assert_eq!(config.home_dir, Some(home.to_string_lossy().to_string()));
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
