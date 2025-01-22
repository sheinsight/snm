use std::env::current_dir;

use e2e::SnmEnv;

// e2e::test1! {
//   #[tokio::test]
//   test_auto_install_node_with_node_version,
//   cwd: current_dir()?.join("tests").join("fixtures").join("auto_install_node_with_node_version"),
//   envs: [],
//   |builder:e2e::CommandBuilder| => {
//     builder.add_snapshot("node -v")?;
//     builder.assert_snapshots(|name,res| {
//       insta::assert_snapshot!(name, res);
//     })?;
//   }
// }

// e2e::test1! {
//   #[tokio::test]
//   test_show_node_version_with_strict_mode,
//   cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
//   envs: [SnmEnv::Strict("true".to_string())],
//   |builder:e2e::CommandBuilder| => {
//     builder.add_snapshot("node -v")?;
//     builder.assert_snapshots(|name,res| {
//       insta::assert_snapshot!(name, res);
//     })?;
//   }
// }

// e2e::test1! {
//   #[tokio::test]
//   test_no_strict_and_no_default_node,
//   cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
//   envs: [SnmEnv::Strict("false".to_string())],
//   |builder:e2e::CommandBuilder| => {
//     builder.add_snapshot("node -v")?;
//     builder.assert_snapshots(|name,res| {
//       insta::assert_snapshot!(name, res);
//     })?;
//   }
// }

e2e::test1! {
  #[tokio::test(flavor = "current_thread")]
  test_with_strict_mode_and_has_default_node,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: [
    SnmEnv::Strict("true".to_string()),
    SnmEnv::Log("snm=trace".to_string()),
  ],
  |builder:e2e::CommandBuilder| => {
    builder.add_snapshot("snm node install 20.0.0")?;
    builder.add_snapshot("snm node list --compact")?;
    builder.add_snapshot("snm node default 20.0.0")?;
    builder.add_snapshot("snm node list --compact")?;
    builder.add_snapshot("node -v")?;
    builder.assert_snapshots(|name,res| {
      insta::assert_snapshot!(name, res);
    })?;
  }
}

// #[test]
// fn hello() {
//   let mock_server = e2e::get_global_mock_server().await;
//   println!("mock_server---->: {:?}", mock_server);
// }
