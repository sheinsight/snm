use std::{env::current_dir, path::PathBuf};

use snm_npmrc::NpmrcReader;
use snm_test_utils::SnmTestContext;
use test_context::test_context;

fn build_path(current: &PathBuf, parts: &[&str]) -> String {
  parts
    .iter()
    .fold(current.to_path_buf(), |acc, part| acc.join(part))
    .to_string_lossy()
    .to_string()
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn should_read_custom_npm_registry(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let current = current_dir()?;

  let prefix_unix = build_path(&current, &["tests", "fixtures", "global", "unix"]);
  let prefix_win = build_path(&current, &["tests", "fixtures", "global", "win"]);

  ctx.vars(&[
    ("PREFIX".to_string(), prefix_unix),
    ("APPDATA".to_string(), prefix_win),
  ]);

  let workspace = current.join("tests").join("fixtures").join("project");

  let npmrc = NpmrcReader::from(&workspace);

  let registry = npmrc.read_registry_with_default();

  assert_eq!(registry, "https://project.com".to_string());

  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn should_read_global_npm_cache(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let current = current_dir()?;

  let prefix_unix = build_path(&current, &["tests", "fixtures", "global", "unix"]);
  let prefix_win = build_path(&current, &["tests", "fixtures", "global", "win"]);

  ctx.vars(&[
    ("PREFIX".to_string(), prefix_unix),
    ("APPDATA".to_string(), prefix_win),
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
