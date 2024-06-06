use std::env;

use clap::Parser;
use snm_core::{
    color_backtrace,
    config::{snm_config::InstallStrategy, SnmConfig},
    snm_content::{SnmContent, SnmContentHandler},
};

use cli::{execute_cli::execute_cli, SnmCli};

#[tokio::main]
async fn main() {
    color_backtrace::install();
    SnmConfig::new().init();

    let snm_content_handler: SnmContentHandler = SnmContentHandler::new(SnmContent {
        strict: SnmConfig::new().get_strict(),
        base_dir_path_buf: SnmConfig::new().get_base_dir_path_buf(),
        download_dir_path_buf: SnmConfig::new().get_download_dir_path_buf(),
        node_modules_dir_path_buf: SnmConfig::new().get_node_modules_dir_path_buf(),
        npm_registry: SnmConfig::new().get_npm_registry_host(),
        package_manager_install_strategy: InstallStrategy::Auto,
    });

    println!("registry: {}", registry);

    let cli = SnmCli::parse();

    execute_cli(cli, snm_content_handler).await;
}
