use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::exec::exec_cli;

pub struct NodeShim {
  pub args: Vec<String>,
  pub snm_config: SnmConfig,
}

impl NodeShim {
  pub async fn proxy(&self) -> anyhow::Result<()> {
    let snode = SNode::try_from(&self.snm_config)?;

    let node_bin_dir = snode.get_bin_dir().await?;

    exec_cli(
      &self.args,
      &vec![node_bin_dir.to_string_lossy().to_string()],
      true,
    )?;

    Ok(())
  }
}
