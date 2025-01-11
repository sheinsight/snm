use std::env::{self, current_dir};

use snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::exec::exec_cli;

pub async fn node() -> anyhow::Result<()> {
  let cwd = current_dir()?;

  let config = SnmConfig::from(&cwd)?;

  let bin_args: Vec<String> = env::args().collect();

  let node_version_reader = SNode::try_from(&config)?;

  let node_bin_dir = node_version_reader.get_bin().await?;

  exec_cli(vec![node_bin_dir.to_string_lossy().to_string()], bin_args)?;

  Ok(())
}
