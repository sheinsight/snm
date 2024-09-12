use std::{collections::HashMap, fs::File, io::BufReader, ops::Not, path::PathBuf};

use regex::{Match, Regex};
use serde::Deserialize;
use snm_utils::{constant::PACKAGE_MANAGER, snm_error::SnmError};

use crate::{
    package_manager_meta::{PackageManager, PackageManagerDownloadHash},
    package_manager_raw::{Bin, PackageJsonRaw},
};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageJson {
    pub package_manager: Option<PackageManager>,

    pub name: Option<String>,

    pub version: Option<String>,

    pub bin: HashMap<String, PathBuf>,

    pub raw: PackageJsonRaw,

    pub raw_file_path: PathBuf,

    pub raw_workspace: PathBuf,
}

pub fn parse_package_json(workspace: &PathBuf) -> Result<Option<PackageJson>, SnmError> {
    let raw_file_path = workspace.join("package.json");

    if raw_file_path.exists() {
        let file = File::open(&raw_file_path)?;
        let reader = BufReader::new(&file);

        let raw = serde_json::from_reader::<_, PackageJsonRaw>(reader)?;

        let package_manager = if let Some(raw_package_manager) = &raw.package_manager {
            parse_package_manager(raw_package_manager.as_str())?
        } else {
            None
        };
        let bin_hashmap = if let Some(b) = &raw.bin {
            bin_transform_to_hashmap(b, &workspace)
        } else {
            HashMap::new()
        };
        return Ok(Some(PackageJson {
            bin: bin_hashmap,
            name: raw.name.clone(),
            version: raw.version.clone(),
            package_manager,
            raw,
            raw_workspace: workspace.clone(),
            raw_file_path,
        }));
    } else {
        Ok(None)
    }
}

fn bin_transform_to_hashmap(bin: &Bin, raw_workspace: &PathBuf) -> HashMap<String, PathBuf> {
    match bin {
        Bin::Str(_) => {
            unimplemented!("parse_package_json_bin_to_hashmap   if bin value is str")
        }
        Bin::Map(map) => map
            .into_iter()
            .filter_map(|(k, v)| {
                if let Some(mut bin_path_buf) = raw_workspace.join(v).canonicalize().ok() {
                    if cfg!(target_os = "windows") {
                        if let Some(stripped) = bin_path_buf
                            .to_str()
                            .map(|s| s.strip_prefix("\\\\?\\"))
                            .flatten()
                        {
                            bin_path_buf = PathBuf::from(stripped);
                        }
                    }
                    Some((k.to_string(), bin_path_buf))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, PathBuf>>(),
    }
}

fn parse_package_manager(raw_package_manager: &str) -> Result<Option<PackageManager>, SnmError> {
    let regex_str = r"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";
    let regex = Regex::new(regex_str).unwrap();

    let map_none = |item: Match| {
        if item.is_empty() {
            None
        } else {
            Some(item.as_str().to_string())
        }
    };

    match regex.captures(raw_package_manager) {
        Some(caps) => {
            let name =
                caps.name("name")
                    .and_then(map_none)
                    .ok_or(SnmError::ParsePackageManagerError {
                        raw_package_manager: raw_package_manager.to_string(),
                    })?;

            if PACKAGE_MANAGER.contains(&name.as_str()).not() {
                return Err(SnmError::UnsupportedPackageManagerError {
                    name: name.to_string(),
                    raw: raw_package_manager.to_string(),
                    supported: PACKAGE_MANAGER.iter().map(|s| s.to_string()).collect(),
                });
            }

            let version = caps.name("version").and_then(map_none).ok_or(
                SnmError::ParsePackageManagerError {
                    raw_package_manager: raw_package_manager.to_string(),
                },
            )?;

            let hash_name = caps.name("hash_method").and_then(map_none);

            let hash_value = caps.name("hash_value").and_then(map_none);

            return Ok(Some(PackageManager {
                raw: raw_package_manager.to_string(),
                name,
                version,
                hash: Some(PackageManagerDownloadHash {
                    value: hash_value,
                    name: hash_name,
                }),
            }));
        }
        None => {
            return Err(SnmError::ParsePackageManagerError {
                raw_package_manager: raw_package_manager.to_string(),
            });
        }
    }
}
