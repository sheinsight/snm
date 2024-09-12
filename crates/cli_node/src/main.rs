use snm_shim::node;
use snm_utils::snm_error::friendly_error_message;

#[tokio::main]
async fn main() {
    if let Err(err) = node("node").await {
        friendly_error_message(err);
    }
}
