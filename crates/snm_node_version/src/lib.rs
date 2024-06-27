use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;
use snm_utils::snm_error::SnmError;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct NodeVersion {
    version: Option<String>,
    pub raw: String,
    pub raw_file_path: PathBuf,
    pub raw_workspace: PathBuf,
}

impl NodeVersion {
    pub fn get_version(&self) -> Option<String> {
        self.version.clone()
    }
}

pub fn parse_node_version(workspace: &PathBuf) -> Result<Option<NodeVersion>, SnmError> {
    let raw_file_path = workspace.join(".node-version");

    if raw_file_path.exists() {
        let raw = read_to_string(&raw_file_path)?.trim().to_string();

        let version = if raw.is_empty() {
            None
        } else {
            Some(raw.trim_start_matches(['v', 'V']).to_string())
        };

        return Ok(Some(NodeVersion {
            version: version,
            raw: raw.to_string(),
            raw_workspace: workspace.clone(),
            raw_file_path,
        }));
    }

    Ok(None)
}
