use std::sync::Arc;

use tokio::sync::OnceCell;
use wiremock::MockServer;

use crate::http_mocker::HttpMocker;

static GLOBAL_MOCK_SERVER: OnceCell<Arc<MockServer>> = OnceCell::const_new();

pub async fn get_global_mock_server() -> Arc<MockServer> {
  GLOBAL_MOCK_SERVER
    .get_or_init(|| async {
      println!("\nInitializing global mock server...");
      let mock_server = HttpMocker::builder().unwrap().build().await.unwrap();
      Arc::new(mock_server)
    })
    .await
    .clone()
}

// 可选：添加清理函数
pub async fn cleanup() {
  if let Some(_server) = GLOBAL_MOCK_SERVER.get() {
    println!("\nCleaning up global mock server...");
    // 如果需要清理逻辑
  }
}
