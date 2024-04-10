use colored::*;
use snm_core::{
    config::{snm_config::InstallStrategy, SnmConfig},
    exec_proxy_child_process,
    model::{manager::ManagerTraitDispatcher, snm_error::handle_snm_error, SnmError},
    println_success,
};
use snm_node::demo::NodeDemo;

use std::{env::current_dir, fs::read_to_string, ops::Not, path::PathBuf};

const NODE_VERSION_FILE: &str = ".node-version";

#[tokio::main]
async fn main() {
    env_logger::init();

    let m = ManagerTraitDispatcher::new(Box::new(NodeDemo::new()));

    match m.launch_proxy().await {
        Ok((v, bin_path_buf)) => {
            println_success!("Use Node {}. ", v.green());
            exec_proxy_child_process!(&bin_path_buf);
        }
        Err(error) => {
            handle_snm_error(error);
        }
    }

    // match execute().await {
    //     Ok((v, bin_path_buf)) => {
    //         println_success!("Use Node {}. ", v.green());
    //         exec_proxy_child_process!(&bin_path_buf);
    //     }
    //     Err(error) => {
    //         handle_snm_error(error);
    //     }
    // };
}

// fn get_node_binary_path(node_bin_dir_path_buf: &PathBuf, version: &str) -> PathBuf {
//     node_bin_dir_path_buf.join(version).join("bin").join("node")
// }

// fn get_version_from_file(node_version_path_buf: &PathBuf) -> Result<String, SnmError> {
//     let version_processor = |value: String| value.trim_start_matches(['v', 'V']).trim().to_string();
//     let version = read_to_string(node_version_path_buf).map(version_processor)?;
//     Ok(version)
// }

// async fn ensure_node_binary_exists_or_panic(
//     node_binary_path_buf: &PathBuf,
//     v: &str,
//     snm_config: &SnmConfig,
// ) -> Result<(), SnmError> {
//     if node_binary_path_buf.exists().not() {
//         match snm_config.get_node_install_strategy()? {
//             InstallStrategy::Ask => {
//                 if ask_download(&v)? {
//                     download(&v).await?;
//                 }
//             }
//             InstallStrategy::Auto => {
//                 download(&v).await?;
//             }
//             InstallStrategy::Panic => {
//                 return Err(SnmError::UnsupportedNodeVersion {
//                     version: v.to_string(),
//                 });
//             }
//         }
//     }
//     Ok(())
// }

// async fn execute() -> Result<(String, PathBuf), SnmError> {
//     let snm_config = SnmConfig::new();

//     let node_version_path_buf = current_dir()?.join(NODE_VERSION_FILE);

//     let node_bin_dir_path_buf = snm_config.get_node_bin_dir_path_buf()?;

//     if node_version_path_buf.exists().not() {
//         if snm_config.get_strict() {
//             Err(SnmError::NotFoundNodeVersionFileError {
//                 file_path: node_version_path_buf.display().to_string(),
//             })?;
//         } else {
//             let (dir_vec, default_version) = read_bin_dir()?;
//             if dir_vec.is_empty() {
//                 return Err(SnmError::EmptyNodeList);
//             }

//             let version = default_version.ok_or(SnmError::NotFoundDefaultNodeBinary)?;

//             let node_binary_abs_path = get_node_binary_path(&node_bin_dir_path_buf, &version);

//             return Ok((version, node_binary_abs_path));
//         }
//     }

//     let v = get_version_from_file(&node_version_path_buf)?;

//     let node_binary_path_buf = get_node_binary_path(&node_bin_dir_path_buf, &v);

//     ensure_node_binary_exists_or_panic(&node_binary_path_buf, &v, &snm_config).await?;

//     Ok((v, node_binary_path_buf))
// }
