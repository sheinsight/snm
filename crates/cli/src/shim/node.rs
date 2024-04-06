use snm_core::{
    config::init_config::{init_config, SNM_STRICT},
    exec_child_process,
    model::{snm_error::handle_snm_error, SnmError},
};
use snm_node::node_mg::{use_bin, use_default_node};
use std::{env::current_dir, fs::read_to_string, ops::Not, path::PathBuf, process::Output};

const NODE_VERSION_FILE: &str = ".node-version";

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = init_config();

    let strict = std::env::var(SNM_STRICT).unwrap();

    if let Err(error) = if strict == "true" {
        execute_strict().await
    } else {
        execute().await
    } {
        handle_snm_error(error);
    }
}

async fn execute_command(bin_path_buf: PathBuf) -> Result<Output, SnmError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = exec_child_process!(&bin_path_buf, &args)?;
    Ok(output)
}

async fn execute() -> Result<Output, SnmError> {
    let node_version_path_buf = current_dir()?.join(NODE_VERSION_FILE);
    if node_version_path_buf.exists().not() {
        let (_, bin_path_buf) = use_default_node().await?;
        Ok(execute_command(bin_path_buf).await?)
    } else {
        Ok(execute_strict().await?)
    }
}

async fn execute_strict() -> Result<Output, SnmError> {
    let node_version_path_buf = current_dir()?.join(NODE_VERSION_FILE);
    if node_version_path_buf.exists().not() {
        return Err(SnmError::NotFoundNodeVersionFileError {
            file_path: node_version_path_buf.display().to_string(),
        });
    }
    let v = read_to_string(&node_version_path_buf)
        .map(|value| value.trim_start_matches(['v', 'V']).trim().to_string())?;
    let bin_path_buf = use_bin(&v).await?;
    Ok(execute_command(bin_path_buf).await?)
}
