use std::env;

use anyhow::bail;
use node_shim::node;
use package_manager_shim::package_manager;
mod node_shim;
mod package_manager_shim;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let bin_name = args[0].clone();

    if bin_name == "npm" {
        package_manager("npm", "npm").await?;
    } else if bin_name == "npx" {
        package_manager("npm", "npx").await?;
    } else if bin_name == "pnpm" {
        package_manager("pnpm", "pnpm").await?;
    } else if bin_name == "pnpx" {
        package_manager("pnpm", "pnpx").await?;
    } else if bin_name == "yarn" {
        package_manager("yarn", "yarn").await?;
    } else if bin_name == "node" {
        node("node").await?;
    } else {
        bail!("Unknown command: {}", bin_name);
    }

    Ok(())
}
