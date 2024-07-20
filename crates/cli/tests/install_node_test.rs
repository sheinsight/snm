use std::{env, error::Error, fs, ops::Not, path::PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use rexpect::spawn;
use uuid::Uuid;

fn create_temp_dir() -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let target_debug_dir = current_dir.join("target").join("debug");
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", target_debug_dir.display(), current_path);
    env::set_var("PATH", new_path);
    let temp_dir = env::temp_dir();
    let unique_dir = temp_dir.join(Uuid::new_v4().to_string());
    fs::create_dir(&unique_dir).expect("Failed to create temp directory");
    println!("Created temp directory: {:?}", unique_dir);
    unique_dir
}

#[test]
fn test_auto_install_node() -> Result<(), Box<dyn Error>> {
    let temp_dir = create_temp_dir().display().to_string();

    env::set_var("SNM_HOME_DIR", temp_dir);

    env::set_var("SNM_NODE_INSTALL_STRATEGY", "ask");

    let node_version = "20.12.0";

    let mut p = spawn(
        format!("snm node install {}", node_version).as_str(),
        Some(600_000),
    )?;

    p.exp_string("Do you want to install")?;

    p.send_line("y")?;

    p.exp_eof()?;

    p = spawn("snm node list", Some(3000_000))?;

    p.exp_string(&node_version)?;

    Ok(())
}

#[test]
fn test_delete_node() -> Result<(), Box<dyn Error>> {
    let temp_dir = create_temp_dir().display().to_string();

    env::set_var("SNM_HOME_DIR", temp_dir);

    env::set_var("SNM_NODE_INSTALL_STRATEGY", "ask");

    let node_version = "20.0.0";

    let mut p = spawn(
        format!("snm node install {}", node_version).as_str(),
        Some(600_000),
    )?;

    p.exp_string("Do you want to install")?;

    p.send_line("y")?;

    p.exp_eof()?;

    p = spawn("snm node list", Some(30_000))?;

    p.exp_string(&node_version)?;

    p = spawn(
        format!("snm node uninstall {}", node_version).as_str(),
        Some(30_000),
    )?;

    p.exp_string("do you want to uninstall")?;

    p.send_line("y")?;

    p.exp_eof()?;

    p = spawn("snm node list", Some(30_000))?;

    let output = p.exp_eof()?;

    assert_eq!(output.contains(&node_version), false);

    Ok(())
}

// #[test]
// fn test_set_default_node() -> Result<(), Box<dyn Error>> {
//     let node_version = "20.12.2";

//     let mut p = spawn(
//         format!("snm node install {}", node_version).as_str(),
//         Some(30_000),
//     )?;
//     p.send_line("y")?;

//     p.exp_eof()?;

//     let mut p = spawn(
//         format!("snm node default {}", node_version).as_str(),
//         Some(30_000),
//     )?;

//     p.send_line("y")?;

//     p.exp_eof()?;

//     let mut p = spawn("node -v", Some(30_000))?;

//     p.exp_string(node_version)?;

//     Ok(())
// }
