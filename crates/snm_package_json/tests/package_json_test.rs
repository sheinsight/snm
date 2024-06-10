use std::env::current_dir;

use snm_package_json::parse_package_json;

#[test]
fn test_parse_for_bin() {
    let workspace_path_buf = current_dir().unwrap();

    let package_json = parse_package_json(
        workspace_path_buf
            .join("tests")
            .join("features")
            .join("has_bin"),
    );

    assert_eq!(package_json.bin.get("npm").is_some(), true);

    assert_eq!(package_json.bin.get("npx").is_some(), true);

    assert_eq!(package_json.bin.get("hello").is_some(), false);
}

#[test]
fn test_parse_package_manager() {
    let workspace_path_buf = current_dir().unwrap();

    let package_json = parse_package_json(
        workspace_path_buf
            .join("tests")
            .join("features")
            .join("package_manager"),
    );

    assert_eq!(package_json.package_manager.is_some(), true);

    if let Some(package_manager) = package_json.package_manager {
        assert_eq!(package_manager.name.unwrap(), "pnpm".to_string());
    }
}
