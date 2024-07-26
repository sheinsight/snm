use std::{
    env::{current_dir, var},
    error::Error,
    process::{Command, Output, Stdio},
};

use tempfile::tempdir;

fn exec(shell: &str, envs: &Vec<(&str, String)>) -> Result<Output, Box<dyn Error>> {
    let shell_vec = shell
        .split(" ")
        .map(|item| item.trim())
        .collect::<Vec<&str>>();

    let x = shell_vec.split_first();

    if let Some((bin_name, args)) = shell_vec.split_first() {
        let output = Command::new(bin_name)
            .envs(envs.clone())
            .args(args)
            // .stdout(Stdio::inherit())
            // .stderr(Stdio::inherit())
            // .stdin(Stdio::inherit())
            .output()?;
        Ok(output)
    } else {
        Err("Invalid shell command".into())
    }
}

#[test]
fn test_install_node() -> Result<(), Box<dyn Error>> {
    let node_version = "16.0.0";

    let dir = tempdir()?.path().to_path_buf();

    let c: std::path::PathBuf = current_dir()?;

    println!("Current dir: {:?}", c);

    let original_path = var("PATH")?;

    let new_path: String = format!("{}:{}", c.display().to_string(), original_path);

    let envs = vec![
        ("PATH", new_path),
        ("SNM_HOME_DIR", dir.display().to_string()),
    ];

    exec(format!("snm node install {}", node_version).as_str(), &envs)?;

    let res = exec("snm node list", &envs)?;

    let x = String::from_utf8(res.stdout)?;

    assert!(x.contains(node_version));

    Ok(())
}
