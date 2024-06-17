use std::{
    env::{self, current_dir, set_current_dir},
    path::PathBuf,
};

fn set_strict() {
    env::set_var("SNM_STRICT", "true");
}

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
use snm_core::{
    config::{snm_config::InstallStrategy, SnmConfig},
    snm_content::{SnmContent, SnmContentHandler},
};
use std::fs;

#[tokio::test]
#[should_panic(expected = "NotFoundPackageJsonFile")]
async fn test_no_package_json() {
    // auto_install_set_current_dir("no_package_json");

    let tmp = current_dir().expect("get current dir error").join("tmps");
    let download_dir = tmp.join("download");
    let node_modules_dir = tmp.join("node_modules");

    let snm_content_handler: SnmContentHandler = SnmContentHandler::new(SnmContent {
        strict: true,
        base_dir_path_buf: tmp,
        download_dir_path_buf: download_dir,
        node_modules_dir_path_buf: node_modules_dir,
        npm_registry: "https://registry.npmjs.org".to_string(),
        package_manager_install_strategy: InstallStrategy::Auto,
    });

    let cli = SnmCli::parse_from(["snm", "i"]);
    execute_cli::execute_cli(cli, snm_content_handler).await;
}

// #[tokio::test]
// #[should_panic(expected = "NoPackageManagerProperty")]
// async fn test_no_package_manager_property() {
//     auto_install_set_current_dir("no_package_manager_property");

//     let cli = SnmCli::parse_from(["snm", "i"]);
//     execute_cli::execute_cli(cli).await;
// }

// #[tokio::test]
// async fn test_pnpm_install() {
//     set_strict();
//     let test_dir = auto_install_set_current_dir("snm_install_pnpm");

//     let cli = SnmCli::parse_from(["snm", "i"]);
//     let lock = &test_dir.join("pnpm-lock.yaml");
//     let _ = fs::remove_file(lock);

//     assert!(!lock.exists());
//     execute_cli::execute_cli(cli).await;
//     assert!(lock.exists());
// }

// #[tokio::test]
// async fn test_npm_install() {
//     set_strict();
//     let test_dir = auto_install_set_current_dir("snm_install_npm");

//     let cli = SnmCli::parse_from(["snm", "i"]);
//     let lock = &test_dir.join("package-lock.json");
//     let _ = fs::remove_file(lock);

//     assert!(!lock.exists());
//     execute_cli::execute_cli(cli).await;
//     assert!(lock.exists());
// }

// #[tokio::test]
// #[should_panic(expected = "UnsupportedPackageManager yarn@1.22.21")]
// async fn test_yarn_install() {
//     set_strict();
//     let test_dir = auto_install_set_current_dir("snm_install_yarn");

//     let cli = SnmCli::parse_from(["snm", "i"]);
//     let lock = &test_dir.join("package-lock.json");
//     let _ = fs::remove_file(lock);

//     assert!(!lock.exists());
//     execute_cli::execute_cli(cli).await;
//     assert!(lock.exists());
// }
