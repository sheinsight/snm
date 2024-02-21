mod shim;

#[tokio::main]
async fn main() {
    crate::shim::launch("node").await;
}
