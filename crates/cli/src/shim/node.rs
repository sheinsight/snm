use snm_core::{
    config::init_config,
    exec_child_process,
    model::{snm_error::handle_snm_error, SnmError},
};
use snm_node::node_mg::use_bin;
use std::{env::current_dir, fs::read_to_string, ops::Not, process::Output};

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(error) = execute().await {
        handle_snm_error(error);
    }
}

async fn execute() -> Result<Output, SnmError> {
    init_config()?;
    let node_version_path_buf = current_dir()?.join(".node-version");
    if node_version_path_buf.exists().not() {
        return Err(SnmError::NotFoundNodeVersionFileError {
            file_path: node_version_path_buf.display().to_string(),
        });
    }
    let v = read_to_string(&node_version_path_buf)
        .map(|value| value.trim_start_matches(['v', 'V']).trim().to_string())?;

    let bin_path_buf = use_bin(&v).await?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = exec_child_process!(&bin_path_buf, &args)?;
    Ok(output)
}
