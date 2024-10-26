use std::{fs::read_to_string, path::PathBuf};

const FILE_NAME: &str = ".node-version";
pub struct NodeVersionReader {
    version: Option<String>,
}

impl NodeVersionReader {
    pub fn from(workspace: &PathBuf) -> Self {
        let file_path = workspace.join(FILE_NAME);

        let version = if file_path.exists() {
            if let Ok(raw) = read_to_string(&file_path) {
                let version = raw.trim_start_matches(['v', 'V']).trim().to_owned();
                let version_parts = version.split('.').collect::<Vec<_>>();
                let has_invalid_part = version_parts.iter().any(|s| s.parse::<u32>().is_err());
                if version_parts.len() != 3 || has_invalid_part {
                    None
                } else {
                    Some(version.to_owned())
                }
            } else {
                None
            }
        } else {
            None
        };

        Self { version }
    }

    pub fn read_version(&self) -> Option<String> {
        self.version.clone()
    }
}
