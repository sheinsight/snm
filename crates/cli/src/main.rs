use clap::Parser;
use snm::{execute_cli::execute_cli, SnmCli};
use snm_config::SnmConfig;
use std::{env::current_dir, process::ExitCode};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> anyhow::Result<()> {
    let dir = current_dir()?;

    let snm_config = SnmConfig::from(dir)?;

    let cli = SnmCli::parse();

    execute_cli(cli, snm_config).await
}
