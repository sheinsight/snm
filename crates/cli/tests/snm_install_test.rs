use std::{
    env::{self, current_dir, set_current_dir},
    path::PathBuf,
};

fn auto_install_set_current_dir(dir: &str) -> PathBuf {
    env::set_var("SNM_NODE_INSTALL_STRATEGY", "auto");
    env::set_var("SNM_PACKAGE_MANAGER_INSTALL_STRATEGY", "auto");
    let c_dir = current_dir().expect("get current dir error");
    let test_dir = c_dir.join("tests").join(dir);
    set_current_dir(&test_dir).expect("set current dir error");
    test_dir
}

use clap::Parser;
use cli::{execute_cli, SnmCli};
use snm_core::model::SnmError;
use std::fs;

#[tokio::test]
async fn test_no_package_json() {
    let _test_dir = auto_install_set_current_dir("no_package_json");

    let cli = SnmCli::parse_from(["snm", "install"]);
    let res = execute_cli::execute_cli(cli).await;
    assert_eq!(res.is_err(), true);

    if let Err(n) = res {
        match n {
            SnmError::NotFoundDefaultPackageManager { name } => {
                assert_eq!(name, "pnpm".to_string());
            }
            _ => panic!("expect SnmError::NotFoundDefaultPackageManager"),
        }
    }
}

#[tokio::test]
async fn test_no_package_manager_property() {
    let test_dir = auto_install_set_current_dir("no_package_manager_property");

    let package_json = test_dir
        .join("package.json")
        .as_path()
        .display()
        .to_string();

    let cli = SnmCli::parse_from(["snm", "install"]);
    let res = execute_cli::execute_cli(cli).await;
    assert_eq!(res.is_err(), true);
    if let Err(n) = res {
        match n {
            SnmError::NotFoundPackageManagerProperty { file_path } => {
                assert_eq!(file_path, package_json);
            }
            _ => panic!("expect SnmError::NotFoundPackageManagerProperty"),
        }
    }
}

#[tokio::test]
async fn test_pnpm_install() {
    let test_dir = auto_install_set_current_dir("snm_install_pnpm");

    let command_str = "snm install";

    let cli = SnmCli::parse_from(command_str.split(" "));
    let lock = &test_dir.join("pnpm-lock.yaml");
    let _ = fs::remove_file(lock);

    assert!(!lock.exists());
    let res = execute_cli::execute_cli(cli).await;
    assert!(res.is_ok());
    assert!(lock.exists());
}

#[tokio::test]
async fn test_npm_install() {
    let test_dir = auto_install_set_current_dir("snm_install_npm");

    let cli = SnmCli::parse_from(["snm", "install"]);
    let lock = &test_dir.join("package-lock.json");
    let _ = fs::remove_file(lock);

    assert!(!lock.exists());
    let res = execute_cli::execute_cli(cli).await;
    assert!(res.is_ok());
    assert!(lock.exists());
}
