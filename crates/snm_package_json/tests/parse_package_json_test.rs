use std::env::current_dir;

use snm_package_json::parse_package_json;

#[test]
fn test_parse_for_bin() {
    let workspace_path_buf = current_dir()
        .unwrap()
        .join("tests")
        .join("features")
        .join("has_bin");

    let package_json = match parse_package_json(&workspace_path_buf).unwrap() {
        Some(package_json) => package_json,
        None => unreachable!("Failed to parse package.json"),
    };

    assert_eq!(package_json.bin.get("npm").is_some(), true);

    assert_eq!(package_json.bin.get("npx").is_some(), true);

    if let Some(npm) = package_json.bin.get("npm").as_deref().cloned() {
        assert_eq!(npm, workspace_path_buf.join("bin").join("npm.cjs"));
    }

    if let Some(npx) = package_json.bin.get("npx").as_deref().cloned() {
        assert_eq!(npx, workspace_path_buf.join("bin").join("npx.cjs"));
    }

    assert_eq!(package_json.bin.get("hello").is_some(), false);
}

#[test]
fn test_parse_package_manager() {
    let workspace_path_buf = current_dir().unwrap();

    let package_json = match parse_package_json(
        &workspace_path_buf
            .join("tests")
            .join("features")
            .join("package_manager"),
    )
    .unwrap()
    {
        Some(package_json) => package_json,
        None => unreachable!("Failed to parse package.json"),
    };

    let package_manager = match package_json.package_manager {
        Some(package_manager) => package_manager,
        None => unreachable!("Failed to parse package manager"),
    };

    let name = package_manager.name;

    assert_eq!(name, "pnpm".to_string());
}
