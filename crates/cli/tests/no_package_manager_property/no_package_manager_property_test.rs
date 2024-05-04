use std::env::{current_dir, set_current_dir};

use cli::execute_cli::get_bin;
use snm_core::model::SnmError;

#[tokio::test]
async fn no_package_manager_property_test() {
    let c_dir = current_dir().expect("get current dir error");
    let test_dir = c_dir.join("tests").join("no_package_manager_property");
    let test_dir_tmp = &test_dir.as_path().display().to_string();
    println!("{}", test_dir_tmp);

    set_current_dir(&test_dir).expect("set current dir error");

    let package_json = test_dir
        .join("package.json")
        .as_path()
        .display()
        .to_string();

    let t = get_bin().await;
    assert_eq!(t.is_err(), true);
    if let Err(n) = t {
        match n {
            SnmError::NotFoundPackageManagerProperty { file_path } => {
                assert_eq!(file_path, package_json);
            }
            _ => panic!("expect SnmError::NotFoundPackageManagerProperty"),
        }
    }
}
