// use std::str::FromStr;

// use package_json_parser::PackageJsonParser;
use snm_config::snm_config::SnmConfig;
// use up_finder::UpFinder;

// use crate::PackageManager;

pub struct PackageManagerSetup {
  pub config: SnmConfig,
}

impl PackageManagerSetup {
  // fn find_up_package_manager(&self) -> anyhow::Result<Option<PackageManager>> {

  //   let find_up = UpFinder::builder()
  //     .cwd(&self.config.workspace) // 从当前目录开始
  //     .build();

  //   let files = find_up.find_up("package.json");

  //   if files.is_empty() {
  //     return Ok(None);
  //   }

  //   let Some(package_manager) = files.iter().find_map(|item| {
  //     let Ok(package_json) = PackageJsonParser::parse(item) else {
  //       return None;
  //     };

  //     let Some(raw) = package_json.package_manager else {
  //       return None;
  //     };

  //     let Some(package_manager) = PackageManager::from_str(&raw.0).ok() else {
  //       return None;
  //     };

  //     Some(package_manager)
  //   }) else {
  //     Ok(None)
  //   }

  //   Ok(None)

  // }
}
