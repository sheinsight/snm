use snm_core::{
    exec_child_process, model::snm_error::handle_snm_error, utils::health::check_multi_lock_file,
};
use snm_node::node_mg::use_node;
use snm_pm::get_manager_bin_file_path;

pub async fn launch(name: &str) {
    env_logger::init();

    if let Err(e) = snm_core::config::init_config() {
        handle_snm_error(e);
        std::process::exit(1);
    }

    if let Err(e) = check_multi_lock_file() {
        handle_snm_error(e);
        std::process::exit(1);
    }

    match name {
        "yarn" | "npm" | "pnpm" => launch_package_manager_shim(name).await,
        "node" => launch_node_shim(name).await,
        _ => {
            std::process::exit(1);
        }
    };
}

async fn launch_package_manager_shim(name: &str) {
    let bin_file_path = match get_manager_bin_file_path(name).await {
        Ok(bin_file_path) => bin_file_path,
        Err(error) => {
            handle_snm_error(error);
            return;
        }
    };

    let args: Vec<String> = std::env::args().skip(1).collect();

    let output = match exec_child_process!(bin_file_path, &args) {
        Ok(output) => output,
        Err(_) => return,
    };

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(-1));
    }
}

async fn launch_node_shim(_name: &str) {
    let node_binary_abs_path = match use_node().await {
        Ok(tuple) => tuple,
        Err(e) => {
            handle_snm_error(e);
            std::process::exit(1);
        }
    };

    let args: Vec<String> = std::env::args().skip(1).collect();

    let output = match exec_child_process!(node_binary_abs_path, &args) {
        Ok(output) => output,
        Err(_) => return,
    };

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(-1));
    }
}
