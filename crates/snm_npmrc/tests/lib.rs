use std::{
  collections::HashMap,
  env::{self, current_dir},
};

use snm_npmrc::NpmrcReader;
use test_context::{test_context, AsyncTestContext};

struct NpmrcTestContext {
  // temp_dir: tempfile::TempDir,
  env_vars: HashMap<String, String>,
}

impl AsyncTestContext for NpmrcTestContext {
  async fn setup() -> Self {
    // let temp_dir = tempfile::tempdir().unwrap();
    Self {
      // temp_dir,
      env_vars: HashMap::new(),
    }
  }

  async fn teardown(self) {
    // 清理所有设置的环境变量
    for (key, _) in self.env_vars {
      env::remove_var(key);
    }
  }
}

impl NpmrcTestContext {
  fn vars(&mut self, envs: &[(String, String)]) {
    for (key, value) in envs {
      self.env_vars.insert(key.to_string(), value.to_string());
      env::set_var(key, value);
    }
  }
}

#[test_context(NpmrcTestContext)]
#[tokio::test]
async fn should_read_custom_npm_registry(ctx: &mut NpmrcTestContext) -> anyhow::Result<()> {
  let current = current_dir()?;

  let global_prefix = current
    .join("tests")
    .join("fixtures")
    .join("global")
    .to_string_lossy()
    .to_string();

  ctx.vars(&[("PREFIX".to_string(), global_prefix)]);

  let workspace = current.join("tests").join("fixtures").join("project");

  let npmrc = NpmrcReader::from(&workspace);

  let registry = npmrc.read_registry_with_default();

  assert_eq!(registry, "https://project.com".to_string());

  // let cache = npmrc.read("cache");

  // #[cfg(target_os = "windows")]
  // assert_eq!(cache, None);

  // #[cfg(not(target_os = "windows"))]
  // assert_eq!(cache, Some("~/.hello".to_string()));

  Ok(())
}

#[test_context(NpmrcTestContext)]
#[tokio::test]
async fn should_read_global_npm_cache(ctx: &mut NpmrcTestContext) -> anyhow::Result<()> {
  let current = current_dir()?;

  let prefix_unix = current
    .join("tests")
    .join("fixtures")
    .join("global")
    .join("unix")
    .to_string_lossy()
    .to_string();

  let prefix_win = current
    .join("tests")
    .join("fixtures")
    .join("global")
    .join("win")
    .to_string_lossy()
    .to_string();

  ctx.vars(&[
    ("PREFIX".to_string(), prefix_unix),
    ("APPDATA".to_string(), prefix_win.to_string()),
  ]);

  let workspace = current.join("tests").join("fixtures").join("project");

  let npmrc = NpmrcReader::from(&workspace);

  let cache = npmrc.read("cache");

  #[cfg(not(target_os = "windows"))]
  assert_eq!(cache, Some("~/.hello".to_string()));

  #[cfg(target_os = "windows")]
  assert_eq!(cache, Some("~/.win-hello".to_string()));

  Ok(())
}
