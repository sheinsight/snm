use snm_utils::{exec::exec_cli, snm_error::SnmError};

use crate::get_node_bin_dir::get_node_bin_dir;

pub async fn node(bin_name: &str) -> Result<(), SnmError> {
    color_backtrace::install();

    tracing_subscriber::fmt::init();

    let bin_args: Vec<String> = std::env::args().skip(1).collect();

    let node_dir = get_node_bin_dir().await?;

    exec_cli(vec![node_dir], bin_name, &bin_args)?;

    Ok(())
}
