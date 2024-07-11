use std::ops::Not;

use snm_core::traits::atom::AtomTrait;
use snm_utils::snm_error::SnmError;

use super::download::download;

pub async fn ensure_binary_path(
    manage: &dyn AtomTrait,
    version: &String,
) -> Result<String, SnmError> {
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

    let binary = manage.get_runtime_binary_dir_string(version.as_str())?;

    return Ok(binary);
}
