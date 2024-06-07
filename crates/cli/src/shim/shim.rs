use std::process::{Command, Stdio};

use colored::*;
use snm_core::{
    model::{dispatch_manage::DispatchManage, PackageJson},
    println_success,
    traits::manage::ManageTrait,
    utils::get_current_dir::get_current_dir,
};

pub async fn launch_shim(manager: Box<dyn ManageTrait>, bin_name: &str, strict: bool) {
    let dispatcher = DispatchManage::new(manager);
    let (v, bin_path_buf) = dispatcher.proxy_process(bin_name, strict).await;
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
}

pub fn _check(actual_package_manager: &str) {
    let dir = get_current_dir();
    let package_json_path_buf = dir.join("package.json");
    if package_json_path_buf.exists() {
        let package_json = PackageJson::from_file_path(&package_json_path_buf);
        let package_manager = package_json.parse_package_manager();
        if package_manager.name != actual_package_manager {
            let msg = format!(
                "NotMatchPackageManager {} {}",
                package_manager.name,
                actual_package_manager.to_string()
            );
            panic!("{msg}");
        }
    }
}
