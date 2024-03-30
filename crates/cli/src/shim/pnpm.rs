use snm_core::model::snm_error::handle_snm_error;

mod shim;

#[tokio::main]
async fn main() {
    match crate::shim::launch("pnpm").await {
        Ok(output) => {
            if !output.status.success() {
                std::process::exit(output.status.code().unwrap_or(-1));
            }
        }
        Err(error) => handle_snm_error(error),
    }
}
