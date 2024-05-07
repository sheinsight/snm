use snm_core::{
    config::SnmConfig,
    model::{snm_error::handle_snm_error, SnmError},
};

use cli::execute_cli::execute_cli;

#[tokio::main]
async fn main() -> Result<(), SnmError> {
    SnmConfig::new().init()?;

    if let Err(error) = execute_cli().await {
        handle_snm_error(error);
    }

    Ok(())
}
