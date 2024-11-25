use std::env;

use anyhow::{bail, Context};
use node_shim::node;
use package_manager_shim::package_manager;
mod node_shim;
mod package_manager_shim;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let actual_bin_name = args.get(0).context("bin name not found")?;

    const PM: [&str; 5] = ["npm", "npx", "pnpm", "pnpx", "yarn"];

    if PM.contains(&actual_bin_name.as_str()) {
        package_manager().await?;
    } else if actual_bin_name == "node" {
        node().await?;
    } else {
        bail!("Unknown command: {}", actual_bin_name);
    }

    Ok(())
}
