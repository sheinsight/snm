use std::{
  env::{current_dir, Args},
  path::Path,
};

use anyhow::bail;
use snm_config::snm_config::SnmConfig;
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

    let Some(actual_bin_name) = args.first() else {
      bail!("No binary name provided in arguments {:#?}", args);
    };

    trace!(r#"try_from args: {:#?}"#, args);

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(SNM_PREFIX, &cwd)?;

    let node_setup = snm_node::NodeSetup::from(snm_config.clone());

    let bin_dir = node_setup.resolve_node_bin_dir().await?;

    let paths = vec![bin_dir.to_string_lossy().into_owned()];

    if actual_bin_name == "node" {
      Ok(CommandShim::Node(NodeShim::new(args, paths)))
    } else {
      Ok(CommandShim::Pm(PmShim::new(args, paths, snm_config)))
    }
  }
}

#[cfg(test)]
mod tests {
  use lazy_regex::regex;

  #[test]
  fn test_trim() {
    let r = regex!(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$");

    let v = r#"18.19.0
"#;

    let v = v.trim();

    assert_eq!(v, "18.19.0");
    assert!(r.is_match(&v));
  }
}
