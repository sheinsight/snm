use clap::Parser;
use snm_core::{color_backtrace, config::SnmConfig};

use cli::{execute_cli::execute_cli, SnmCli};

#[tokio::main]
async fn main() {
    color_backtrace::install();
    SnmConfig::new().init();

    let cli = SnmCli::parse();
    execute_cli(cli).await;
}
