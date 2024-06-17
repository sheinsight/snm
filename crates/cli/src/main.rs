use clap::Parser;
use snm_config::parse_snm_config;
use snm_core::color_backtrace;

use cli::{execute_cli::execute_cli, SnmCli};
use snm_current_dir::current_dir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_backtrace::install();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let cli = SnmCli::parse();

    execute_cli(cli, snm_config).await;

    Ok(())
}
