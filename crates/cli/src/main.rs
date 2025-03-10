use std::env::current_dir;

use clap::Parser;
use cli::SnmCli;
use snm_config::snm_config::SnmConfig;
use snm_utils::{consts::SNM_PREFIX, log::init_snm_log};
use tracing::trace;

mod cli;
mod manage_command;
mod snm_command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_snm_log()?;

  // xx().await

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

  cli.exec(snm_config).await
}

async fn xx() -> anyhow::Result<()> {
  let snm_config = SnmConfig::from(SNM_PREFIX, current_dir()?).unwrap();

  let args = snm_pm::ops::ops::InstallArgs {
    package_spec: vec![],
    frozen: true,
    save_prod: false,
    save_peer: false,
    save_dev: false,
    save_optional: false,
    save_exact: false,
  };

  SnmCli::from(snm_command::SnmCommands::Install(args))
    .exec(snm_config.clone())
    .await?;

  let args = snm_pm::ops::ops::RunArgs {
    command: "build".to_string(),
    passthrough_args: vec![],
  };

  SnmCli::from(snm_command::SnmCommands::Run(args))
    .exec(snm_config.clone())
    .await?;

  Ok(())
}
