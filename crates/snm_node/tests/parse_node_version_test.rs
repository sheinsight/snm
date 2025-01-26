use std::env::{self, current_dir};

use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_utils::consts::SNM_PREFIX;

#[test]
fn should_fail_when_node_version_file_not_exists() -> Result<(), Box<dyn std::error::Error>> {
  let prefix = uuid::Uuid::new_v4().to_string();
  env::set_var(format!("{}_STRICT", prefix), "true");
  let workspace = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("no_node_version_file");
  let snm_config = SnmConfig::from(&prefix, &workspace)?;
  let node_version_reader = SNode::try_from(&snm_config);
  assert!(node_version_reader.is_err());
  Ok(())
}

#[test]
fn should_fail_when_node_version_file_is_empty() -> Result<(), Box<dyn std::error::Error>> {
  let prefix = uuid::Uuid::new_v4().to_string();

  env::set_var(format!("{}_STRICT", prefix), "true");
  let workspace = current_dir()
    .unwrap()
    .join("tests")
    .join("features")
    .join("no_content");

  let snm_config = SnmConfig::from(&prefix, &workspace)?;
  let node_version_reader = SNode::try_from(&snm_config);

  assert!(node_version_reader.is_err());

  env::remove_var(format!("{}_STRICT", prefix));

  Ok(())
}

#[test]
fn should_parse_version_when_starts_with_v_prefix() -> Result<(), Box<dyn std::error::Error>> {
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
