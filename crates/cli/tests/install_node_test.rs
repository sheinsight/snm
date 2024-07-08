use std::{error::Error, ops::Not};

use assert_cmd::Command;
use predicates::prelude::*;
use rexpect::spawn;

#[test]
fn test_auto_install_node() -> Result<(), Box<dyn Error>> {
    let node_version = "20.12.1";

    let mut p = spawn(
        format!("snm node install {}", node_version).as_str(),
        Some(30_000),
    )?;
    p.send_line("y")?;

    let mut p = spawn("snm node list", Some(30_000))?;

    p.exp_string(&node_version)?;

    Ok(())
}

#[test]
fn test_delete_node() -> Result<(), Box<dyn Error>> {
    let node_version = "20.0.0";

    let mut p = spawn(
        format!("snm node install {}", node_version).as_str(),
        Some(30_000),
    )?;
    p.send_line("y")?;

    p.exp_eof()?;

    let mut p = spawn("snm node list", Some(30_000))?;

    let output = p.exp_eof()?;

    assert_eq!(output.contains(&node_version), true);

    let mut p = spawn(
        format!("snm node uninstall {}", node_version).as_str(),
        Some(30_000),
    )?;

    p.exp_eof()?;

    let mut p = spawn("snm node list", Some(30_000))?;

    let output = p.exp_eof()?;

    assert_eq!(output.contains(&node_version), false);

    Ok(())
}

#[test]
fn test_set_default_node() -> Result<(), Box<dyn Error>> {
    let node_version = "20.12.2";

    let mut p = spawn(
        format!("snm node install {}", node_version).as_str(),
        Some(30_000),
    )?;
    p.send_line("y")?;

    p.exp_eof()?;

    let mut p = spawn(
        format!("snm node default {}", node_version).as_str(),
        Some(30_000),
    )?;

    p.send_line("y")?;

    p.exp_eof()?;

    let mut p = spawn("node -v", Some(30_000))?;

    p.exp_string(node_version)?;

    Ok(())
}
