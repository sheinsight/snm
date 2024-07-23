use snm_shim::package_manager;
use snm_utils::snm_error::friendly_error_message;

#[tokio::main]
async fn main() {
    if let Err(err) = package_manager("yarn", "yarn").await {
        friendly_error_message(err);
    }
}
