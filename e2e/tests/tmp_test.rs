use std::env::current_dir;

use e2e::tmp_test;
use snm_test_utils::SnmTestContext;
use test_context::test_context;

#[test_context(SnmTestContext)]
#[tokio::test]
async fn h(ctx: &mut SnmTestContext) -> anyhow::Result<()> {
  let cwd = current_dir()?.join("tests/fixtures/empty");
  tmp_test::setup_http_server(ctx).await?;
  ctx.vars(&[
    ("SNM_STRICT".to_string(), "true".to_string()),
    (
      "SNM_HOME_DIR".to_string(),
      ctx.temp_dir().to_string_lossy().to_string(),
    ),
  ]);
  ctx.cwd(&cwd);
  ctx.add_snapshot("snm node install 20.0.0")?;
  ctx.add_snapshot("snm node list --compact")?;
  ctx.add_snapshot("snm node default 20.0.0")?;
  ctx.add_snapshot("snm node list --compact")?;
  ctx.add_snapshot("node -v")?;
  ctx.name("hhh");
  ctx.assert_snapshots(|name, res| {
    insta::assert_snapshot!(name, res);
  })?;
  Ok(())
}

// #[tokio::test]
// async fn test_reqwest_download() -> Result<(), Box<dyn std::error::Error>> {
//   let mock_server = e2e::http_mocker::HttpMocker::builder()?.build().await?;

//   let uri = mock_server.uri();

//   let cur = std::env::current_dir()?;

//   let config = snm_config::SnmConfig {
//     node_bin_dir: cur.join(".snm").join("node_bin"),
//     download_dir: cur.join(".snm").join("downloads"),
//     cache_dir: cur.join(".snm").join("cache"),
//     node_modules_dir: cur.join(".snm").join("node_modules"),
//     node_dist_url: uri,
//     node_github_resource_host: "https://raw.githubusercontent.com".to_string(),
//     node_install_strategy: snm_config::InstallStrategy::Auto,
//     node_white_list: "".to_string(),
//     download_timeout_secs: 30,
//     npm_registry: "https://registry.npmmirror.com".to_string(),
//     workspace: cur.join("tests").join("fixtures").join("empty"),
//     lang: "en".to_string(),
//     restricted_list: vec![],
//     strict: false,
//   };

//   let node_downloader = snm_node::downloader::NodeDownloader::new(&config);

//   let res = node_downloader.download("20.0.0").await?;

//   let node_bin_dir = config.node_bin_dir.join("20.0.0").join("bin").join("node");

//   println!("res---->: {:?}", res);
//   println!(
//     "node_bin_dir---->: {:?} {}",
//     node_bin_dir,
//     node_bin_dir.exists()
//   );

//   Ok(())
// }

// 或者使用 duct
// use duct::cmd;

// #[tokio::test]
// async fn test_download_node() -> Result<(), Box<dyn std::error::Error>> {
//   // let mock_server = e2e::http_mocker::HttpMocker::builder()?.build().await?;
//   // let uri = mock_server.uri();

//   // 获取 download_test 的路径
//   let exe_dir = std::env::current_exe()?
//     .parent()
//     .unwrap()
//     .parent()
//     .unwrap()
//     .to_path_buf();

//   #[cfg(windows)]
//   let download_test = exe_dir.join("download_test.exe");
//   #[cfg(not(windows))]
//   let download_test = exe_dir.join("download_test");

//   println!("Executing: {:?}", download_test);

//   // 执行 download_test
//   let output = cmd!(download_test)
//     .stdout_capture()
//     .stderr_capture()
//     .run()?;

//   println!("Output: {}", String::from_utf8_lossy(&output.stdout));
//   println!("Error: {}", String::from_utf8_lossy(&output.stderr));

//   Ok(())
// }

// e2e::test1! {
//   #[tokio::test(flavor = "current_thread")]
//   test_with_strict_mode_and_has_default_node,
//   cwd: std::env::current_dir()?.join("tests").join("fixtures").join("empty"),
//   envs: [e2e::SnmEnv::Strict("true".to_string())],
//   |builder:e2e::CommandBuilder| => {
//     let res = builder.exec("snm node install 20.0.0")?;
//     println!("res---->: {:?}", res);
//     builder.assert_snapshots(|name,res| {
//       insta::assert_snapshot!(name, res);
//     })?;
//   }
// }

// fn get_debug_dir() -> std::path::PathBuf {
//   // 获取 e2e 目录 (CARGO_MANIFEST_DIR 指向 e2e/Cargo.toml 所在目录)
//   let e2e_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

//   // 向上一级找到项目根目录
//   let root_dir = e2e_dir.parent().expect("Failed to get root dir");

//   // 进入 target/debug 目录
//   root_dir.join("target").join("debug")
// }

// #[tokio::test]
// async fn test_install_node() -> anyhow::Result<()> {
//   let mock_server = e2e::get_global_mock_server().await;

//   let uri = mock_server.uri();

//   let command = "snm node install 20.0.0";

//   let env_path = env!("PATH");

//   let debug_dir = dunce::canonicalize(get_debug_dir())?
//     .to_str()
//     .unwrap()
//     .to_string();

//   let path_separator = if cfg!(target_os = "windows") {
//     ";"
//   } else {
//     ":"
//   };
//   let new_path = format!("{}{}{}", debug_dir, path_separator, env_path);

//   let mut cmd = if cfg!(target_os = "windows") {
//     let command = if command.starts_with("snm") {
//       command.replace("snm", "snm.exe")
//     } else {
//       command.to_string()
//     };
//     let mut cmd = std::process::Command::new("cmd");
//     cmd.args(["/C", &command]);
//     cmd
//   } else {
//     let mut cmd = std::process::Command::new("sh");
//     cmd.args(["-c", &command]);
//     cmd
//   };

//   let output = cmd
//     .envs([
//       ("SNM_NODE_DIST_URL", uri.to_string()),
//       ("SNM_NPM_REGISTRY", uri.to_string()),
//       ("SNM_HOME_DIR", debug_dir),
//       ("PATH", new_path),
//       ("SNM_NODE_INSTALL_STRATEGY", "auto".to_string()),
//     ])
//     .current_dir(
//       std::env::current_dir()?
//         .join("tests")
//         .join("fixtures")
//         .join("empty"),
//     )
//     .output()?;

//   let res = if !output.status.success() {
//     String::from_utf8(output.stderr)?.trim().to_string()
//   } else {
//     String::from_utf8(output.stdout)?.trim().to_string()
//   };

//   // let builder = e2e::CommandBuilder::with_envs(
//   //   "test_install_node",
//   //   std::env::current_dir()?
//   //     .join("tests")
//   //     .join("fixtures")
//   //     .join("empty"),
//   //   vec![
//   //     e2e::SnmEnv::NodeDistUrl(uri.clone()),
//   //     e2e::SnmEnv::NpmRegistry(uri.clone()),
//   //   ],
//   // )?;

//   // builder.exec("snm setup")?;

//   // let res = builder.exec("snm node install 20.0.0")?;

//   println!("res---->: {:?}", res);

//   Ok(())
// }
