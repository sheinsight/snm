use core::panic;
use std::{
    fs,
    ops::Not,
    path::PathBuf,
    process::{Command, Stdio},
};

use colored::*;
use snm_config::SnmConfig;
use snm_core::{
    model::dispatch_manage::DispatchManage,
    println_success,
    traits::manage::ManageTrait,
    utils::download::{DownloadBuilder, WriteStrategy},
};
use snm_utils::snm_error::SnmError;

async fn download<'a>(shim: &Box<dyn ManageTrait + 'a>, v: &str) -> Result<(), SnmError> {
    let download_url = shim.get_download_url(v);
    let downloaded_file_path_buf = match shim.get_downloaded_file_path_buf(v) {
        Ok(downloaded_file_path_buf) => downloaded_file_path_buf,
        Err(_) => panic!("download get_downloaded_file_path_buf error"),
    };
    DownloadBuilder::new()
        .retries(3)
        .write_strategy(WriteStrategy::Nothing)
        .download(&download_url, &downloaded_file_path_buf)
        .await;

    let runtime_dir_path_buf = shim.get_runtime_dir_path_buf(v)?;

    shim.decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf);

    let remove_result = fs::remove_file(&downloaded_file_path_buf);

    if remove_result.is_err() {
        let msg = format!(
            "download remove_file error {:?}",
            &downloaded_file_path_buf.display()
        );
        panic!("{msg}");
    }

    Ok(())
}

pub async fn node_exec_strict<'a>(
    shim: &Box<dyn ManageTrait + 'a>,
    snm_config: SnmConfig,
    bin_name: &str,
    v: Option<String>,
) -> Result<PathBuf, SnmError> {
    if let Some(node_version) = v {
        if shim
            .get_anchor_file_path_buf(node_version.as_str())?
            .exists()
            .not()
        {
            if shim.download_condition(node_version.as_str()) {
                // download
                download(&shim, node_version.as_str()).await?;
            } else {
                // exit 0
            }
        }

        let binary = shim.get_strict_shim_binary_path_buf(bin_name, node_version.as_str())?;

        return Ok(binary);
    }

    Err(SnmError::NotFoundNodeVersionConfigFile)
}

pub async fn exec_default(
    shim: &Box<dyn ManageTrait>,
    bin_name: &str,
) -> Result<(String, PathBuf), SnmError> {
    let tuple = read_runtime_dir_name_vec(&shim)?;
    let v = shim.check_default_version(&tuple);
    let binary_path_buf = shim.get_runtime_binary_file_path_buf(bin_name, &v)?;
    Ok((v, binary_path_buf))
}

fn read_runtime_dir_name_vec(
    shim: &Box<dyn ManageTrait>,
) -> Result<(Vec<String>, Option<String>), SnmError> {
    let runtime_dir_path_buf = shim.get_runtime_base_dir_path_buf()?;

    let mut default_dir = None;

    if runtime_dir_path_buf.exists().not() {
        // TODO here create not suitable , should be find a better way
        fs::create_dir_all(&runtime_dir_path_buf).expect(
            format!(
                "read_runtime_dir_name_vec create_dir_all error {:?}",
                &runtime_dir_path_buf.display()
            )
            .as_str(),
        );
    }

    let dir_name_vec = runtime_dir_path_buf
        .read_dir()
        .expect(
            format!(
                "read_runtime_dir_name_vec read_dir error {:?}",
                &runtime_dir_path_buf.display()
            )
            .as_str(),
        )
        .filter_map(|dir_entry| dir_entry.ok())
        .filter(|dir_entry| dir_entry.path().is_dir())
        .filter_map(|dir_entry| {
            let file_name = dir_entry.file_name().into_string().ok()?;

            if file_name.ends_with("-default") {
                default_dir = Some(file_name.trim_end_matches("-default").to_string());
                return None;
            }

            return Some(file_name);
        })
        .collect::<Vec<String>>();

    Ok((dir_name_vec, default_dir))
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
