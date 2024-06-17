use std::env::current_dir;

use snm_node_version::parse_node_version;

#[test]
fn test_parse_no_node_version_file() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = current_dir().unwrap();
    let node_version = parse_node_version(
        &workspace
            .join("tests")
            .join("features")
            .join("no_node_version_file"),
    )?;
    assert!(node_version.is_none());
    Ok(())
}

#[test]
fn test_parse_no_node_version_content() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = current_dir().unwrap();
    if let Some(node_version) =
        parse_node_version(&workspace.join("tests").join("features").join("no_content"))?
    {
        assert_eq!(node_version.get_version().is_none(), true);
    }
    Ok(())
}

#[test]
fn test_parse_node_version_start_with_v() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = current_dir().unwrap();

    if let Some(node_version) = parse_node_version(
        &workspace
            .join("tests")
            .join("features")
            .join("node_version_start_width_v"),
    )? {
        assert_eq!(node_version.get_version(), Some("20.0.1".to_string()));
    }
    Ok(())
}
