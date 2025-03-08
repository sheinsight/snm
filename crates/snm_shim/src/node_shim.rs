use std::path::Path;

use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::{consts::SNM_PREFIX, exec::exec_cli};
use tracing::trace;

// pub async fn load_node(config: &SnmConfig, args: &Vec<String>) -> anyhow::Result<()> {
//   let snode = SNode::try_from(&config)?;

//   let node_bin_dir = snode.get_bin_dir().await?;

//   exec_cli(
//     args,
//     &vec![node_bin_dir.to_string_lossy().to_string()],
//     true,
//   )?;

//   Ok(())
// }

pub struct NodeShim {
  pub args: Vec<String>,
}

impl NodeShim {
  pub async fn proxy<T: AsRef<Path>>(&self, cwd: &T) -> anyhow::Result<()> {
    let snm_config = SnmConfig::from(SNM_PREFIX, cwd)?;
    trace!(r#"{:#?}"#, snm_config);
    let snode = SNode::try_from(&snm_config)?;

    let node_bin_dir = snode.get_bin_dir().await?;

    exec_cli(
      &self.args,
      &vec![node_bin_dir.to_string_lossy().to_string()],
      true,
    )?;

    Ok(())
  }
}
