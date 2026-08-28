use std::env::current_dir;

use snm_test_utils::SnmTestContext;
use test_context::test_context;
use wiremock::{
  matchers::{method, path},
  Mock, MockServer, ResponseTemplate,
};

const PNPM_12_VERSION: &str = "12.0.0";
const NPM_REGISTRY: &str = "https://registry.npmjs.org";

async fn mirror_pnpm_12_wrapper(mock_server: &MockServer) -> anyhow::Result<()> {
  // 下载官方固定版本后转由本地服务器提供，模拟企业镜像只同步 pnpm 主包的真实场景。
  let metadata = reqwest::get(format!("{NPM_REGISTRY}/pnpm/{PNPM_12_VERSION}"))
    .await?
    .error_for_status()?
    .bytes()
    .await?;
  let archive = reqwest::get(format!("{NPM_REGISTRY}/pnpm/-/pnpm-{PNPM_12_VERSION}.tgz"))
    .await?
    .error_for_status()?
    .bytes()
    .await?;

  Mock::given(method("GET"))
    .and(path(format!("/pnpm/{PNPM_12_VERSION}")))
    .respond_with(ResponseTemplate::new(200).set_body_raw(metadata, "application/json"))
    .mount(mock_server)
    .await;
  Mock::given(method("GET"))
    .and(path(format!("/pnpm/-/pnpm-{PNPM_12_VERSION}.tgz")))
    .respond_with(ResponseTemplate::new(200).set_body_raw(archive, "application/octet-stream"))
    .mount(mock_server)
    .await;

  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_nested(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/nested");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.add_snapshot("node -v")?;
  // ✅ 这个可以，因为被 pnpm 劫持了， 就会像环境变量塞数据，
  ctx.exec("snm run test", true)?;
  // ctx.add_snapshot("snm run test")?;
  // ❌ 这个不行，因为没有被 pnpm 劫持，就拿不到 pm
  // ctx.exec("node node.cjs", true)?;
  // ❌ 需要特别注意的是只对当前环境生效，因此 pnpm -v && node node.cjs 不行的，因为不是同一个进程链路了
  // ctx.exec("pnpm -v && node node.cjs", true)?;

  // ctx.add_snapshot("node node.cjs")?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_nested_pnpx(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/nested");
  ctx.start_server().await?;
  ctx.set_cwd(&cwd);
  ctx.exec("snm setup", false)?;
  ctx.add_snapshot("node -v")?;
  // ✅ 这个可以，因为被 pnpm 劫持了， 就会像环境变量塞数据，
  ctx.exec("pnpx cowsay '22'", true)?;
  ctx.assert_snapshots(|res| {
    insta::assert_snapshot!(res);
  })?;
  Ok(())
}

#[test_context(SnmTestContext)]
#[tokio::test]
async fn test_pnpm_12_native_bootstrap_uses_its_own_registry(
  ctx: &mut SnmTestContext,
) -> anyhow::Result<()> {
  // 在临时目录创建项目，避免 pnpm 写入锁文件等运行状态时污染仓库中的固定 fixture。
  let cwd = ctx.get_temp_dir().join("pnpm-12-project");
  std::fs::create_dir_all(&cwd)?;
  std::fs::write(
    cwd.join("package.json"),
    format!(
      r#"{{
  "name": "pnpm-12-native-bootstrap",
  "private": true,
  "packageManager": "pnpm@{PNPM_12_VERSION}"
}}"#
    ),
  )?;
  let mock_server = ctx.start_server().await?;
  mirror_pnpm_12_wrapper(&mock_server).await?;
  ctx.set_cwd(&cwd);

  // 主包继续走本地企业镜像模拟源；不配置 Corepack registry 时，原生包应走 pnpm 官方默认源。
  ctx.remove_env("COREPACK_NPM_REGISTRY");
  ctx.remove_env("COREPACK_ENABLE_NETWORK");
  ctx.exec("snm setup", false)?;
  // pnpm 的 .mjs 引导入口必须由项目选定的 Node.js 启动，先建立明确的默认版本。
  let node_install = ctx.exec("snm node install 20.0.0", false)?;
  assert!(
    node_install.contains("status:0"),
    "failed to install the Node.js fixture:\n{node_install}"
  );
  let node_default = ctx.exec("snm node default 20.0.0", false)?;
  assert!(
    node_default.contains("status:0"),
    "failed to set the default Node.js fixture:\n{node_default}"
  );

  let first_run = ctx.exec("pnpm --version", false)?;
  assert!(
    first_run.contains("Downloading the pnpm 12.0.0 binary"),
    "pnpm 12 first run did not bootstrap the native binary:\n{first_run}"
  );
  assert!(
    first_run.contains("stdout:12.0.0"),
    "pnpm 12 first run returned an unexpected version:\n{first_run}"
  );
  assert!(
    first_run.contains("status:0"),
    "pnpm 12 first run failed:\n{first_run}"
  );

  let native_binary = ctx
    .get_temp_dir()
    .join(".snm/node_modules/pnpm/12.0.0")
    .join(if cfg!(windows) {
      "pnpm-native.exe"
    } else {
      "pnpm-native"
    });
  assert!(native_binary.is_file());

  // 禁用网络后仍能执行，证明第二次运行确实复用了首次下载的本地原生程序。
  ctx.set_envs(&[("COREPACK_ENABLE_NETWORK".to_string(), "0".to_string())]);
  let cached_run = ctx.exec("pnpm --version", false)?;
  assert!(
    !cached_run.contains("Downloading the pnpm 12.0.0 binary"),
    "pnpm 12 downloaded the native binary again instead of using its cache:\n{cached_run}"
  );
  assert!(
    cached_run.contains("stdout:12.0.0"),
    "cached pnpm 12 returned an unexpected version:\n{cached_run}"
  );
  assert!(
    cached_run.contains("status:0"),
    "cached pnpm 12 execution failed:\n{cached_run}"
  );

  // 删除缓存后指定测试源，失败信息必须指向该源，证明用户的 Corepack 专用配置仍拥有最高优先级。
  std::fs::remove_file(&native_binary)?;
  ctx.set_envs(&[
    ("COREPACK_ENABLE_NETWORK".to_string(), "1".to_string()),
    ("COREPACK_NPM_REGISTRY".to_string(), mock_server.uri()),
  ]);
  let configured_registry_run = ctx.exec("pnpm --version", false)?;
  assert!(
    configured_registry_run.contains(&mock_server.uri()),
    "pnpm 12 ignored the explicit Corepack registry:\n{configured_registry_run}"
  );
  assert!(
    configured_registry_run.contains("404 Not Found"),
    "the explicit Corepack registry did not receive the native package request:\n{configured_registry_run}"
  );
  assert!(
    configured_registry_run.contains("status:1"),
    "pnpm 12 unexpectedly succeeded against the intentionally incomplete registry:\n{configured_registry_run}"
  );

  Ok(())
}
