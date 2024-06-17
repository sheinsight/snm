use colored::*;
use snm_core::{
    traits::manage::ManageTrait,
    utils::download::{DownloadBuilder, WriteStrategy},
};
use snm_utils::snm_error::SnmError;
use snm_utils::to_ok::ToOk;
use std::{fs, ops::Not, path::PathBuf};

async fn download<'a>(manage: &Box<dyn ManageTrait + 'a>, v: &str) -> Result<(), SnmError> {
    let download_url = manage.get_download_url(v);

    let downloaded_file_path_buf = manage.get_downloaded_file_path_buf(v)?;

    DownloadBuilder::new()
        .retries(3)
        .write_strategy(WriteStrategy::Nothing)
        .download(&download_url, &downloaded_file_path_buf)
        .await;

    let runtime_dir_path_buf = manage.get_runtime_dir_path_buf(v)?;

    manage.decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf);

    fs::remove_file(&downloaded_file_path_buf)?;

    Ok(())
}

fn read_runtime_dir_name_vec(
    shim: &Box<dyn ManageTrait>,
) -> Result<(Vec<String>, Option<String>), SnmError> {
    let runtime_dir_path_buf = shim.get_runtime_base_dir_path_buf()?;

    let mut default_dir = None;

    if runtime_dir_path_buf.exists().not() {
        // TODO here create not suitable , should be find a better way
        fs::create_dir_all(&runtime_dir_path_buf)?;
    }

    let dir_name_vec = runtime_dir_path_buf
        .read_dir()?
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

pub async fn get_binary_path_buf_by_strict<'a>(
    manage: &Box<dyn ManageTrait + 'a>,
    bin_name: &str,
    v: Option<String>,
) -> Result<PathBuf, SnmError> {
    if let Some(node_version) = v {
        if manage
            .get_anchor_file_path_buf(node_version.as_str())?
            .exists()
            .not()
        {
            if manage.download_condition(node_version.as_str()) {
                download(&manage, node_version.as_str()).await?;
            } else {
                // exit 0
            }
        }

        let binary = manage.get_strict_shim_binary_path_buf(bin_name, node_version.as_str())?;

        return Ok(binary);
    }

    Err(SnmError::NotFoundNodeVersionConfigFile)
}

pub async fn get_binary_path_buf_by_default(
    manage: &Box<dyn ManageTrait>,
    bin_name: &str,
) -> Result<PathBuf, SnmError> {
    let (_, default_v) = read_runtime_dir_name_vec(&manage)?;
    if let Some(v) = default_v {
        return Ok(manage.get_runtime_binary_file_path_buf(bin_name, &v)?);
    } else {
        return Err(SnmError::NotFoundDefaultNodeVersion);
    }
}

pub fn get_default_version(manage: &Box<dyn ManageTrait>) -> Result<Option<String>, SnmError> {
    let (_, default_v) = read_runtime_dir_name_vec(&manage)?;
    return Ok(default_v);
}
