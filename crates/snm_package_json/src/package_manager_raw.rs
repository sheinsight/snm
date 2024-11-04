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
pub struct PackageJson {
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

impl PackageJson {
    pub fn from<P: AsRef<Path>>(workspace: P) -> Option<Self> {
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
                    Some((
                        k.to_string(),
                        workspace.as_ref().join(v.replace('/', MAIN_SEPARATOR_STR)),
                    ))
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

impl PackageJson {
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

    pub fn get_bin_with_name(&self, name: &str) -> Option<PathBuf> {
        let x = self.internal_bin.as_ref().and_then(|bin| bin.get(name));

        x.as_deref().cloned()
    }

    pub fn enumerate_bin<F>(&self, f: F)
    where
        F: Fn(&str, &PathBuf),
    {
        for (k, v) in self.internal_bin.as_ref().unwrap() {
            f(k, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_manager_with_pnpm() {
        let pm = PackageJson::parse_package_manager("pnpm@9.0.0");
        assert!(pm.is_some());
        if let Some(pm) = pm {
            assert_eq!(&pm.name, "pnpm");
            assert_eq!(&pm.version, "9.0.0");
        }
    }

    #[test]
    fn test_parse_package_manager_with_pnpm_and_hash() {
        let pm = PackageJson::parse_package_manager("pnpm@9.0.0+sha.1234567890");
        assert!(pm.is_some());

        if let Some(pm) = pm {
            let hash = pm.hash.unwrap();
            assert_eq!(&pm.name, "pnpm");
            assert_eq!(&pm.version, "9.0.0");
            assert_eq!(hash.name.as_deref(), Some("sha"));
            assert_eq!(hash.value.as_deref(), Some("1234567890"));
        }
    }

    #[test]
    fn test_parse_package_manager_with_npm() {
        let pm = PackageJson::parse_package_manager("npm@10.0.0");
        assert!(pm.is_some());

        if let Some(pm) = pm {
            assert_eq!(&pm.name, "npm");
            assert_eq!(&pm.version, "10.0.0");
        }
    }

    #[test]
    fn test_parse_bin() {
        let bin = PackageJson::parse_bin(".", &Bin::Map(HashMap::new()));
        assert_eq!(bin, HashMap::new());
    }

    #[test]
    fn test_parse_bin_with_path() {
        let mut bin = HashMap::new();
        bin.insert("pnpm".to_string(), "pnpm".to_string());
        bin.insert("pnpx".to_string(), "pnpx".to_string());
        let internal_bin = PackageJson::parse_bin("/usr/local/bin", &Bin::Map(bin));
        assert_eq!(
            *internal_bin.get("pnpm").unwrap(),
            PathBuf::from("/usr/local/bin/pnpm")
        );
        assert_eq!(
            *internal_bin.get("pnpx").unwrap(),
            PathBuf::from("/usr/local/bin/pnpx")
        );
    }
}
