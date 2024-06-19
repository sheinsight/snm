use clap::Parser;
use cli::{execute_cli::execute_cli, SnmCli};
use snm_config::parse_snm_config;
use snm_core::color_backtrace;
use snm_current_dir::current_dir;
use snm_utils::snm_error::{friendly_error_message, SnmError};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        friendly_error_message(error);
    }
}

async fn run() -> Result<(), SnmError> {
    color_backtrace::install();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let cli = SnmCli::parse();

    execute_cli(cli, snm_config).await?;

    Ok(())
}
