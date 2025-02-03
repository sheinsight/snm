use std::env::current_dir;

use snm_pm::package_json::PJson;

#[test]
fn test_parse_for_bin() {
  let workspace_path_buf = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("has_bin");

  let package_json = match PJson::from(&workspace_path_buf) {
    Ok(package_json) => package_json,
    Err(e) => unreachable!("Failed to parse package.json: {}", e),
  };

  assert_eq!(package_json.get_bin_with_name("npm").is_ok(), true);

  assert_eq!(package_json.get_bin_with_name("npm").is_ok(), true);

  if let Ok(npm) = package_json.get_bin_with_name("npm") {
    assert_eq!(npm, workspace_path_buf.join("bin").join("npm.cjs"));
  }

  if let Ok(npx) = package_json.get_bin_with_name("npx") {
    assert_eq!(npx, workspace_path_buf.join("bin").join("npx.cjs"));
  }

  assert_eq!(package_json.get_bin_with_name("hello").is_err(), true);
}

#[test]
fn test_parse_package_manager() {
  let workspace_path_buf = current_dir().unwrap();

  let package_json = match PJson::from(
    &workspace_path_buf
      .join("tests")
      .join("features")
      .join("package_manager"),
  ) {
    Ok(package_json) => package_json,
    Err(e) => unreachable!("Failed to parse package.json: {}", e),
  };

  assert_eq!(
    package_json.package_manager,
    Some("pnpm@10.12.0".to_string())
  );
}
