mod shim;

use std::{
    ops::Not,
    process::{Command, Stdio},
};

use shim::{exec_default, node_exec_strict};
use snm_config::parse_snm_config;
use snm_core::traits::manage::ManageTrait;
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_node_version::parse_node_version;
use snm_utils::exec::exec_cli;
const BIN_NAME: &str = "node";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: Box<dyn ManageTrait> = Box::new(SnmNode::new(snm_config.clone()));

    let v = parse_node_version(&snm_config.get_workspace()?)
        .ok()
        .and_then(|node_version| node_version.map(|nv| nv.get_version()))
        .flatten();

    if let Ok(x) = node_exec_strict(&snm_node, snm_config.clone(), BIN_NAME, v).await {
        println!("严格模式 {:?}", x);
        let args: Vec<String> = std::env::args().skip(1).collect();

        exec_cli(x, args);

        return Ok(());
    }

    if snm_config.get_strict().not() {
        if let Ok(sss) = exec_default(&snm_node, BIN_NAME).await {
            println!("默认版本 {:?}", sss);
        }
    }

    Ok(())
}
