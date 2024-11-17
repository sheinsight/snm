use std::env::{self, current_dir};

use anyhow::bail;
use snm_config::SnmConfig;
use snm_package_json::{package_json::PackageJson, pm::PackageManager};
use snm_utils::{constant::RESTRICTED_LIST, exec::exec_cli};

pub async fn package_manager(prefix: &str, bin_name: &str) -> anyhow::Result<()> {
    let args_all: Vec<String> = env::args().collect();

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(&cwd)?;

    let json = PackageJson::from(&cwd)?;

    let package_manager = match PackageManager::from_env(&snm_config) {
        Ok(pm) => pm,
        Err(_) => match json.package_manager {
            Some(raw) => PackageManager::from_str(&raw, &snm_config).unwrap(),
            None => bail!("No package manager found"),
        },
    };

    let version = package_manager.version();

    let bin_dir = package_manager.get_bin(version, prefix).await?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    // let bin_dirs = if let Some(pm) = package_manager {
    //     let snm_package_manage = PackageManagerAtom::new(prefix, snm_config.clone());

    //     if pm.name() == prefix {
    //         let version = pm.version();

    //         if snm_package_manage
    //             .get_anchor_file_path_buf(&version)?
    //             .exists()
    //             .not()
    //         {
    //             let download_url = snm_package_manage.get_download_url(&version);

    //             let downloaded_file_path_buf =
    //                 snm_package_manage.get_downloaded_file_path_buf(&version)?;

    //             DownloadBuilder::new()
    //                 .retries(3)
    //                 .timeout(snm_package_manage.get_snm_config().download_timeout_secs)
    //                 .write_strategy(WriteStrategy::WriteAfterDelete)
    //                 .download(&download_url, &downloaded_file_path_buf)
    //                 .await?;

    //             let runtime_dir_path_buf = snm_package_manage.get_runtime_dir_path_buf(&version)?;

    //             snm_package_manage
    //                 .decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf)?;

    //             if let Some(parent) = downloaded_file_path_buf.parent() {
    //                 fs::remove_dir_all(parent)?;
    //             }
    //         }

    //         let binary = snm_package_manage.get_runtime_binary_dir_string(version)?;

    //         vec![node_dir.clone(), binary]
    //     } else if RESTRICTED_LIST.contains(&command.as_str()) {
    //         bail!(SnmError::NotMatchPackageManagerError {
    //             raw_command: args_all.join(" ").to_string(),
    //             expect: pm.name().to_string(),
    //             actual: prefix.to_string(),
    //         });
    //     } else {
    //         vec![node_dir.clone()]
    //     }
    // } else {
    //     vec![node_dir.clone()]
    // };

    // let exists = bin_dirs
    //     .iter()
    //     .any(|dir| PathBuf::from(dir).join(bin_name).exists());

    // if exists.not() {
    //     bail!(SnmError::NotFoundCommandError {
    //         bin_name: bin_name.to_string(),
    //     });
    // }

    exec_cli(vec![bin_dir], bin_name, &args)?;

    Ok(())
}
