use std::env::current_dir;

use snm_test_utils::{ResponseSource, SnmMockServerArg, SnmTestContext};
use wiremock::MockServer;

pub async fn setup_http_server(ctx: &mut SnmTestContext) -> anyhow::Result<MockServer> {
  let current = current_dir()?;

  let server = ctx
    .start_server(vec![
      SnmMockServerArg {
        path: "/index.json".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/node/index.json")),
      },
      SnmMockServerArg {
        path: "/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(
          current.join("src/fixtures/node/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz"),
        ),
      },
      SnmMockServerArg {
        path: "/v20.0.0/SHASUMS256.txt".to_string(),
        mime: "text/plain".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/node/v20.0.0/SHASUMS256.txt")),
      },
      SnmMockServerArg {
        path: "/npm/9.0.0".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/npm/9.0.0.json")),
      },
      SnmMockServerArg {
        path: "/npm/-/npm-9.0.0.tgz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/npm/npm-9.0.0.tgz")),
      },
      SnmMockServerArg {
        path: "/pnpm/9.0.0".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/pnpm/9.0.0.json")),
      },
      SnmMockServerArg {
        path: "/pnpm/-/pnpm-9.0.0.tgz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/pnpm/pnpm-9.0.0.tgz")),
      },
      SnmMockServerArg {
        path: "/yarn/1.22.22".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/yarn/1.22.22.json")),
      },
      SnmMockServerArg {
        path: "/yarn/-/yarn-1.22.22.tgz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/yarn/yarn-1.22.22.tgz")),
      },
      SnmMockServerArg {
        path: "/@yarnpkg/cli-dist/4.0.0".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/yarn/4.0.0.json")),
      },
      SnmMockServerArg {
        path: "/@yarnpkg/cli-dist/-/cli-dist-4.0.0.tgz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/yarn/yarn-4.0.0.tgz")),
      },
    ])
    .await?;

  Ok(server)
}
