mod shim;

use crate::shim::launch_shim;
use snm_npm::snm_npm::SnmNpm;

#[tokio::main]
async fn main() {
    env_logger::init();

    launch_shim(Box::new(SnmNpm::new())).await;
}
