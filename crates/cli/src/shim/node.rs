mod shim;

use crate::shim::launch_shim;
use snm_node::demo::NodeDemo;

#[tokio::main]
async fn main() {
    env_logger::init();

    launch_shim(Box::new(NodeDemo::new())).await;
}
