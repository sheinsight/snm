use colored::*;
use snm_config::{parse_snm_config, SnmConfig};
use snm_core::{println_error, traits::atom::AtomTrait};
use snm_current_dir::current_dir;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_node::snm_node::SnmNode;
use snm_node_version::parse_node_version;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::{fs, ops::Not, path::PathBuf};

async fn download<'a>(manage: &Box<dyn AtomTrait + 'a>, v: &str) -> Result<(), SnmError> {
    let download_url = manage.get_download_url(v);

    let downloaded_file_path_buf = manage.get_downloaded_file_path_buf(v)?;

    DownloadBuilder::new()
        .retries(3)
        .write_strategy(WriteStrategy::Nothing)
        .download(&download_url, &downloaded_file_path_buf)
        .await?;

    let runtime_dir_path_buf = manage.get_runtime_dir_path_buf(v)?;

    manage.decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf);

    fs::remove_file(&downloaded_file_path_buf)?;

    Ok(())
}

fn read_runtime_dir_name_vec(
    shim: &Box<dyn AtomTrait>,
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
    manage: &Box<dyn AtomTrait + 'a>,
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
    manage: &Box<dyn AtomTrait>,
    bin_name: &str,
) -> Result<PathBuf, SnmError> {
    let (_, default_v) = read_runtime_dir_name_vec(&manage)?;
    if let Some(v) = default_v {
        return Ok(manage.get_runtime_binary_file_path_buf(bin_name, &v)?);
    } else {
        return Err(SnmError::NotFoundDefaultNodeVersion);
    }
}

pub fn get_default_version(manage: &Box<dyn AtomTrait>) -> Result<Option<String>, SnmError> {
    let (_, default_v) = read_runtime_dir_name_vec(&manage)?;
    return Ok(default_v);
}

pub async fn get_binary_path_buf(
    bin_name: &str,
    version: Option<String>,
    manage: &Box<dyn AtomTrait>,
) -> Result<PathBuf, SnmError> {
    match version {
        Some(v) => get_binary_path_buf_by_strict(manage, bin_name, Some(v)).await,
        None => return Err(SnmError::NotFoundValidNodeVersion),
    }
}

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let mut version = None;
    if let Some(package_manager) = parse_package_json(&dir).and_then(|x| x.package_manager) {
        let name = package_manager.name.unwrap();
        version = package_manager.version;

        if snm_config.get_strict().not() && version.is_none() {
            let snm_node: Box<dyn AtomTrait> =
                Box::new(SnmPackageManager::from_prefix(prefix, snm_config.clone()));
            version = get_default_version(&snm_node)?;
        }

        if name != prefix {
            let msg = format!("you config {} but use {}", name, bin_name);
            panic!("{msg}");
        }
    } else {
        println_error!("No valid package manager found");
        return Ok(());
    }

    load_shim(bin_name, version, |snm_config| {
        SnmPackageManager::from_prefix(prefix, snm_config)
    })
    .await
}

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let mut version = parse_node_version(&snm_config.get_workspace()?)
        .ok()
        .and_then(|node_version| node_version.map(|nv| nv.get_version()))
        .flatten();

    if snm_config.get_strict().not() && version.is_none() {
        let snm_node: Box<dyn AtomTrait> = Box::new(SnmNode::new(snm_config.clone()));
        version = get_default_version(&snm_node)?;
    }

    load_shim(bin_name, version, SnmNode::new).await
}

pub async fn load_shim<T>(
    bin_name: &str,
    version: Option<String>,
    create_manager: impl Fn(SnmConfig) -> T,
) -> Result<(), SnmError>
where
    T: AtomTrait + 'static,
{
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: Box<dyn AtomTrait> = Box::new(create_manager(snm_config.clone()));

    let args: Vec<String> = std::env::args().skip(1).collect();

    let binary_path_buf = get_binary_path_buf(bin_name, version, &snm_node).await?;

    exec_cli(binary_path_buf, &args);

    Ok(())
}
