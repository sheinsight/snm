// use snm_config::parse_snm_config;
// use snm_current_dir::current_dir;

// #[test]
// fn test_init_without_env() {
//     let current = current_dir().unwrap();

//     let snm_config = parse_snm_config(&current).unwrap();

//     let home_dir = dirs::home_dir().expect("get home dir error.");
//     let base_dir = home_dir.join(".snm");
//     assert_eq!(
//         snm_config.get_node_bin_dir().unwrap(),
//         base_dir.join("node_bin")
//     );
//     assert_eq!(
//         snm_config.get_download_dir().unwrap(),
//         base_dir.join("downloads")
//     );
//     assert_eq!(
//         snm_config.get_node_modules_dir().unwrap(),
//         base_dir.join("node_modules")
//     );
//     assert!(snm_config.get_node_bin_dir().unwrap().exists());
//     assert!(snm_config.get_download_dir().unwrap().exists());
//     assert!(snm_config.get_node_modules_dir().unwrap().exists());
// }
