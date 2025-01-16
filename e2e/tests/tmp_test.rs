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

#[tokio::test]
async fn test_install_node() -> anyhow::Result<()> {
  let mock_server = e2e::get_global_mock_server().await;

  let uri = mock_server.uri();

  let builder = e2e::CommandBuilder::with_envs(
    "test_install_node",
    std::env::current_dir()?
      .join("tests")
      .join("fixtures")
      .join("empty"),
    vec![
      e2e::SnmEnv::NodeDistUrl(uri.clone()),
      e2e::SnmEnv::NpmRegistry(uri.clone()),
    ],
  )?;

  builder.exec("snm setup")?;

  let res = builder.exec("snm node install 20.0.0")?;

  println!("res---->: {:?}", res);

  Ok(())
}
