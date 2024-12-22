use std::env::current_dir;

e2e::assert_snapshot! {
  #[tokio::test]
  test_show_npm_version_when_missing_default_node_and_npm,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![],
  commands: [
    "npm -v",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_show_npm_version_when_default_npm_missing_but_node_exists,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![],
  commands: [
    "snm node install 20.0.0",
    "snm node default 20.0.0",
    "npm -v",
  ]
}
