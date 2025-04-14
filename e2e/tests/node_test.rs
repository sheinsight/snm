use std::env::current_dir;

use snm_test_utils::SnmTestContext;
use test_context::test_context;

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_auto_install_node_with_node_version(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/auto_install_node_with_node_version");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.add_snapshot("node -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_show_node_version_with_strict_mode(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.set_envs(&[("SNM_STRICT".to_string(), "true".to_string())]);
  ctx.add_snapshot("node -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_no_strict_and_no_default_node(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.set_envs(&[("SNM_STRICT".to_string(), "false".to_string())]);
  ctx.add_snapshot("node -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_with_strict_mode_and_has_default_node(
  ctx: &mut SnmTestContext,
) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.set_envs(&[("SNM_STRICT".to_string(), "true".to_string())]);
  ctx.add_snapshot("snm node install 20.0.0")?;
  ctx.add_snapshot("snm node list --compact")?;
  ctx.add_snapshot("snm node default 20.0.0")?;
  ctx.add_snapshot("snm node list --compact")?;
  ctx.add_snapshot("node -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_local_node_list_is_empty(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.add_snapshot("snm node list --compact")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn should_show_node_version(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/new_line");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.add_snapshot("snm node list --compact")?;
  ctx.add_snapshot("node -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}
