use std::env::current_dir;

use clap::Parser;
use snm::{execute_cli::execute_cli, SnmCli};
use snm_config::SnmConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  run().await
}

async fn run() -> anyhow::Result<()> {
  let dir = current_dir()?;

  let snm_config = SnmConfig::from(dir)?;

  let cli = SnmCli::parse();

  execute_cli(cli, snm_config).await
}
