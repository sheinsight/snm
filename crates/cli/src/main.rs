use std::env::current_dir;

use clap::Parser;
use cli::SnmCli;
use snm_config::snm_config::SnmConfig;
use snm_utils::{consts::SNM_PREFIX, log::init_snm_log};
use tracing::trace;

mod cli;
mod execute_cli;
mod manage_command;
mod snm_command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_snm_log()?;

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
