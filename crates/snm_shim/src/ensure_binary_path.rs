use std::{ops::Not, path::PathBuf};

use snm_core::traits::atom::AtomTrait;
use snm_utils::snm_error::SnmError;

use super::download::download;

pub async fn ensure_binary_path(
    bin_name: &str,
    manage: &dyn AtomTrait,
    version: &String,
) -> Result<PathBuf, SnmError> {
    if manage
        .get_anchor_file_path_buf(version.as_str())?
        .exists()
        .not()
    {
        if manage.download_condition(version.as_str()) {
            download(version.as_str(), manage).await?;
        } else {
            // exit 0
        }
    }

    let binary = manage.get_strict_shim_binary_path_buf(bin_name, version.as_str())?;

    return Ok(binary);
}
