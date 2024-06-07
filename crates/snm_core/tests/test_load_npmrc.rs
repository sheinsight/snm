use std::env::{current_dir, set_current_dir};

use snm_core::load_npmrc::load_npmrc;

#[test]
fn test_load() {
    let c_dir = current_dir().expect("get current dir error");
    let test_dir = c_dir.join("tests").join("nested-2").join("nested-1");
    set_current_dir(&test_dir).expect("set current dir error");

    let config = load_npmrc(test_dir);

    assert_eq!(config.get("registry"), Some(&("https://a.com".to_string())));
    assert_eq!(config.get("strict"), Some(&("true".to_string())));
    assert_eq!(config.get("strict_none"), None);
}
