use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use regex::Regex;
use serde::Deserialize;

use crate::{
    package_manager_meta::{PackageManager, PackageManagerDownloadHash},
    package_manager_raw::{Bin, PackageJsonRaw},
};

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub package_manager: Option<PackageManager>,

    pub name: Option<String>,

    pub version: Option<String>,

    pub bin: HashMap<String, PathBuf>,

    pub raw: PackageJsonRaw,

    pub raw_file_path: PathBuf,

    pub raw_workspace: PathBuf,
}

pub fn parse_package_json(workspace: &PathBuf) -> Option<PackageJson> {
    let raw_file_path = workspace.join("package.json");

    let file = File::open(&raw_file_path).unwrap();

    let reader = BufReader::new(&file);

    if raw_file_path.exists() {
        let raw = serde_json::from_reader::<_, PackageJsonRaw>(reader).unwrap();

        let package_manager = if let Some(raw_package_manager) = &raw.package_manager {
            parse_package_manager(raw_package_manager.as_str())
        } else {
            None
        };
        let bin_hashmap = if let Some(b) = &raw.bin {
            bin_transform_to_hashmap(b, &workspace)
        } else {
            HashMap::new()
        };
        return Some(PackageJson {
            bin: bin_hashmap,
            name: raw.name.clone(),
            version: raw.version.clone(),
            package_manager,
            raw,
            raw_workspace: workspace.clone(),
            raw_file_path,
        });
    }

    // Self {}
    None
}

fn bin_transform_to_hashmap(bin: &Bin, raw_workspace: &PathBuf) -> HashMap<String, PathBuf> {
    match bin {
        Bin::Str(_) => {
            unimplemented!("parse_package_json_bin_to_hashmap   if bin value is str")
        }
        Bin::Map(map) => map
            .into_iter()
            .filter_map(|(k, v)| {
                if let Some(bin_path_buf) = raw_workspace.join(v).canonicalize().ok() {
                    Some((k.to_string(), bin_path_buf))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, PathBuf>>(),
    }
}

fn parse_package_manager(raw_package_manager: &str) -> Option<PackageManager> {
    let regex_str = r"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";
    let regex = Regex::new(regex_str).unwrap();

    regex.captures(raw_package_manager).map(|caps| {
        return PackageManager {
            raw: raw_package_manager.to_string(),
            name: caps.name("name").map(|x| x.as_str().to_string()),
            version: caps.name("version").map(|x| x.as_str().to_string()),
            hash: Some(PackageManagerDownloadHash {
                value: caps.name("hash_value").map(|x| x.as_str().to_string()),
                name: caps.name("hash_method").map(|x| x.as_str().to_string()),
            }),
        };
    })
}
