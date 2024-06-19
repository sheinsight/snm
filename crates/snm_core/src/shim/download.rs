use std::fs;

use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

pub async fn download(version: &str, manage: &dyn AtomTrait) -> Result<(), SnmError> {
    let download_url = manage.get_download_url(version);

    let downloaded_file_path_buf = manage.get_downloaded_file_path_buf(version)?;

    DownloadBuilder::new()
        .retries(3)
        .timeout(manage.get_snm_config().get_download_timeout_secs())
        .write_strategy(WriteStrategy::Nothing)
        .download(&download_url, &downloaded_file_path_buf)
        .await?;

    let runtime_dir_path_buf = manage.get_runtime_dir_path_buf(version)?;

    manage.decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf);

    fs::remove_file(&downloaded_file_path_buf)?;

    Ok(())
}
