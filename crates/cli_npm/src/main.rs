use snm_shim::load_package_manage_shim;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_package_manage_shim("npm", "npm").await?;
    Ok(())
}
