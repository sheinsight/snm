use std::env::{self, current_dir};

use anyhow::bail;
use node_shim::load_node;
use pm_shim::load_pm;
use snm_config::snm_config::SnmConfig;
use snm_utils::{consts::SNM_PREFIX, trace_if};
use tracing::trace;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
mod node_shim;
mod pm_shim;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  if let Some(home) = dirs::home_dir() {
    let file = std::fs::File::create(home.join("snm_shim.log"))?;

    let file_layer = fmt::layer()
      .with_writer(file)
      .with_filter(EnvFilter::from_env("SNM_LOG"));

    // 创建控制台写入器
    let stdout_layer = fmt::layer().with_filter(EnvFilter::from_env("SNM_LOG"));

    tracing_subscriber::registry()
      .with(file_layer)
      .with(stdout_layer)
      .try_init()?;
  }

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
