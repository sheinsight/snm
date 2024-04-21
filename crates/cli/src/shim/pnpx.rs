mod shim;

use crate::shim::launch_shim;
use snm_pnpm::snm_pnpm::SnmPnpm;

const BIN_NAME: &str = "pnpx";

#[tokio::main]
async fn main() {
    env_logger::init();
    launch_shim(Box::new(SnmPnpm::new()), BIN_NAME).await;
}
