use snm_shim::load_node_shim;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_node_shim("node").await?;
    Ok(())
}
