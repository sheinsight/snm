use std::{
  collections::HashMap,
  fs::File,
  io::BufReader,
  path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use anyhow::Context;
use serde::Deserialize;

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
  pub package_manager: Option<String>,

  bin: Option<Bin>,

  #[serde(skip)]
  internal_bin: Option<HashMap<String, PathBuf>>,
}

impl PackageJson {
  pub fn from<P: AsRef<Path>>(workspace: P) -> anyhow::Result<Self> {
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
        // raw.package_manager.as_ref().map(|pm| {
        //     raw.internal_package_manager = PackageManager::parse(pm);
        // });

        raw
      })
      .with_context(|| format!("Failed to parse package.json: {}", raw_file_path.display()))
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
  pub fn get_bin_with_name(&self, name: &str) -> anyhow::Result<PathBuf> {
    self
      .internal_bin
      .as_ref()
      .and_then(|bin| bin.get(name).cloned())
      .with_context(|| format!("Bin not found: {}", name))
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
