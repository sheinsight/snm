use std::env::{self, current_dir};

use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::consts::SNM_PREFIX;

#[test]
fn test_parse_no_node_version_file() -> Result<(), Box<dyn std::error::Error>> {
  env::set_var("SNM_NODE_BIN_DIR", "node_bin_demo");
  let workspace = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("no_node_version_file");
  let snm_config = SnmConfig::from(SNM_PREFIX, &workspace)?;
  let node_version_reader = SNode::try_from(&snm_config);
  assert!(node_version_reader.is_err());
  Ok(())
}

#[test]
fn test_parse_no_node_version_content() -> Result<(), Box<dyn std::error::Error>> {
  env::set_var("SNM_STRICT", "true");
  let workspace = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("no_content");

  let snm_config = SnmConfig::from(SNM_PREFIX, &workspace)?;
  let node_version_reader = SNode::try_from(&snm_config);

  assert!(node_version_reader.is_err());

  Ok(())
}

#[test]
fn test_parse_node_version_start_with_v() -> Result<(), Box<dyn std::error::Error>> {
  let workspace = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("node_version_start_width_v");

  let snm_config = SnmConfig::from(SNM_PREFIX, &workspace)?;
  let node_version_reader = SNode::try_from(&snm_config)?;

  assert_eq!(node_version_reader.version, Some("20.0.1".to_string()));

  Ok(())
}
