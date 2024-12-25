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

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_install_set_default_pnpm,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
    envs: vec![],
    commands: [
        "snm node install 20.0.0",
        "snm node default 20.0.0",
        "snm pnpm install 9.0.0",
        "snm pnpm default 9.0.0",
        "pnpm -v",
    ]
}

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_install_set_default_npm_with_node_20,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
    envs: vec![],
    commands: [
        "snm node install 20.0.0",
        "snm node default 20.0.0",
        "npm -v",
        "snm npm install 9.0.0",
        "snm npm default 9.0.0",
        "npm -v",
    ]
}

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_install_set_default_yarn,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
    envs: vec![],
    commands: [
        "snm node install 20.0.0",
        "snm node default 20.0.0",
        "yarn -v",
        "snm yarn install 1.22.22",
        "snm yarn default 1.22.22",
        "yarn -v",
    ]
}

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_install_set_default_yarn4,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
    envs: vec![],
    commands: [
        "snm node install 20.0.0",
        "snm node default 20.0.0",
        "yarn -v",
        "snm yarn install 4.0.0",
        "snm yarn default 4.0.0",
        "yarn -v",
    ]
}

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_install_with_node_20_npm,
    cwd: current_dir()?.join("tests").join("fixtures").join("snm_i_with_node_npm"),
    envs: vec![],
    commands: [
        "snm node install 20.0.0",
        "snm node default 20.0.0",
        "npm -v",
        "npm install",
        "npm list",
    ]
}

e2e::assert_snapshot! {
    #[tokio::test]
    test_snm_install_with_outside_pnpm,
    cwd: current_dir()?.join("tests").join("fixtures").join("test_snm_install_with_outside_pnpm"),
    envs: vec![],
    commands: [
        "snm node install 20.0.0",
        "snm node default 20.0.0",
        "npm -v",
        "snm npm install 9.0.0",
        "snm npm default 9.0.0",
        "npm -v",
        "npm install",
        "npm list",
    ]
}
