use e2e::SnmEnv;
use std::env::current_dir;

e2e::assert_snapshot! {
  #[tokio::test]
  test_snm_install_node,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![],
  commands: [
    "snm node install 20.0.0",
    "snm node list --compact",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_snm_uninstall_node,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![],
  commands: [
    "snm node install 20.0.0",
    "snm node list --compact",
    "snm node uninstall 20.0.0",
    "snm node list --compact",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_snm_set_default_node,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![],
  commands: [
    "snm node install 20.0.0",
    "snm node default 20.0.0",
    "node -v",
  ]
}

e2e::assert_snapshot! {
  #[tokio::test]
  test_snm_list,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: vec![],
  commands: [
    "snm node install 20.0.0",
    "snm node list",
    "snm node default 20.0.0",
    "snm node list",
    "snm node list --compact",
    "snm node list --remote",
  ]
}

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_list_with_strict_mode,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
    envs: vec![SnmEnv::Strict("true".to_string())],
    commands: [
        "snm node install 20.0.0",
        "snm node list",
        "snm node default 20.0.0",
        "snm node list",
        "snm node list --compact",
        "snm node list --remote",
        "node -v",
    ]
}
