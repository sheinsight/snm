// use std::env::current_dir;

// use snm_test_utils::SnmTestContext;
// use test_context::test_context;

// #[test_context(SnmTestContext)]
// #[tokio::test]
// async fn test_snm_install_with_node_20_npm(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
//   let cwd = current_dir()?.join("tests/fixtures/snm_i_with_node_npm");
//   ctx.start_server().await?;
//   ctx.set_cwd(&cwd);
//   ctx.exec("snm setup", true)?;
//   ctx.exec("snm node install 20.0.0", true)?;
//   ctx.exec("snm node default 20.0.0", true)?;
//   ctx.exec("npm -v", true)?;
//   // #[cfg(target_os = "windows")]
//   // {
//   //   ctx.add_snapshot("where npm")?;
//   //   ctx.add_snapshot("dir")?;
//   // }

//   // #[cfg(not(target_os = "windows"))]
//   // ctx.add_snapshot("which npm")?;
//   ctx.exec("npm install", true)?;
//   ctx.exec("node index.cjs", true)?;
//   // ctx.assert_snapshots(|res| {
//   //   insta::assert_snapshot!(res);
//   // })?;
//   Ok(())
// }
