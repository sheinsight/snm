// use std::env::current_dir;

// // use duct::cmd;
// use e2e::SnmEnv;

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_node,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_uninstall_node,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.add_snapshot("snm node uninstall 20.0.0")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_set_default_node,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("node -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_list,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.add_snapshot("snm node list --remote")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_list_with_strict_mode,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[SnmEnv::Strict("true".to_string())],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.add_snapshot("snm node list --remote")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_set_default_pnpm,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("pnpm -v")?;
//         builder.add_snapshot("snm pnpm install 9.0.0")?;
//         builder.add_snapshot("snm pnpm default 9.0.0")?;
//         builder.add_snapshot("pnpm -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_set_default_npm_with_node_20,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.add_snapshot("snm npm install 9.0.0")?;
//         builder.add_snapshot("snm npm default 9.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_set_default_yarn,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("yarn -v")?;
//         builder.add_snapshot("snm yarn install 1.22.22")?;
//         builder.add_snapshot("snm yarn default 1.22.22")?;
//         builder.add_snapshot("yarn -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_set_default_yarn4,
//     cwd:current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("yarn -v")?;
//         builder.add_snapshot("snm yarn install 4.0.0")?;
//         builder.add_snapshot("snm yarn default 4.0.0")?;
//         builder.add_snapshot("yarn -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_with_node_20_npm,
//     cwd: current_dir()?.join("tests").join("fixtures").join("snm_i_with_node_npm"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.exec("npm install")?;
//         builder.add_snapshot("node index.cjs")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_with_outside_npm,
//     cwd:current_dir()?
//     .join("tests")
//     .join("fixtures")
//     .join("test_snm_install_with_outside_pnpm"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.add_snapshot("snm npm install 9.0.0")?;
//         builder.add_snapshot("snm npm default 9.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.exec("npm install")?;
//         builder.add_snapshot("node index.cjs")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_when_node_modules_has_other_pm,
//     cwd: current_dir()?.join("tests").join("fixtures").join("test_when_node_modules_has_other_pm"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.exec("snm node install 20.0.0")?;
//         builder.exec("snm node default 20.0.0")?;
//         builder.exec("npm -v")?;
//         builder.add_snapshot("snm npm install 9.0.0")?;
//         builder.add_snapshot("snm npm default 9.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// // #[tokio::test]
// // async fn test_reqwest_download() -> Result<(), Box<dyn std::error::Error>> {
// //   let mock_server = e2e::http_mocker::HttpMocker::builder()?.build().await?;

// //   let node_url = format!(
// //     "{}{}",
// //     mock_server.uri(),
// //     "/v20.0.0/node-v20.0.0-win-x64.zip"
// //   );

// //   let out = current_dir()?.join("temp.zip");

// //   let res = snm_download_builder::DownloadBuilder::new()
// //     .retries(3)
// //     .timeout(30)
// //     .download(&node_url, &out)
// //     .await?;

// //   println!("test_reqwest_download ---->: {:?} {:?}", res, out.exists());

// //   //   let _url = "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json";
// //   //   let resp = if cfg!(target_os = "windows") {
// //   //     cmd!(
// //   //         "cmd",
// //   //         "/C",
// //   //         "certutil -urlcache -split -f https://raw.githubusercontent.com/nodejs/Release/main/schedule.json temp.json & type temp.json & del temp.json"
// //   //       )
// //   //       .stdout_capture()
// //   //       .stderr_capture()
// //   //       .read()?
// //   //   } else {
// //   //     cmd!(
// //   //       "curl",
// //   //       "-s",
// //   //       "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json"
// //   //     )
// //   //     .stdout_capture()
// //   //     .stderr_capture()
// //   //     .read()?
// //   //   };

// //   //   let response = reqwest::get(url).await?;
// //   //   println!("response---->: {:?}", response);
// //   //   assert!(response.status().is_success());
// //   //   let content = response.text().await?;
// //   //   assert!(content.contains("v0.8")); // 验证内容
// //   //   println!("content---->: {:?}", content);
// //   Ok(())
// // }

// // fn get_debug_dir() -> std::path::PathBuf {
// //   // 获取 e2e 目录 (CARGO_MANIFEST_DIR 指向 e2e/Cargo.toml 所在目录)
// //   let e2e_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

// //   // 向上一级找到项目根目录
// //   let root_dir = e2e_dir.parent().expect("Failed to get root dir");

// //   // 进入 target/debug 目录
// //   root_dir.join("target").join("debug")
// // }

// use std::path::PathBuf;

#[tokio::test]
async fn test_reqwest_download() -> Result<(), Box<dyn std::error::Error>> {
  let mock_server = e2e::http_mocker::HttpMocker::builder()?.build().await?;

  let uri = mock_server.uri();

  //   let download_url = format!("{}{}", uri, "/v20.0.0/node-v20.0.0-win-x64.zip");
  //   let abs_path = std::env::current_dir()?.join("temp.zip");
  //   let res = snm_download_builder::DownloadBuilder::new()

  let cur = std::env::current_dir()?;

  let config = snm_config::SnmConfig {
    node_bin_dir: cur.join(".snm").join("node_bin_demo"),
    download_dir: cur.join(".snm").join("downloads_demo"),
    cache_dir: cur.join(".snm").join("cache_demo"),
    node_modules_dir: cur.join(".snm").join("node_modules_demo"),
    node_dist_url: uri,
    node_github_resource_host: "https://raw.githubusercontent.com".to_string(),
    node_install_strategy: snm_config::InstallStrategy::Auto,
    node_white_list: "".to_string(),
    download_timeout_secs: 30,
    npm_registry: "https://registry.npmmirror.com".to_string(),
    workspace: cur.join("tests").join("fixtures").join("empty"),
    lang: "en".to_string(),
    restricted_list: vec![],
    strict: false,
  };

  let node_downloader = snm_node::downloader::NodeDownloader::new(&config);

  let res = node_downloader.download("20.0.0").await?;

  let node_bin_dir = config.node_bin_dir.join("20.0.0").join("bin").join("node");

  println!("res---->: {:?}", res);
  println!(
    "node_bin_dir---->: {:?} {}",
    node_bin_dir,
    node_bin_dir.exists()
  );

  //   let builder = e2e::CommandBuilder::with_envs(
  //     "test_reqwest_download",
  //     std::env::current_dir()?
  //       .join("tests")
  //       .join("fixtures")
  //       .join("empty"),
  //     vec![
  //       e2e::SnmEnv::NodeDistUrl(uri.clone()),
  //       e2e::SnmEnv::NpmRegistry(uri.clone()),
  //     ],
  //   )?;

  //   let res = builder.exec("snm node install 20.0.0")?;
  //   println!("res---->: {:?}", res);
  //   let res = builder.exec("snm node list --compact")?;
  //   println!("res---->: {:?}", res);
  //   let res = builder.exec("snm node default 20.0.0")?;
  //   println!("res---->: {:?}", res);
  //   let res = builder.exec("snm node list --compact")?;
  //   println!("res---->: {:?}", res);
  //   let res = builder.exec("node -v")?;
  //   println!("res---->: {:?}", res);
  //   builder.assert_snapshots(|name, res| {
  //     insta::assert_snapshot!(name, res);
  //   })?;

  //   let out = current_dir()?.join("temp.zip");

  //   // 获取 download_test 可执行文件路径
  //   let test_exe = get_debug_dir().join("download_test");

  //   let output = if cfg!(target_os = "windows") {
  //     duct::cmd!(
  //       "cmd",
  //       "/C",
  //       format!(
  //         "{} {} {}",
  //         test_exe.display(),
  //         mock_server.uri(),
  //         out.display()
  //       )
  //     )
  //   } else {
  //     duct::cmd!(
  //       "sh",
  //       "-c",
  //       format!(
  //         "{} {} {}",
  //         test_exe.display(),
  //         mock_server.uri(),
  //         out.display()
  //       )
  //     )
  //   }
  //   .stdout_capture()
  //   .stderr_capture()
  //   .run()?;

  //   println!(
  //     "Child process output: {}",
  //     String::from_utf8_lossy(&output.stdout)
  //   );
  //   println!(
  //     "Child process error: {}",
  //     String::from_utf8_lossy(&output.stderr)
  //   );

  //   assert!(out.exists(), "Downloaded file should exist");

  Ok(())
}
