use std::env::{current_dir, set_current_dir};

use cli::execute_cli::get_bin;
use snm_core::model::SnmError;

#[tokio::test]
async fn no_package_json_test() {
    let c_dir = current_dir().expect("get current dir error");
    let test_dir = c_dir.join("tests").join("no_package_json");
    set_current_dir(&test_dir).expect("set current dir error");

    let t = get_bin().await;
    assert_eq!(t.is_err(), true);
    if let Err(n) = t {
        match n {
            SnmError::NotFoundDefaultPackageManager { name } => {
                assert_eq!(name, "pnpm".to_string());
            }
            _ => panic!("expect SnmError::NotFoundDefaultPackageManager"),
        }
    }
}
