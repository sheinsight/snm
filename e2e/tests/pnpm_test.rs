use std::env::current_dir;

use snm_test_utils::SnmTestContext;
use test_context::test_context;

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_111(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/nested");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.add_snapshot("node -v")?;

  // ✅ 这个可以，因为被 pnpm 劫持了， 就会像环境变量塞数据，
  // ctx.exec("pnpm run node", true)?;
  // ❌ 这个不行，因为没有被 pnpm 劫持，就拿不到 pm
  ctx.exec("node node.cjs", true)?;
  // ❌ 需要特别注意的是只对当前环境生效，因此 pnpm -v && node node.cjs 不行的，因为不是同一个进程链路了
  // ctx.exec("pnpm -v && node node.cjs", true)?;

  // ctx.add_snapshot("node node.cjs")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}
