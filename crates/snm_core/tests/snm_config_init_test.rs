use std::env;

use snm_core::config::SnmConfig;

#[test]
fn no_env_init_test() {
    let snm_config = SnmConfig::new();
    snm_config.init().expect("snm_config_init_success");
    assert_eq!(env::var("SNM_STRICT"), Ok("false".to_string()));

    let home_dir = dirs::home_dir().expect("get home dir error.");
    let base_dir = home_dir.join(".snm");
    assert_eq!(snm_config.get_base_dir_path_buf(), base_dir);
    assert_eq!(snm_config.get_node_bin_dir_path_buf(), base_dir.join("node_bin"));
    assert_eq!(snm_config.get_download_dir_path_buf(), base_dir.join("download"));
    assert_eq!(snm_config.get_node_modules_dir_path_buf(), base_dir.join("node_modules"));

    assert!(snm_config.get_base_dir_path_buf().exists());
    assert!(snm_config.get_node_bin_dir_path_buf().exists());
    assert!(snm_config.get_download_dir_path_buf().exists());
    assert!(snm_config.get_node_modules_dir_path_buf().exists());
}
