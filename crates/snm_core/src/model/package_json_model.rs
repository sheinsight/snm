use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::utils::read_to_json;

use super::SnmError;

#[derive(Debug, Deserialize)]
pub struct PackageManager {
    pub raw: String,
    pub package_manager: String,
    pub version: String,
    pub hash: Option<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub bin: Option<Bin>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}

impl PackageJson {
    pub fn from_file_path(workspace: Option<&PathBuf>) -> Result<Self, SnmError> {
        let pkg_file_path = if let Some(wk) = workspace {
            wk.join("package.json")
        } else {
            std::env::current_dir()?.join("package.json")
        };
        let pkg = read_to_json::<PackageJson>(&pkg_file_path)?;
        return Ok(pkg);
    }

    pub fn parse_package_manager(&self) -> Result<PackageManager, SnmError> {
        if let Some(raw_package_manager) = &self.package_manager {
            let regex_str = r"^(?P<package_manager>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";

            let regex = Regex::new(regex_str)?;

            let map_to_struct = |caps: Captures| PackageManager {
                raw: raw_package_manager.clone(),
                package_manager: caps["package_manager"].to_string(),
                version: caps["version"].to_string(),
                hash: caps.name("hash_method").and_then(|m| {
                    caps.name("hash_value")
                        .map(|v| (m.as_str().to_string(), v.as_str().to_string()))
                }),
            };
            return Ok(regex
                .captures(raw_package_manager.as_str())
                .map(map_to_struct)
                .ok_or(SnmError::UnknownError)?);
        }
        return Err(SnmError::UnknownError);
    }
}
