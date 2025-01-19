use std::env::current_dir;

use clap::Parser;
use cli::SnmCli;
// use snm::SnmCli;
// use snm::{execute_cli, SnmCli};
use snm_config::SnmConfig;
use tracing::trace;

// pub mod execute_cli;
mod cli;
mod execute_cli;
mod fig;
mod manage_command;
mod snm_command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_env_filter(std::env::var("SNM_LOG").unwrap_or_else(|_| "snm=info".into()))
    .init();

  trace!("Start snm");

  let dir = current_dir()?;

  trace!("Get current dir: {:?}", dir);

  let snm_config = SnmConfig::from(dir)?;

  trace!(
    r#"Get snm config:
{}"#,
    snm_config
  );

  trace!("Start parse cli");

  let cli = SnmCli::parse();

  execute_cli::execute_cli(cli, snm_config).await
}
