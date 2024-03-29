use super::read_to_json;
use crate::model::SnmError;
use crate::model::{Bin, PackageJsonModel};
use regex::{Captures, Regex};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

pub async fn parse_package_json_bin_to_hashmap(
    package_json_abs_path: &PathBuf,
) -> Result<HashMap<String, String>, SnmError> {
    let package_json = read_to_json::<PackageJsonModel>(&package_json_abs_path)?;

    let dir = package_json_abs_path.parent().unwrap();

    if let Some(bin) = package_json.bin {
        match bin {
            Bin::Str(_bin) => {
                unimplemented!(
                    "parse_package_json_bin_to_hashmap   if bin value is str {}",
                    _bin
                );
            }
            Bin::Map(map) => {
                let new_map = map
                    .into_iter()
                    .filter_map(|(key, value)| {
                        dir.join(value)
                            .canonicalize()
                            .ok()
                            .map(|path| (key, path.display().to_string()))
                    })
                    .collect::<HashMap<String, String>>();

                return Ok(new_map);
            }
            _ => {
                return Err(SnmError::PackageJsonBinPropertyUnknownTypeError {
                    file_path: package_json_abs_path.display().to_string(),
                });
            }
        }
    } else {
        return Err(SnmError::PackageJsonBinPropertyNotFound {
            file_path: package_json_abs_path.display().to_string(),
        });
    }
}

#[derive(Debug, Deserialize)]
pub struct VersionParsed {
    pub raw: String,
    pub package_manager: String,
    pub version: String,
    pub hash: Option<(String, String)>,
}

pub fn parse_package_manager_config(
    package_manager_config_str: &str,
) -> Result<VersionParsed, SnmError> {
    let regex_str = r"^(?P<package_manager>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";

    let regex = Regex::new(regex_str)?;

    let map_to_struct = |caps: Captures| VersionParsed {
        raw: package_manager_config_str.to_string(),
        package_manager: caps["package_manager"].to_string(),
        version: caps["version"].to_string(),
        hash: caps.name("hash_method").and_then(|m| {
            caps.name("hash_value")
                .map(|v| (m.as_str().to_string(), v.as_str().to_string()))
        }),
    };

    match regex
        .captures(package_manager_config_str)
        .map(map_to_struct)
    {
        Some(version_parsed) => Ok(version_parsed),
        None => Err(SnmError::ParsePackageManagerConfigError {
            raw_value: package_manager_config_str.to_string(),
        }),
    }
}

pub fn automatic_version_parsed(workspace: Option<PathBuf>) -> Result<VersionParsed, SnmError> {
    let wk = match workspace {
        Some(workspace) => workspace,
        None => std::env::current_dir()?,
    };
    let pkg_file_path = wk.join("package.json");

    let pkg = read_to_json::<PackageJsonModel>(&pkg_file_path)?;

    return match pkg.package_manager {
        Some(package_manager) => parse_package_manager_config(&package_manager),
        None => Err(SnmError::NoPackageManagerError {
            file_path: pkg_file_path.display().to_string(),
        }),
    };
}
