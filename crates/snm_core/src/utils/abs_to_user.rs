use std::path::PathBuf;

pub fn transform_abs_to_user(abs_path: &PathBuf) -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home dir");
    let home_dir_str = home_dir.to_str().unwrap();
    let abs_path_str = abs_path.display().to_string();
    abs_path_str.replace(home_dir_str, "~")
}
