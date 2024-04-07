use colored::*;
use snm_core::{
    config::SnmConfig,
    exec_proxy_child_process,
    model::{snm_error::handle_snm_error, SnmError},
    println_success,
};
use snm_node::node_mg::{use_bin, use_default_node};
use std::{env::current_dir, fs::read_to_string, ops::Not, process::Output};

const NODE_VERSION_FILE: &str = ".node-version";

#[tokio::main]
async fn main() {
    env_logger::init();

    let snm_config = SnmConfig::new();

    if let Err(error) = if snm_config.get_strict() {
        execute_strict().await
    } else {
        execute().await
    } {
        handle_snm_error(error);
    }
}

async fn execute() -> Result<Output, SnmError> {
    let node_version_path_buf = current_dir()?.join(NODE_VERSION_FILE);
    if node_version_path_buf.exists().not() {
        let mut stdout = std::io::stdout();
        let (v, bin_path_buf) = use_default_node().await?;
        println_success!(
            stdout,
            "Use Node {} . {}",
            format!("{}", v.green()),
            "by default".bright_black()
        );
        Ok(exec_proxy_child_process!(&bin_path_buf)?)
    } else {
        Ok(execute_strict().await?)
    }
}

async fn execute_strict() -> Result<Output, SnmError> {
    let mut stdout = std::io::stdout();
    let node_version_path_buf = current_dir()?.join(NODE_VERSION_FILE);

    if node_version_path_buf.exists().not() {
        return Err(SnmError::NotFoundNodeVersionFileError {
            file_path: node_version_path_buf.display().to_string(),
        });
    }
    let v = read_to_string(&node_version_path_buf)
        .map(|value| value.trim_start_matches(['v', 'V']).trim().to_string())?;
    let (v, bin_path_buf) = use_bin(&v).await?;
    println_success!(stdout, "Use Node {} .", format!("{}", v.green()));
    Ok(exec_proxy_child_process!(&bin_path_buf)?)
}
