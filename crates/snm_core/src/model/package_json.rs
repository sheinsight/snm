use regex::{Captures, Regex};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use super::SnmError;

#[derive(Debug, Deserialize)]
pub struct PackageManager {
    pub raw: String,
    pub name: String,
    pub version: String,
    pub hash: Option<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub version: Option<String>,

    pub bin: Option<Bin>,

    #[serde(skip_serializing)]
    pub _raw_file_path: Option<PathBuf>,
    #[serde(skip_serializing)]
    pub _raw_workspace: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}

impl PackageJson {
    pub fn from_dir_path(workspace: Option<PathBuf>) -> Result<Self, SnmError> {
        let wk = if let Some(wk) = workspace {
            wk
        } else {
            std::env::current_dir().expect("get current dir error.")
        };

        let pkg_file_path = wk.join("package.json");

        if pkg_file_path.exists() {
            let mut pkg = read_to_json::<PackageJson>(&pkg_file_path);

            pkg._raw_file_path = Some(pkg_file_path);
            pkg._raw_workspace = Some(wk);
            return Ok(pkg);
        }

        return Err(SnmError::NotFoundPackageJsonFileError {
            package_json_file_path: pkg_file_path.display().to_string(),
        });
    }

    pub fn from_file_path(file_path: &PathBuf) -> Result<Self, SnmError> {
        let mut pkg = read_to_json::<PackageJson>(&file_path);
        pkg._raw_file_path = Some(file_path.to_path_buf());

        pkg._raw_workspace = file_path.parent().map(|x| x.to_path_buf());
        Ok(pkg)
    }

    pub fn parse_package_manager(&self) -> Result<PackageManager, SnmError> {
        if let Some(raw_package_manager) = &self.package_manager {
            let regex_str = r"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";

            let regex = Regex::new(regex_str).expect("create regex error");

            let map_to_struct = |caps: Captures| PackageManager {
                raw: raw_package_manager.clone(),
                name: caps["name"].to_string(),
                version: caps["version"].to_string(),
                hash: caps.name("hash_method").and_then(|m| {
                    caps.name("hash_value")
                        .map(|v| (m.as_str().to_string(), v.as_str().to_string()))
                }),
            };

            let res = regex
                .captures(raw_package_manager.as_str())
                .map(map_to_struct)
                .expect("parse_package_manager error");

            return Ok(res);
        }
        return Err(SnmError::NotFoundPackageJsonBinProperty {
            file_path: self._raw_file_path.clone().unwrap().display().to_string(),
        });
    }

    pub fn bin_to_hashmap(&self) -> Result<HashMap<String, PathBuf>, SnmError> {
        let raw_workspace = self._raw_workspace.as_ref().unwrap();
        if let Some(bin) = &self.bin {
            match bin {
                Bin::Str(_) => {
                    unimplemented!("parse_package_json_bin_to_hashmap   if bin value is str")
                }
                Bin::Map(map) => {
                    let new_map = map
                        .into_iter()
                        .filter_map(|(k, v)| {
                            if let Some(absolute_file_path) =
                                raw_workspace.join(v).canonicalize().ok()
                            {
                                return Some((k.to_string(), absolute_file_path));
                            } else {
                                return None;
                            }
                        })
                        .collect::<HashMap<String, PathBuf>>();
                    Ok(new_map)
                }
            }
        } else {
            return Err(SnmError::NotFoundPackageJsonBinProperty {
                file_path: raw_workspace.display().to_string(),
            });
        }
    }
}

fn read_to_json<T: DeserializeOwned>(file_path: &PathBuf) -> T {
    let content =
        read_to_string(&file_path).expect(format!("read {:?} error", &file_path).as_str());
    serde_json::from_str::<T>(&content)
        .expect(format!("parse {:?} json error", &file_path).as_str())
}
