mod shim;

use crate::shim::launch_shim;
use shim::check;
use snm_core::model::snm_error::handle_snm_error;
use snm_npm::snm_npm::SnmNpm;

const BIN_NAME: &str = "npx";

#[tokio::main]
async fn main() {
    env_logger::init();

    match check(BIN_NAME) {
        Ok(_) => {
            launch_shim(Box::new(SnmNpm::new()), BIN_NAME).await;
        }
        Err(error) => handle_snm_error(error),
    }
}
