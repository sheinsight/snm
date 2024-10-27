use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::package_manager_meta::{PackageManager, PackageManagerDownloadHash};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageJsonRaw {
    pub name: Option<String>,

    pub version: Option<String>,

    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub bin: Option<Bin>,

    #[serde(skip)]
    internal_bin: Option<HashMap<String, PathBuf>>,

    #[serde(skip)]
    internal_package_manager: Option<PackageManager>,
}

impl PackageJsonRaw {
    fn from<P: AsRef<Path>>(workspace: P) -> Option<Self> {
        let raw_file_path = workspace.as_ref().join("package.json");
        let mut x: Option<Self> = raw_file_path
            .exists()
            .then(|| File::open(&raw_file_path).ok())
            .flatten()
            .map(BufReader::new)
            .and_then(|reader| serde_json::from_reader(reader).ok());

        if let Some(ref mut raw) = x {
            if let Some(bin) = &raw.bin {
                raw.internal_bin = Some(Self::parse_bin(&workspace, &bin));
            }

            if let Some(pm) = &raw.package_manager {
                raw.internal_package_manager = Self::parse_package_manager(pm);
            }
        }

        x
    }

    fn parse_bin<P: AsRef<Path>>(workspace: P, bin: &Bin) -> HashMap<String, PathBuf> {
        match bin {
            Bin::Str(s) => HashMap::new(),
            Bin::Map(map) => map
                .into_iter()
                .filter_map(|(k, v)| {
                    if let Some(path) = workspace
                        .as_ref()
                        .join(v.replace('/', MAIN_SEPARATOR_STR))
                        .canonicalize()
                        .ok()
                    {
                        Some((k.to_string(), path))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    fn parse_package_manager(raw: &str) -> Option<PackageManager> {
        let regex_str = r"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";
        let regex = match Regex::new(regex_str) {
            Ok(regex) => regex,
            Err(_) => return None,
        };

        let captures = match regex.captures(raw) {
            Some(caps) => caps,
            None => return None,
        };

        let [name, version, hash_method, hash_value] =
            ["name", "version", "hash_method", "hash_value"]
                .map(|name| captures.name(name).map(|s| s.as_str().to_string()));

        let package_manager = match (name, version, hash_method, hash_value) {
            (Some(name), Some(version), None, None) => PackageManager {
                name,
                version,
                hash: None,
                raw: raw.to_string(),
            },
            (Some(name), Some(version), Some(hash_method), Some(hash_value)) => PackageManager {
                name,
                version,
                hash: Some(PackageManagerDownloadHash {
                    name: Some(hash_method),
                    value: Some(hash_value),
                }),
                raw: raw.to_string(),
            },
            _ => {
                return None;
            }
        };

        Some(package_manager)
    }
}

impl PackageJsonRaw {
    pub fn get_pm_name(&self) -> Option<String> {
        self.package_manager
            .as_deref()
            .and_then(|s| s.split('@').next().map(|s| s.to_string()))
    }

    pub fn get_pm_version(&self) -> Option<String> {
        self.package_manager
            .as_deref()
            .and_then(|s| s.split('@').nth(1).map(|s| s.to_string()))
    }

    pub fn get_bin_with_name(&self, name: &str) -> Option<String> {
        self.bin.as_ref().and_then(|bin| match bin {
            Bin::Str(s) => Some(s.clone()),
            Bin::Map(map) => map.get(name).map(|s| s.clone()),
        })
    }
}
