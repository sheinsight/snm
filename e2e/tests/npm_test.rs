use std::env::current_dir;

use snm_test_utils::SnmTestContext;
use test_context::test_context;

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_show_npm_version_when_missing_default_node_and_npm(
  ctx: &mut SnmTestContext,
) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup")?;
  ctx.add_snapshot("npm -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_show_npm_version_when_default_npm_missing_but_node_exists(
  ctx: &mut SnmTestContext,
) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup")?;
  ctx.add_snapshot("snm node install 20.0.0")?;
  ctx.add_snapshot("snm node default 20.0.0")?;
  ctx.add_snapshot("npm -v")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}
