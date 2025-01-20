use std::{
  env::{self, current_dir, current_exe},
  ffi::OsStr,
};

use anyhow::Context;
use node_shim::load_node;
use pm_shim::load_pm;
use snm_config::SnmConfig;
use snm_utils::trace_if;
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

  let snm_config = SnmConfig::from(&cwd)?;

  trace_if!(|| {
    trace!(
      r#"Snm config: 
{}"#,
      snm_config
    );
  });

  let actual_bin_name = args.get(0).context("bin name not found")?.clone();

  trace_if!(|| {
    trace!("Actual bin name: {:?}", actual_bin_name);
  });

  // let exe_path = current_exe()?;

  // trace_if!(|| {
  //   trace!("Exe path: {:?}", exe_path);
  // });

  // let exe_name = exe_path
  //   .file_name()
  //   .ok_or(anyhow::anyhow!("file name not found"))?;

  // trace_if!(|| {
  //   trace!("Exe name: {:?}", exe_name);
  // });

  if actual_bin_name == "node" {
    load_node(&snm_config, args).await?;
  } else {
    load_pm(&snm_config, &actual_bin_name, args).await?;
  }

  Ok(())
}
