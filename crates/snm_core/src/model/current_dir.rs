use std::env::current_dir;
use std::path::PathBuf;

pub fn truncate_node_modules_path(path: PathBuf) -> String {
    let path_str = path.to_str().expect("msg");
    if let Some(pos) = path_str.find("node_modules") {
        path_str[..(pos - 1)].to_string()
    } else {
        path_str.to_string()
    }
}

pub fn cwd() -> PathBuf {
    return current_dir()
        .and_then(|item| Ok(truncate_node_modules_path(item)))
        .and_then(|item| Ok(PathBuf::from(item)))
        .expect("xx");
}
