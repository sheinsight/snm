use std::{
  env::{current_dir, set_var},
  process::{Command, Stdio},
};

#[tokio::test]
async fn test_auto_install_node_with_node_version() -> anyhow::Result<()> {
  let cwd = current_dir()?;

  let tmp_dir = cwd
    .join("tests")
    .join("fixtures")
    .join("auto_install_node_with_node_version")
    .join(".tmp");

  let node_bin_dir = tmp_dir.join(".snm").join("node_bin");
  let download_dir = tmp_dir.join(".snm").join("downloads");
  let node_modules_dir = tmp_dir.join(".snm").join("node_modules");
  let cache_dir = tmp_dir.join(".snm").join("cache");

  set_var("SNM_NODE_BIN_DIR", node_bin_dir.to_str().unwrap());
  set_var("SNM_DOWNLOAD_DIR", download_dir.to_str().unwrap());
  set_var("SNM_NODE_MODULES_DIR", node_modules_dir.to_str().unwrap());
  set_var("SNM_CACHE_DIR", cache_dir.to_str().unwrap());

  let res = Command::new("node")
    .args(["-v"])
    .current_dir(cwd)
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .status()?;

  println!("{:?}", res);

  Ok(())
}
