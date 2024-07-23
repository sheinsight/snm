use std::{env::current_dir, fs, ops::Not as _};

use snm_atom::{atom::AtomTrait as _, node_atom::NodeAtom};
use snm_config::parse_snm_config;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::snm_error::SnmError;
use tracing::{instrument, Level};

use crate::ensure_binary_path;

#[instrument(level = Level::TRACE, ret)]
pub async fn get_node_bin_dir() -> Result<String, SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node = NodeAtom::new(snm_config.clone());
    let version = if let Some(version) = snm_config.get_runtime_node_version() {
        version
    } else {
        let (_, version) = snm_node.read_runtime_dir_name_vec()?;
        version.ok_or(SnmError::NoDefaultNodeBinary)?
    };

    let node_white_list = snm_config.get_node_white_list();

    if node_white_list.contains(&version).not() {
        return Err(SnmError::UnsupportedNodeVersionError {
            actual: version.to_string(),
            expect: node_white_list,
        });
    }

    if snm_node
        .get_anchor_file_path_buf(version.as_str())?
        .exists()
        .not()
    {
        let download_url = snm_node.get_download_url(&version);

        let downloaded_file_path_buf = snm_node.get_downloaded_file_path_buf(&version)?;

        DownloadBuilder::new()
            .retries(3)
            .timeout(snm_node.get_snm_config().get_download_timeout_secs())
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
            return Err(SnmError::ShasumError {
                file_path: downloaded_file_path_buf.display().to_string(),
                expect: "None".to_string(),
                actual: "None".to_string(),
            });
        }

        if actual.eq(&expect).not() {
            fs::remove_file(&downloaded_file_path_buf)?;
            return Err(SnmError::ShasumError {
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

    let binary_dir_string = ensure_binary_path(&snm_node, &version, true).await?;

    Ok(binary_dir_string)
}
