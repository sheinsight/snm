use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::exec::exec_cli;

pub async fn load_node(config: &SnmConfig, args: &Vec<String>) -> anyhow::Result<()> {
  let node_version_reader = SNode::try_from(&config)?;

  let node_bin_dir = node_version_reader.ensure_node_and_return_dir().await?;

  exec_cli(
    args,
    &vec![node_bin_dir.to_string_lossy().to_string()],
    true,
  )?;

  Ok(())
}
