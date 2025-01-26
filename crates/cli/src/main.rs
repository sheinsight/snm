use std::env::current_dir;

use clap::Parser;
use cli::SnmCli;
use snm_config::snm_config::SnmConfig;
use snm_utils::consts::SNM_PREFIX;
use tracing::trace;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

mod cli;
mod execute_cli;
mod fig;
mod manage_command;
mod snm_command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  if let Some(home) = dirs::home_dir() {
    let file = std::fs::File::create(home.join("snm.log"))?;

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

  trace!("Start snm");

  let dir = current_dir()?;

  trace!("Get current dir: {:?}", dir);

  let snm_config = SnmConfig::from(SNM_PREFIX, dir)?;

  trace!(
    r#"Get snm config:
{}"#,
    snm_config
  );

  trace!("Start parse cli");

  let cli = SnmCli::parse();

  execute_cli::execute_cli(cli, snm_config).await
}
