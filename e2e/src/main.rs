use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mock_server_url = env::args().nth(1).expect("Missing mock server URL");
  let output_path = env::args().nth(2).expect("Missing output path");

  let node_url = format!("{}{}", mock_server_url, "/v20.0.0/node-v20.0.0-win-x64.zip");

  let res = snm_download_builder::DownloadBuilder::new()
    .retries(3)
    .timeout(30)
    .download(&node_url, &output_path)
    .await?;

  println!("download_test result: {:?}", res);
  println!("file exists: {:?}", PathBuf::from(&output_path).exists());

  Ok(())
}
