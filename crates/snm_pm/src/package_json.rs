use std::{
  collections::HashMap,
  fs::File,
  io::BufReader,
  path::{MAIN_SEPARATOR_STR, Path, PathBuf},
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
pub struct PJson {
  name: Option<String>,

  version: Option<String>,

  #[serde(rename = "packageManager")]
  pub package_manager: Option<String>,

  bin: Option<Bin>,

  #[serde(skip)]
  internal_bin: Option<HashMap<String, PathBuf>>,
}

impl PJson {
  pub fn from<P: AsRef<Path>>(dir: P) -> anyhow::Result<Self> {
    let raw_file_path = dir.as_ref().join("package.json");

    raw_file_path
      .exists()
      .then(|| File::open(&raw_file_path).ok())
      .flatten()
      .map(BufReader::new)
      .and_then(|reader| serde_json::from_reader(reader).ok())
      .map(|mut raw: Self| {
        // 处理 bin

        raw.bin.as_ref().map(|bin| {
          raw.internal_bin = Some(Self::parse_bin(&dir, bin));
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
    let path = workspace.as_ref();
    match bin {
      Bin::Str(_bin_path) => {
        // // 从 package name 中解析命令名称（处理作用域包）
        // let command_name = package_name
        //   .split('/') // 分割作用域和包名
        //   .last() // 取最后一部分（包名）
        //   .unwrap_or(package_name)
        //   .to_string();

        // // 构建完整路径（处理路径分隔符）
        // let full_path = workspace
        //   .as_ref()
        //   .join(bin_path.replace('/', MAIN_SEPARATOR_STR));

        // // 返回单个键值对
        // HashMap::from_iter([(command_name, full_path)])
        HashMap::new()
      }
      Bin::Map(map) => map
        .into_iter()
        .filter_map(|(k, v)| Some((k.to_string(), path.join(v.replace('/', MAIN_SEPARATOR_STR)))))
        .collect(),
    }
  }
}

impl PJson {
  pub fn exists(dir: &PathBuf) -> bool {
    dir.join("package.json").exists()
  }

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
    let bin = PJson::parse_bin(".", &Bin::Map(HashMap::new()));
    assert_eq!(bin, HashMap::new());
  }

  #[test]
  fn test_parse_bin_with_path() {
    let mut bin = HashMap::new();
    bin.insert("pnpm".to_string(), "pnpm".to_string());
    bin.insert("pnpx".to_string(), "pnpx".to_string());
    let internal_bin = PJson::parse_bin("/usr/local/bin", &Bin::Map(bin));
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
