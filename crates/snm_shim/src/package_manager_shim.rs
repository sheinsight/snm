use std::{
    env::{self, current_dir},
    fs,
};

use snm_atom::{atom::AtomTrait as _, package_manager_atom::PackageManagerAtom};
use snm_config::parse_snm_config;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::{exec::exec_cli, snm_error::SnmError};

use crate::get_node_bin_dir::get_node_bin_dir;

pub async fn package_manager(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    color_backtrace::install();

    tracing_subscriber::fmt::init();

    let node_dir = get_node_bin_dir().await?;

    let args_all: Vec<String> = env::args().collect();

    let command = &args_all[1];

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_package_manage = PackageManagerAtom::new(prefix, snm_config.clone());

    let restricted_list = vec!["install", "i", "run"];

    let bin_dirs = if let Some(package_manager) = snm_config.get_runtime_package_manager() {
        tracing::trace!(
            "There is a package manager in the entry process that is currently in use."
        );
        if package_manager.name == prefix {
            let version = package_manager.version;

            let download_url = snm_package_manage.get_download_url(&version);

            let downloaded_file_path_buf =
                snm_package_manage.get_downloaded_file_path_buf(&version)?;

            DownloadBuilder::new()
                .retries(3)
                .timeout(
                    snm_package_manage
                        .get_snm_config()
                        .get_download_timeout_secs(),
                )
                .write_strategy(WriteStrategy::WriteAfterDelete)
                .download(&download_url, &downloaded_file_path_buf)
                .await?;

            let runtime_dir_path_buf = snm_package_manage.get_runtime_dir_path_buf(&version)?;

            snm_package_manage
                .decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf)?;

            if let Some(parent) = downloaded_file_path_buf.parent() {
                fs::remove_dir_all(parent)?;
            }

            let binary = snm_package_manage.get_runtime_binary_dir_string(version.as_str())?;

            vec![node_dir.clone(), binary]
        } else if restricted_list.contains(&command.as_str()) {
            return Err(SnmError::NotMatchPackageManagerError {
                raw_command: args_all.join(" ").to_string(),
                expected: package_manager.name,
                actual: prefix.to_string(),
            });
        } else {
            vec![node_dir.clone()]
        }
    } else {
        vec![node_dir.clone()]
    };

    exec_cli(bin_dirs, bin_name, &args)?;

    Ok(())
}
