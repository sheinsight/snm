use crate::shim::launch_shim;
use snm_pnpm::snm_pnpm::SnmPnpm;

mod shim;

#[tokio::main]
async fn main() {
    env_logger::init();

    launch_shim(Box::new(SnmPnpm::new())).await;
}
