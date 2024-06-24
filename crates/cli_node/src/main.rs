use snm_shim::load_node_shim;
use snm_utils::snm_error::friendly_error_message;

#[tokio::main]
async fn main() {
    if let Err(err) = load_node_shim("node").await {
        friendly_error_message(err);
    }
}
