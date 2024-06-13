use core::panic;
use std::{
    ops::Not,
    path::PathBuf,
    process::{Command, Stdio},
};

use colored::*;
use snm_config::SnmConfig;
use snm_core::{
    model::dispatch_manage::DispatchManage,
    println_success,
    traits::{manage::ManageTrait, shim::ShimTrait},
};
use snm_current_dir::current_dir;
use snm_package_json::parse_package_json;
use snm_utils::snm_error::SnmError;

pub fn exec_strict(manager: Box<dyn ShimTrait>, snm_config: SnmConfig) -> Result<(), SnmError> {
    let workspace = snm_config.get_workspace()?;
    let package_json = parse_package_json(&workspace);
    let node_version_path_buf = workspace.join(".node-version");
    if node_version_path_buf.exists().not() {
        panic!("Not found .node-version file")
    }
    if package_json.is_none() {
        panic!("Not found package.json file")
    }
    let package_json = package_json.unwrap();
    match package_json.package_manager {
        Some(package_manager) => {
            package_manager.name;

            let x = manager
                .get_runtime_binary_file_path_buf("bin_name", &package_manager.version.unwrap())?;
        }
        None => panic!("Not found package manager"),
    }

    Ok(())
}

pub async fn hello(
    workspace: &PathBuf,
    manager: Box<dyn ShimTrait>,
    snm_config: SnmConfig,
) -> Result<(), String> {
    let package_json = parse_package_json(&workspace);
    // let y = parse_node_version(workspace)

    let node_version_path_buf = workspace.join(".node-version");

    if snm_config.get_strict() {
        // if package manager

        if let Some(pj) = package_json {
            // pj.check_package_manager()?;
            let package_manager = match pj.package_manager {
                Some(package_manager) => package_manager,
                None => {
                    return Err("Not found package manager".to_string());
                }
            };

            let name = match package_manager.name {
                Some(name) => name,
                None => {
                    return Err("Not found package manager name".to_string());
                }
            };

            let version = match package_manager.version {
                Some(version) => version,
                None => {
                    return Err("Not found package manager version".to_string());
                }
            };
        } else {
            panic!("Not found package.json file")
        }
        if node_version_path_buf.exists().not() {
            panic!("Not found .node-version file")
        }
        // manager.check_satisfy_strict_mode("npm");
        let v = manager.get_strict_shim_version();

        let anchor_file_path_buf = match manager.get_anchor_file_path_buf(&v) {
            Ok(anchor_file_path_buf) => anchor_file_path_buf,
            Err(_) => panic!("set_default get_anchor_file_path_buf error"),
        };

        if anchor_file_path_buf.exists().not() {
            if manager.download_condition(&v) {
                // exec download
            } else {
                panic!("Not found anchor file")
            }
        }
    } else {
        // if none then found default target then panic
    }

    Ok(())
}

pub async fn launch_shim(
    manager: Box<dyn ManageTrait>,
    bin_name: &str,
    strict: bool,
) -> Result<(), SnmError> {
    let dispatcher = DispatchManage::new(manager);
    let (v, bin_path_buf) = dispatcher.proxy_process(bin_name, strict).await?;
    println_success!(
        "Use {:<8}. {}",
        v.bright_green(),
        format!("by {}", bin_path_buf.display()).bright_black()
    );
    let args: Vec<String> = std::env::args().skip(1).collect();
    let _ = Command::new(&bin_path_buf)
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .and_then(|process| process.wait_with_output());

    Ok(())
}

pub fn _check(actual_package_manager: &str) {
    let dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!("NoCurrentDir"),
    };

    let package_json = match parse_package_json(&dir) {
        Some(pkg) => pkg,
        None => panic!("NoPackageManager"),
    };

    println!("dir: {:?}", package_json);
    let package_manager = match package_json.package_manager {
        Some(pm) => pm,
        None => panic!("NoPackageManager"),
    };

    let name = match package_manager.name {
        Some(n) => n,
        None => panic!("NoPackageManager"),
    };

    if name != actual_package_manager {
        let msg = format!(
            "NotMatchPackageManager {} {}",
            name,
            actual_package_manager.to_string()
        );
        panic!("{msg}");
    }
}
