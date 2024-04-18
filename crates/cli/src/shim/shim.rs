use std::{
    env::current_dir,
    process::{Command, Stdio},
};

use colored::*;
use snm_core::{
    model::{
        dispatch_manage::DispatchManage, snm_error::handle_snm_error, trait_manage::ManageTrait,
        PackageJson, SnmError,
    },
    println_success,
};

pub async fn launch_shim(manager: Box<dyn ManageTrait>, bin_name: &str) {
    let dispatcher = DispatchManage::new(manager);
    match dispatcher.proxy_process(bin_name).await {
        Ok((v, bin_path_buf)) => {
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
        Err(error) => {
            handle_snm_error(error);
        }
    }
}

pub fn check(actual_package_manager: &str) -> Result<(), SnmError> {
    let dir = current_dir().expect("get current dir failed");
    let package_json_path_buf = dir.join("package.json");
    if package_json_path_buf.exists() {
        let package_json = PackageJson::from_file_path(&package_json_path_buf)?;
        let package_manager = package_json.parse_package_manager()?;
        if package_manager.name != actual_package_manager {
            return Err(SnmError::NotMatchPackageManager {
                expect: package_manager.name,
                actual: actual_package_manager.to_string(),
            });
        }
        return Ok(());
    }

    Ok(())
}
