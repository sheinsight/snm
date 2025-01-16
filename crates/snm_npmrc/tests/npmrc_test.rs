use std::env::{self, current_dir};

use snm_npmrc::NpmrcReader;

#[test]
fn test() {
  let c = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("project");

  env::set_var(
    "PREFIX",
    current_dir()
      .unwrap()
      .join("tests")
      .join("features")
      .join("global")
      .display()
      .to_string(),
  );

  let npmrc = NpmrcReader::from(&c);

  let registry = npmrc.read_registry_with_default();
  assert_eq!(registry, "https://project.com".to_string());

  let cache = npmrc.read("cache");
  #[cfg(target_os = "windows")]
  assert_eq!(cache, None);

  #[cfg(not(target_os = "windows"))]
  assert_eq!(cache, Some("~/.hello".to_string()));
}
