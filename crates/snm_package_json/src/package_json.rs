use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::pm::PackageManager;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageJson {
    name: Option<String>,

    version: Option<String>,

    #[serde(rename = "packageManager")]
    package_manager: Option<String>,

    bin: Option<Bin>,

    #[serde(skip)]
    internal_bin: Option<HashMap<String, PathBuf>>,

    #[serde(skip)]
    internal_package_manager: Option<PackageManager>,
}

impl PackageJson {
    pub fn from<P: AsRef<Path>>(workspace: P) -> Option<Self> {
        let raw_file_path = workspace.as_ref().join("package.json");
        raw_file_path
            .exists()
            .then(|| File::open(&raw_file_path).ok())
            .flatten()
            .map(BufReader::new)
            .and_then(|reader| serde_json::from_reader(reader).ok())
            .map(|mut raw: Self| {
                // 处理 bin
                raw.bin.as_ref().map(|bin| {
                    raw.internal_bin = Some(Self::parse_bin(&workspace, bin));
                });

                // 处理 package_manager
                raw.package_manager.as_ref().map(|pm| {
                    raw.internal_package_manager = PackageManager::parse(pm);
                });

                raw
            })
    }

    fn parse_bin<P: AsRef<Path>>(workspace: P, bin: &Bin) -> HashMap<String, PathBuf> {
        match bin {
            Bin::Str(_) => HashMap::new(),
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
}

impl PackageJson {
    pub fn get_pm(&self) -> Option<PackageManager> {
        self.internal_package_manager.as_ref().cloned()
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
