use snm_shim::package_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    package_manager("pnpm", "pnpm").await?;
    Ok(())
}
