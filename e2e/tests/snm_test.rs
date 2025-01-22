// use std::env::current_dir;

// use e2e::SnmEnv;

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_node,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_uninstall_node,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.add_snapshot("snm node uninstall 20.0.0")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_set_default_node,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("node -v")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_list,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.add_snapshot("snm node list --remote")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_list_with_strict_mode,
//     cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

//     envs:[SnmEnv::Strict("true".to_string())],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("snm node list")?;
//         builder.add_snapshot("snm node list --compact")?;
//         builder.add_snapshot("snm node list --remote")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }

// e2e::test1! {
//     #[tokio::test]
//     test_snm_install_with_node_20_npm,
//     cwd: current_dir()?.join("tests").join("fixtures").join("snm_i_with_node_npm"),

//     envs:[],
//     |builder:e2e::CommandBuilder| => {
//         builder.add_snapshot("snm node install 20.0.0")?;
//         builder.add_snapshot("snm node default 20.0.0")?;
//         builder.add_snapshot("npm -v")?;
//         builder.exec("npm install")?;
//         builder.add_snapshot("node index.cjs")?;
//         builder.assert_snapshots(|name,res| {
//             insta::assert_snapshot!(name, res);
//         })?;
//     }
// }
