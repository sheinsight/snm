use snm_shim::node;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    node("node").await?;

    Ok(())
}
