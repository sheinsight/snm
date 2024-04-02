use super::read_to_json;
use crate::model::SnmError;
use crate::model::{Bin, PackageJson};
use regex::{Captures, Regex};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

pub async fn parse_package_json_bin_to_hashmap(
    package_json_abs_path: &PathBuf,
) -> Result<HashMap<String, String>, SnmError> {
    let package_json = read_to_json::<PackageJson>(&package_json_abs_path)?;

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

// pub fn automatic_version_parsed(
//     workspace: Option<PathBuf>,
// ) -> Result<PackageManagerModel, SnmError> {
//     let wk = match workspace {
//         Some(workspace) => workspace,
//         None => std::env::current_dir()?,
//     };
//     let pkg_file_path = wk.join("package.json");

//     let pkg = read_to_json::<PackageJsonModel>(&pkg_file_path)?;

//     return match pkg.package_manager {
//         Some(package_manager) => parse_package_manager_config(&package_manager),
//         None => Err(SnmError::NoPackageManagerError {
//             file_path: pkg_file_path.display().to_string(),
//         }),
//     };
// }
