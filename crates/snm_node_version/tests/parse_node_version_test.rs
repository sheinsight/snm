use std::env::current_dir;

use snm_node_version::NodeVersionReader;

#[test]
fn test_parse_no_node_version_file() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = current_dir()
        .unwrap()
        .join("tests")
        .join("features")
        .join("no_node_version_file");
    let node_version_reader = NodeVersionReader::from(&workspace);
    let node_version = node_version_reader.read_version();
    assert!(node_version.is_none());
    Ok(())
}

#[test]
fn test_parse_no_node_version_content() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = current_dir()
        .unwrap()
        .join("tests")
        .join("features")
        .join("no_content");

    let node_version_reader = NodeVersionReader::from(&workspace);

    let node_version = node_version_reader.read_version();

    assert!(node_version.is_none());

    Ok(())
}

#[test]
fn test_parse_node_version_start_with_v() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = current_dir()
        .unwrap()
        .join("tests")
        .join("features")
        .join("node_version_start_width_v");

    let node_version_reader = NodeVersionReader::from(&workspace);

    let node_version = node_version_reader.read_version();

    assert_eq!(node_version, Some("20.0.1".to_string()));

    Ok(())
}
