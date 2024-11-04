use std::env::current_dir;

use clap::Parser;
// use node_manager::NodeManager;
use snm::{execute_cli::execute_cli, SnmCli};
use snm_config::{parse_snm_config, SnmConfig};
use snm_utils::snm_error::{friendly_error_message, SnmError};

pub mod node_manager;

#[tokio::main]
async fn main() {
    // color_backtrace::install();

    if let Err(error) = run().await {
        friendly_error_message(error);
    }
}

async fn run() -> Result<(), SnmError> {
    let dir = current_dir()?;

    // let snm_config = SnmConfig::from(dir)?;

    let snm_config = parse_snm_config(&dir)?;

    let cli = SnmCli::parse();

    execute_cli(cli, snm_config).await?;

    Ok(())
}
