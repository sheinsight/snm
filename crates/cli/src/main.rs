use clap::Parser;
use snm_core::{
    config::SnmConfig,
    model::{snm_error::handle_snm_error, SnmError},
};

use cli::{execute_cli::execute_cli, SnmCli};

#[tokio::main]
async fn main() -> Result<(), SnmError> {
    SnmConfig::new().init()?;
    let cli = SnmCli::parse();

    if let Err(error) = execute_cli(cli).await {
        handle_snm_error(error);
    }

    Ok(())
}
