use std::env::{self, current_dir};

use anyhow::bail;
use node_shim::load_node;
use pm_shim::load_pm;
use snm_config::snm_config::SnmConfig;
use snm_utils::{consts::SNM_PREFIX, log::init_snm_log, trace_if};
use tracing::trace;
mod node_shim;
mod pm_shim;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_snm_log()?;

  trace!("Start snm_shim");

  let cwd = current_dir()?;

  trace_if!(|| {
    trace!("Current dir: {:?}", cwd);
  });

  let args: Vec<String> = env::args().collect();

  trace_if!(|| {
    trace!("Command args: {:?}", &args);
  });

  let snm_config = SnmConfig::from(SNM_PREFIX, &cwd)?;

  if let [actual_bin_name, ..] = args.as_slice() {
    trace_if!(|| {
      trace!("Actual bin name: {:?}", actual_bin_name);
    });

    if actual_bin_name == "node" {
      load_node(&snm_config, &args).await?;
    } else {
      load_pm(&snm_config, &args).await?;
    }
  } else {
    bail!(
      r#"No binary name provided in arguments
args: {:?}"#,
      args
    );
  }

  Ok(())
}
