use colored::*;
use snm_core::{
    config::{snm_config::InstallStrategy, SnmConfig},
    exec_proxy_child_process,
    model::{snm_error::handle_snm_error, SnmError},
    println_success,
};
use snm_node::node_mg::{ask_download, download};
use std::{env::current_dir, fs::read_to_string, ops::Not, process::Output};

const NODE_VERSION_FILE: &str = ".node-version";

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(error) = execute().await {
        handle_snm_error(error);
    }
}

async fn execute() -> Result<Output, SnmError> {
    let snm_config = SnmConfig::new();

    let node_version_path_buf = current_dir()?.join(NODE_VERSION_FILE);

    if node_version_path_buf.exists().not() {
        if snm_config.get_strict() {
            Err(SnmError::NotFoundNodeVersionFileError {
                file_path: node_version_path_buf.display().to_string(),
            })?;
        } else {
            todo!("use default node")
        }
    }

    let version_processor = |value: String| value.trim_start_matches(['v', 'V']).trim().to_string();
    let v = read_to_string(&node_version_path_buf).map(version_processor)?;

    let node_binary_path_buf = snm_config
        .get_node_bin_dir_path_buf()?
        .join(&v)
        .join("bin")
        .join("node");

    if node_binary_path_buf.exists().not() {
        match snm_config.get_node_install_strategy()? {
            InstallStrategy::Ask => {
                if ask_download(&v)? {
                    download(&v).await?;
                }
            }
            InstallStrategy::Install => {
                download(&v).await?;
            }
            InstallStrategy::Panic => {
                return Err(SnmError::UnSupportNodeVersion { version: v });
            }
        }
    }

    println_success!(std::io::stdout(), "Use Node {}. ", v.green());

    Ok(exec_proxy_child_process!(&node_binary_path_buf)?)
}
