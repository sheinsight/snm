mod shim;

use crate::shim::launch_shim;
use snm_npm::snm_npm::SnmNpm;

const BIN_NAME: &str = "npx";

#[tokio::main]
async fn main() {
    env_logger::init();
    launch_shim(Box::new(SnmNpm::new()), BIN_NAME).await;
}
