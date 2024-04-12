mod shim;

use crate::shim::launch_shim;
use snm_node::snm_node::SnmNode;

#[tokio::main]
async fn main() {
    env_logger::init();

    launch_shim(Box::new(SnmNode::new())).await;
}
