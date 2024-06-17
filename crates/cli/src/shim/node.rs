mod shim;
use colored::*;
use shim::load_node_shim;

const BIN_NAME: &str = "node";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_node_shim("node").await?;
    Ok(())
}
