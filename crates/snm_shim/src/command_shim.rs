use std::{
  env::{current_dir, Args},
  path::Path,
};

use anyhow::bail;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::consts::SNM_PREFIX;
use tracing::trace;

use crate::{node_shim::NodeShim, pm_shim::PmShim};

pub enum CommandShim {
  Node(NodeShim),
  Pm(PmShim),
}

impl CommandShim {
  pub async fn proxy<T: AsRef<Path>>(&self, cwd: &T) -> anyhow::Result<()> {
    let snm_config = SnmConfig::from(SNM_PREFIX, cwd)?;
    trace!(r#"{:#?}"#, snm_config);
    match self {
      CommandShim::Node(node_shim) => node_shim.proxy().await?,
      CommandShim::Pm(pm_shim) => pm_shim.proxy().await?,
    }
    Ok(())
  }

  pub async fn from_args(args: Args) -> anyhow::Result<Self> {
    let args = args.collect::<Vec<String>>();

    trace!(r#"try_from args: {:#?}"#, args);

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(SNM_PREFIX, &cwd)?;

    let snode = SNode::try_from(snm_config.clone())?;

    let bin_dir = snode.get_bin_dir().await?;

    let paths = vec![bin_dir.to_string_lossy().into_owned()];

    if let Some(actual_bin_name) = args.first() {
      if actual_bin_name == "node" {
        Ok(CommandShim::Node(NodeShim::new(args, paths)))
      } else {
        Ok(CommandShim::Pm(PmShim::new(args, paths, snm_config)))
      }
    } else {
      bail!("No binary name provided in arguments {:#?}", args);
    }
  }
}
