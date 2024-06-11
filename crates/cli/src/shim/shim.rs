use std::process::{Command, Stdio};

use colored::*;
use snm_core::{
    model::dispatch_manage::DispatchManage, println_success, traits::manage::ManageTrait,
};
use snm_current_dir::current_dir;
use snm_package_json::parse_package_json;

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
    let dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!("NoCurrentDir"),
    };

    let package_json = match parse_package_json(dir) {
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
