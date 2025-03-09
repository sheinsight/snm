// use std::env::current_dir;

// use snm_test_utils::SnmTestContext;
// use test_context::test_context;

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_install_node(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/empty");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", false)?;
//   ctx.add_snapshot("snm node install 20.0.0")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.assert_snapshots(|res| {
//     insta::assert_snapshot!(res);
//   })?;
//   Ok(())
// }

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_uninstall_node(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/empty");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", false)?;
//   ctx.add_snapshot("snm node install 20.0.0")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.add_snapshot("snm node uninstall 20.0.0")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.assert_snapshots(|res| {
//     insta::assert_snapshot!(res);
//   })?;
//   Ok(())
// }

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_set_default_node(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/empty");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", false)?;
//   ctx.add_snapshot("snm node install 20.0.0")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.add_snapshot("snm node default 20.0.0")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.assert_snapshots(|res| {
//     insta::assert_snapshot!(res);
//   })?;
//   Ok(())
// }

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_list(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/empty");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", false)?;
//   ctx.add_snapshot("snm node install 20.0.0")?;
//   ctx.add_snapshot("snm node list")?;
//   ctx.add_snapshot("snm node default 20.0.0")?;
//   ctx.add_snapshot("snm node list")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.add_snapshot("snm node list --remote")?;
//   ctx.assert_snapshots(|res| {
//     insta::assert_snapshot!(res);
//   })?;
//   Ok(())
// }

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_list_with_strict_mode(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/empty");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", false)?;
//   ctx.set_envs(&[("SNM_STRICT".to_string(), "true".to_string())]);
//   ctx.add_snapshot("snm node install 20.0.0")?;
//   ctx.add_snapshot("snm node list")?;
//   ctx.add_snapshot("snm node default 20.0.0")?;
//   ctx.add_snapshot("snm node list")?;
//   ctx.add_snapshot("snm node list --compact")?;
//   ctx.add_snapshot("snm node list --remote")?;
//   ctx.assert_snapshots(|res| {
//     insta::assert_snapshot!(res);
//   })?;
//   Ok(())
// }

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_install_with_node_20_npm(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/snm_i_with_node_npm");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", false)?;
//   ctx.add_snapshot("snm node install 20.0.0")?;
//   ctx.add_snapshot("snm node default 20.0.0")?;
//   ctx.add_snapshot("npm -v")?;
//   // #[cfg(target_os = "windows")]
//   // {
//   //   ctx.add_snapshot("where npm")?;
//   //   ctx.add_snapshot("dir")?;
//   // }

//   // #[cfg(not(target_os = "windows"))]
//   // ctx.add_snapshot("which npm")?;
//   ctx.exec("npm install", true)?;
//   ctx.add_snapshot("node index.cjs")?;
//   ctx.assert_snapshots(|res| {
//     insta::assert_snapshot!(res);
//   })?;
//   Ok(())
// }
