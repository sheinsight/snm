use snm_shim::load_package_manage_shim;
use snm_utils::snm_error::friendly_error_message;

#[tokio::main]
async fn main() {
    if let Err(err) = load_package_manage_shim("yarn", "yarn").await {
        friendly_error_message(err);
    }
}
