use std::{ops::Not as _, path::Path};

use snm_utils::snm_error::SnmError;
use tracing::instrument;

#[instrument(level = "trace", ret)]
pub fn get_default_bin_dir(node_dir: &str, bin_name: &str) -> Result<String, SnmError> {
    let default_bin_dir = Path::new(&node_dir);

    let default_bin = default_bin_dir.join(bin_name);
    if default_bin.exists().not() {
        return Err(SnmError::CannotFindDefaultCommand {
            command: bin_name.to_string(),
        });
    } else {
        Ok(default_bin_dir.display().to_string())
    }
}
