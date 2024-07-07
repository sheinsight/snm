use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_auto_install_node() -> Result<(), Box<dyn Error>> {
    let node_version = "20.12.1";
    Command::cargo_bin("snm")?
        .env("SNM_NODE_INSTALL_STRATEGY", "auto")
        .arg("node")
        .arg("install")
        .arg(node_version)
        .assert()
        .success();

    Command::cargo_bin("snm")?
        .arg("node")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(node_version));

    Ok(())
}

#[test]
fn test_delete_node() -> Result<(), Box<dyn Error>> {
    let node_version = "20.0.0";
    Command::cargo_bin("snm")?
        .env("SNM_NODE_INSTALL_STRATEGY", "auto")
        .arg("node")
        .arg("install")
        .arg(node_version)
        .assert()
        .success();

    Command::cargo_bin("snm")?
        .arg("node")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(node_version));

    Command::cargo_bin("snm")?
        .arg("node")
        .arg("uninstall")
        .arg(node_version)
        .assert()
        .success();

    Command::cargo_bin("snm")?
        .arg("node")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(node_version).not());

    Ok(())
}

#[test]
fn test_set_default_node() -> Result<(), Box<dyn Error>> {
    let node_version = "20.12.2";
    Command::cargo_bin("snm")?
        .env("SNM_NODE_INSTALL_STRATEGY", "auto")
        .arg("node")
        .arg("install")
        .arg(node_version)
        .assert()
        .success();

    Command::cargo_bin("snm")?
        .env("SNM_NODE_INSTALL_STRATEGY", "auto")
        .arg("node")
        .arg("default")
        .arg(node_version)
        .assert()
        .success();

    Command::cargo_bin("node")?
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains(node_version));

    Ok(())
}
