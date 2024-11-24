use std::env::{self, current_dir};

use anyhow::{bail, Context};
use colored::Colorize;
use snm_config::SnmConfig;
use snm_node_version::SNode;
use snm_package_json::pm::PackageManager;
use snm_utils::exec::exec_cli;

pub async fn package_manager(prefix: &str, bin_name: &str) -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!(
        "args: {:?} {}",
        &args,
        args.iter().find(|item| *item == "-g").is_some()
    );
    let command = args.get(1).context("command not found")?;

    let cwd = current_dir()?;
    let snm_config = SnmConfig::from(&cwd)?;

    let pm_bin_dir = {
        let pm = PackageManager::try_from_env(&snm_config).ok();

        let path = match pm {
            Some(pm) => {
                if &prefix == &pm.name() {
                    pm.get_bin(pm.version(), prefix).await?
                } else if snm_config.restricted_list.contains(&command.to_string()) {
                    bail!(
                        "Package manager mismatch, expect: {}, actual: {} . Restricted list: {}",
                        pm.library_name().green(),
                        prefix.red(),
                        snm_config.restricted_list.join(", ").black(),
                    );
                } else {
                    String::new()
                }
            }
            None => {
                if snm_config.strict {
                    bail!("Failed to determine package manager")
                } else {
                    String::new()
                }
            }
        };

        vec![path]
    };

    let node_bin_dir = {
        let node_version_reader =
            SNode::try_from(&snm_config).with_context(|| "Failed to determine Node.js version")?;

        let bin_dir = node_version_reader.get_bin().await?;
        vec![bin_dir]
    };

    let mut bin_dir = pm_bin_dir;
    bin_dir.extend(node_bin_dir);

    exec_cli(bin_dir, bin_name, &args[1..])?;

    Ok(())
}
