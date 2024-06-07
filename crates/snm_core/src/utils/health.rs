use std::path::PathBuf;

pub static LOCK_FILE_VEC: [&'static str; 3] = ["package-lock.json", "pnpm-lock.yaml", "yarn.lock"];

pub fn check_multi_lock_file(workspace: PathBuf) -> Vec<String> {
    let exists_vec = LOCK_FILE_VEC
        .iter()
        .flat_map(|item| {
            let file_path = workspace.join(item);
            if file_path.exists() {
                Some(item.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    if exists_vec.len() > 1 {
        let msg = format!(
            "Multiple package manager lock files found: {} , Please remove the unnecessary ones.",
            exists_vec.join(", ")
        );
        panic!("{msg}");
    }

    exists_vec
}
