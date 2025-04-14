use clap::Parser;
use cli::SnmCli;
use snm_utils::log::init_snm_log;

mod cli;
mod manage_command;
mod node;
mod package_manager;
mod snm_command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_snm_log()?;

  SnmCli::parse().exec().await
}
