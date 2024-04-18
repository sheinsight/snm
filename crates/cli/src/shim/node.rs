mod shim;

use crate::shim::launch_shim;
use snm_node::snm_node::SnmNode;
const BIN_NAME: &str = "node";

#[tokio::main]
async fn main() {
    env_logger::init();

    launch_shim(Box::new(SnmNode::new()), BIN_NAME).await;
}
