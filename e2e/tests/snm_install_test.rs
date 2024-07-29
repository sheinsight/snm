use std::{
    env::current_dir,
    error::Error,
    process::{Command, Output},
};

use tempfile::tempdir;

fn exec(shell: &str, envs: &Vec<(&str, String)>) -> Result<Output, Box<dyn Error>> {
    let shell_vec = shell
        .split(" ")
        .map(|item| item.trim())
        .collect::<Vec<&str>>();

    if let Some((bin_name, args)) = shell_vec.split_first() {
        let output = Command::new(bin_name)
            .envs(envs.clone())
            .args(args)
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

    let path_dir = current_dir()?.join("tests");

    println!("Current dir: {:?}", path_dir);

    let envs = vec![
        ("PATH", path_dir.display().to_string()),
        ("SNM_HOME_DIR", dir.display().to_string()),
        ("SNM_NODE_INSTALL_STRATEGY", "auto".to_string()),
    ];

    let install_output = exec(&format!("snm node install {}", node_version), &envs)?;
    println!(
        "Install stdout: {}",
        String::from_utf8_lossy(&install_output.stdout)
    );
    println!(
        "Install stderr: {}",
        String::from_utf8_lossy(&install_output.stderr)
    );

    let list_output = exec("snm node list", &envs)?;
    let stdout = String::from_utf8(list_output.stdout)?;
    println!("List stdout: {}", &stdout);
    println!(
        "List stderr: {}",
        String::from_utf8_lossy(&list_output.stderr)
    );
    assert!(
        stdout.contains(node_version),
        "Expected to find node version {} in stdout, but got: {}",
        node_version,
        stdout
    );

    Ok(())
}
