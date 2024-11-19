use std::env::current_dir;

use anyhow::Context;
use snm_config::SnmConfig;
use snm_node_version::SNode;
use snm_utils::exec::exec_cli;

pub async fn node(bin_name: &str) -> anyhow::Result<()> {
    let cwd = current_dir()?;

    let config = SnmConfig::from(&cwd)?;

    let bin_args: Vec<String> = std::env::args().skip(1).collect();

    let node_version_reader =
        SNode::try_from(&config).with_context(|| "Failed to determine Node.js version")?;

    let node_bin_dir = node_version_reader.get_bin().await?;

    exec_cli(vec![node_bin_dir], bin_name, &bin_args)?;

    Ok(())
}
