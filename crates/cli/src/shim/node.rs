mod shim;

use crate::shim::launch_shim;
use snm_config::parse_snm_config;
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
const BIN_NAME: &str = "node";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    launch_shim(
        Box::new(SnmNode::new(snm_config.clone())),
        BIN_NAME,
        snm_config.get_strict(),
    )
    .await;

    Ok(())
}
