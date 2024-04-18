use crate::shim::launch_shim;
use shim::check;
use snm_core::model::snm_error::handle_snm_error;
use snm_pnpm::snm_pnpm::SnmPnpm;

mod shim;
const BIN_NAME: &str = "pnpm";
#[tokio::main]
async fn main() {
    env_logger::init();

    match check("pnpm") {
        Ok(_) => {
            launch_shim(Box::new(SnmPnpm::new()), BIN_NAME).await;
        }
        Err(error) => handle_snm_error(error),
    }
}
