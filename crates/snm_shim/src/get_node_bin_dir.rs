use std::{fs, ops::Not as _};

use anyhow::bail;
use snm_atom::{atom::AtomTrait, node_atom::NodeAtom};
use snm_config::SnmConfig;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_node_version::NodeVersionReader;
use snm_utils::snm_error::SnmError;

pub fn check_node_v(config: SnmConfig, v: String) -> anyhow::Result<()> {
    if config.node_white_list.is_empty() {
        return Ok(());
    }
    if config.node_white_list.contains(&v) {
        return Ok(());
    }
    bail!(SnmError::UnsupportedNodeVersionError {
        version: v,
        node_white_list: config
            .node_white_list
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    });
}

pub async fn get_node_bin_dir(config: SnmConfig) -> anyhow::Result<String> {
    let snm_node = NodeAtom::new(config.clone());

    let version = NodeVersionReader::from_env(&config)?.read_version();

    check_node_v(config, version.clone())?;

    if snm_node
        .get_anchor_file_path_buf(version.as_str())?
        .exists()
        .not()
    {
        let download_url = snm_node.get_download_url(&version);

        let downloaded_file_path_buf = snm_node.get_downloaded_file_path_buf(&version)?;

        DownloadBuilder::new()
            .retries(3)
            .timeout(snm_node.get_snm_config().download_timeout_secs)
            .write_strategy(WriteStrategy::WriteAfterDelete)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        let runtime_dir_path_buf = snm_node.get_runtime_dir_path_buf(&version)?;

        let downloaded_file_path_buf = snm_node.get_downloaded_file_path_buf(&version)?;

        let expect = snm_node.get_expect_shasum(&version).await?;

        let actual = snm_node
            .get_actual_shasum(&downloaded_file_path_buf)
            .await?;

        if actual.is_none() || expect.is_none() {
            fs::remove_file(&downloaded_file_path_buf)?;
            bail!(SnmError::ShasumError {
                file_path: downloaded_file_path_buf.display().to_string(),
                expect: "None".to_string(),
                actual: "None".to_string(),
            });
        }

        if actual.eq(&expect).not() {
            fs::remove_file(&downloaded_file_path_buf)?;
            bail!(SnmError::ShasumError {
                file_path: downloaded_file_path_buf.display().to_string(),
                expect: expect.unwrap_or("None".to_string()),
                actual: actual.unwrap_or("None".to_string()),
            });
        }

        snm_node.decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf)?;

        if let Some(parent) = downloaded_file_path_buf.parent() {
            fs::remove_dir_all(parent)?;
        }
    }
    let binary = snm_node.get_runtime_binary_dir_string(version.as_str())?;

    Ok(binary)
}
