use std::{env::current_dir, path::PathBuf, process::Command};

use e2e::{exec, SnmEnv};
use tempfile::tempdir;

fn get_debug_dir() -> PathBuf {
    // 获取 e2e 目录 (CARGO_MANIFEST_DIR 指向 e2e/Cargo.toml 所在目录)
    let e2e_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // 向上一级找到项目根目录
    let root_dir = e2e_dir.parent().expect("Failed to get root dir");

    // 进入 target/debug 目录
    root_dir.join("target").join("debug")
}

#[tokio::test]
async fn test_auto_install_node_with_node_version() -> anyhow::Result<()> {
    let res = exec! {
        cwd: ["auto_install_node_with_node_version"],
        command: "node -v",
        env: |dir| => vec![
            SnmEnv::Lang("zh".to_string()),
        ],
    };

    assert!(res.to_string().contains("v20.11.1"));

    Ok(())
}
