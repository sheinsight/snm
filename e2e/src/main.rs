// use std::env;
// use std::path::PathBuf;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//   let mock_server_url = env::args().nth(1).expect("Missing mock server URL");
//   let output_path = env::args().nth(2).expect("Missing output path");

//   let node_url = format!("{}{}", mock_server_url, "/v20.0.0/node-v20.0.0-win-x64.zip");

//   let res = snm_download_builder::DownloadBuilder::new()
//     .retries(3)
//     .timeout(30)
//     .download(&node_url, &output_path)
//     .await?;

//   println!("download_test result: {:?}", res);
//   println!("file exists: {:?}", PathBuf::from(&output_path).exists());

//   Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mock_server = e2e::http_mocker::HttpMocker::builder()?.build().await?;

  let uri = mock_server.uri();

  let cur = std::env::current_dir()?;

  let config = snm_config::SnmConfig {
    node_bin_dir: cur.join(".snm").join("node_bin"),
    download_dir: cur.join(".snm").join("downloads"),
    cache_dir: cur.join(".snm").join("cache"),
    node_modules_dir: cur.join(".snm").join("node_modules"),
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

  #[cfg(windows)]
  let node_bin_dir = config
    .node_bin_dir
    .join("20.0.0")
    .join("bin")
    .join("node.exe");
  #[cfg(not(windows))]
  let node_bin_dir = config.node_bin_dir.join("20.0.0").join("bin").join("node");

  println!("res---->: {:?}", res);
  println!(
    "node_bin_dir---->: {:?} {}",
    node_bin_dir,
    node_bin_dir.exists()
  );

  Ok(())
}
