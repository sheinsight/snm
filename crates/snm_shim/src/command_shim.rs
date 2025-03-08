use std::{env::Args, path::Path};

use anyhow::bail;
use snm_config::snm_config::SnmConfig;
use snm_utils::consts::SNM_PREFIX;
use tracing::trace;

use crate::{node_shim::NodeShim, pm_shim::PmShim};

pub enum CommandShim {
  Node(NodeShim),
  Pm(PmShim),
}

impl TryFrom<Args> for CommandShim {
  type Error = anyhow::Error;

  fn try_from(args: Args) -> Result<Self, Self::Error> {
    let args = args.collect::<Vec<String>>();

    trace!(r#"try_from args: {:#?}"#, args);

    if let Some(actual_bin_name) = args.first() {
      if actual_bin_name == "node" {
        Ok(CommandShim::Node(NodeShim { args }))
      } else {
        Ok(CommandShim::Pm(PmShim { args }))
      }
    } else {
      bail!("No binary name provided in arguments {:#?}", args);
    }
  }
}

impl CommandShim {
  pub async fn proxy<T: AsRef<Path>>(&self, cwd: &T) -> anyhow::Result<()> {
    let snm_config = SnmConfig::from(SNM_PREFIX, cwd)?;
    trace!(r#"{:#?}"#, snm_config);
    match self {
      CommandShim::Node(node_shim) => {
        // load_node(&snm_config, &node_shim.args).await?
        node_shim.proxy(cwd).await?
      }
      CommandShim::Pm(pm_shim) => {
        pm_shim.proxy(cwd).await?
        // load_pm(&snm_config, &pm_shim.args).await?
      }
    }
    Ok(())
  }
}
