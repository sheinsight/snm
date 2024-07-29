use std::{
    env::current_dir,
    error::Error,
    fs,
    path::PathBuf,
    process::{Command, Output},
};

use e2e::exec_builder::ExecBuilder;
use tempfile::tempdir;

// #[cfg(windows)]
// const SNM_CMD: &str = "snm.exe";
// #[cfg(not(windows))]
// const SNM_CMD: &str = "snm";

const SNM_CMD: &str = "snm";

fn exec(
    shell: &str,
    current: &PathBuf,
    envs: &Vec<(&str, String)>,
) -> Result<String, Box<dyn Error>> {
    let shell_vec = shell
        .split(" ")
        .map(|item| item.trim())
        .collect::<Vec<&str>>();

    if let Some((bin_name, args)) = shell_vec.split_first() {
        let output = Command::new(bin_name)
            .envs(envs.clone())
            .args(args)
            .current_dir(current)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        println!(
            r##"
Exec shell: {}
Stdout: {}
Stderr: {}
        "##,
            shell, stdout, stderr
        );
        Ok(stdout)
    } else {
        Err("Invalid shell command".into())
    }
}

#[test]
fn should_auto_install() -> Result<(), Box<dyn Error>> {
    let node_version = "16.0.0";

    let dir = tempdir()?.path().to_path_buf();

    let current_dir_path_buf = current_dir()?;

    let envs = vec![
        ("SNM_HOME_DIR".to_string(), dir.display().to_string()),
        ("SNM_NODE_INSTALL_STRATEGY".to_string(), "auto".to_string()),
    ];

    let executor = ExecBuilder::builder()
        .current(&current_dir_path_buf)
        .envs(envs)
        .build();

    executor.exec(&format!("{} node install {}", SNM_CMD, node_version))?;

    let stdout = executor.exec(&format!("{} node list", SNM_CMD))?;

    assert!(
        stdout.contains(node_version),
        "Expected to find node version {} in stdout, but got: {}",
        node_version,
        stdout
    );

    Ok(())
}

#[test]
fn should_auto_install_and_exec() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?.path().to_path_buf();

    let cwd = current_dir()?
        .join("tests")
        .join("features")
        .join("node-proxy");

    let node_version = fs::read_to_string(cwd.join(".node-version"))?;

    let envs = vec![
        ("SNM_HOME_DIR".to_string(), dir.display().to_string()),
        ("SNM_NODE_INSTALL_STRATEGY".to_string(), "auto".to_string()),
    ];

    let executor = ExecBuilder::builder().current(&cwd).envs(envs).build();

    let stdout = executor.exec("node -v")?;

    assert!(
        stdout.contains(node_version.as_str()),
        "Expected to find node version {} in stdout, but got: {}",
        node_version,
        stdout
    );

    Ok(())
}
