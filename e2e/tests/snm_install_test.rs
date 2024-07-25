use std::{
    env::{current_dir, var},
    error::Error,
    process::{Command, Output, Stdio},
};

use tempfile::tempdir;

fn run_command(args: &[&str], envs: &Vec<(&str, String)>) -> Result<Output, Box<dyn Error>> {
    let output = Command::new("snm")
        .envs(envs.clone())
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()?;
    Ok(output)
}

#[test]
fn test_parse_no_node_version_file() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?.path().to_path_buf();

    let c = current_dir()?;

    let original_path = var("PATH")?;

    let new_path: String = format!("{}:{}", c.display().to_string(), original_path);

    println!("临时目录路径: {} {}", dir.display(), c.display());

    let envs = vec![
        ("PATH", new_path),
        ("SNM_HOME_DIR", dir.display().to_string()),
    ];

    let res = run_command(&["node", "install", "16.0.0"], &envs)?;

    println!("res1--->: {:?}", res);

    // 列出已安装的 Node.js 版本
    let res = run_command(&["node", "list"], &envs)?;

    // let x = String::from_utf8(res.stdout)?;

    println!("res2--->: {:?}", res);

    // assert!(x.contains("16.0.0"));

    Ok(())
}
