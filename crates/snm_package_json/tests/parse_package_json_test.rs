use std::env::current_dir;

use snm_package_json::package_manager_raw::PackageJson;

#[test]
fn test_parse_for_bin() {
    let workspace_path_buf = current_dir()
        .unwrap()
        .join("tests")
        .join("features")
        .join("has_bin");

    let package_json = match PackageJson::from(&workspace_path_buf) {
        Some(package_json) => package_json,
        None => unreachable!("Failed to parse package.json"),
    };

    assert_eq!(package_json.get_bin_with_name("npm").is_some(), true);

    assert_eq!(package_json.get_bin_with_name("npm").is_some(), true);

    if let Some(npm) = package_json.get_bin_with_name("npm") {
        assert_eq!(npm, workspace_path_buf.join("bin").join("npm.cjs"));
    }

    if let Some(npx) = package_json.get_bin_with_name("npx") {
        assert_eq!(npx, workspace_path_buf.join("bin").join("npx.cjs"));
    }

    assert_eq!(package_json.get_bin_with_name("hello").is_none(), true);
}

#[test]
fn test_parse_package_manager() {
    let workspace_path_buf = current_dir().unwrap();

    let package_json = match PackageJson::from(
        &workspace_path_buf
            .join("tests")
            .join("features")
            .join("package_manager"),
    ) {
        Some(package_json) => package_json,
        None => unreachable!("Failed to parse package.json"),
    };

    assert_eq!(package_json.get_pm_name().unwrap(), "pnpm".to_string());
}
