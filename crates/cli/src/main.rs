use std::env::current_dir;

use clap::Parser;
use snm::{execute_cli::execute_cli, SnmCli};
use snm_config::parse_snm_config;
use snm_utils::{
    color_backtrace,
    snm_error::{friendly_error_message, SnmError},
};

#[tokio::main]
async fn main() {
    color_backtrace::install();

    if let Err(error) = run().await {
        friendly_error_message(error);
    }
}

async fn run() -> Result<(), SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let cli = SnmCli::parse();

    execute_cli(cli, snm_config).await?;

    Ok(())
}
