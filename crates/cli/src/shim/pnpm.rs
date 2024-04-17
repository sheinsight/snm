use crate::shim::launch_shim;
use shim::check;
use snm_core::model::snm_error::handle_snm_error;
use snm_pnpm::snm_pnpm::SnmPnpm;

mod shim;

#[tokio::main]
async fn main() {
    env_logger::init();

    match check("pnpm") {
        Ok(_) => {
            launch_shim(Box::new(SnmPnpm::new())).await;
        }
        Err(error) => handle_snm_error(error),
    }
}
