use std::env::{self, current_dir};

use command_shim::CommandShim;
use snm_utils::log::init_snm_log;
use tracing::trace;
mod command_shim;

mod node_shim;
mod pm_shim;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_snm_log()?;

  let cwd = current_dir()?;

  trace!(cwd = ?cwd, "Current working directory");

  let command = CommandShim::from_args(env::args()).await?;

  command.proxy(&cwd).await?;

  Ok(())
}
