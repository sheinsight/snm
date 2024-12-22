use std::env::current_dir;

use e2e::SnmEnv;

e2e::assert_snapshot! {
  #[tokio::test]
  test_auto_install_node_with_node_version,
  cwd: current_dir()?.join("tests").join("fixtures").join("auto_install_node_with_node_version"),
  envs: vec![],
  commands: [
    "node -v",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_show_node_version_with_strict_mode,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![SnmEnv::Strict("true".to_string())],
  commands: [
    "node -v",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_no_strict_and_no_default_node,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![SnmEnv::Strict("false".to_string())],
  commands: [
    "node -v",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_with_strict_mode_and_has_default_node,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![SnmEnv::Strict("true".to_string())],
  commands: [
    "snm node install 20.0.0",
    "snm node list --compact",
    "snm node default 20.0.0",
    "snm node list --compact",
    "node -v",
  ]
}
