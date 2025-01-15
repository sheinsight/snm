use std::{env::current_dir, path::PathBuf, time::Duration};

use anyhow::Context;

pub struct HttpMocker {
  os: &'static str,
  arch: &'static str,
  ext: &'static str,
  fixtures: PathBuf,
}

impl HttpMocker {
  pub fn builder() -> anyhow::Result<Self> {
    let fixtures = current_dir()?.join("src").join("fixtures");
    Ok(Self {
      os: snm_utils::consts::os(),
      arch: snm_utils::consts::arch(),
      ext: snm_utils::consts::ext(),
      fixtures: fixtures,
    })
  }

  async fn setup_node_index(&self, mock_server: &wiremock::MockServer) -> anyhow::Result<()> {
    let file_path_bug = self.fixtures.join("node").join("index.json");

    let request_url = format!("/index.json");

    println!(
      r#"
    URI   : {}{}
    PATH  : {:?}
    EXISTS: {:?}
"#,
      mock_server.uri(),
      request_url,
      file_path_bug,
      file_path_bug.exists()
    );

    let index_body = std::fs::read(&file_path_bug)
      .with_context(|| format!("Can not found {:?}", &file_path_bug))?;

    wiremock::Mock::given(wiremock::matchers::any())
      .and(wiremock::matchers::path(&request_url))
      .respond_with(
        wiremock::ResponseTemplate::new(200).set_body_raw(index_body, "application/json"),
      )
      .mount(&mock_server)
      .await;
    Ok(())
  }

  async fn setup_node(&self, mock_server: &wiremock::MockServer) -> anyhow::Result<()> {
    let node_versions = vec!["20.0.0".to_string()];

    for v in &node_versions {
      let node_tgz_path_buf = self
        .fixtures
        .join("node")
        .join(format!("v{}", v))
        .join(format!(
          "node-v{}-{}-{}.{}",
          v, self.os, self.arch, self.ext
        ));

      let node_tgz = std::fs::read(&node_tgz_path_buf)
        .with_context(|| format!("Can not found {:?}", &node_tgz_path_buf))?;

      let request_url = format!(
        "/v{version}/node-v{version}-{os}-{arch}.{ext}",
        version = v,
        os = self.os,
        arch = self.arch,
        ext = self.ext
      );

      println!(
        r#"
      URI   : {}{}
      PATH  : {:?}
      EXISTS: {:?}
  "#,
        mock_server.uri(),
        request_url,
        node_tgz_path_buf,
        node_tgz_path_buf.exists()
      );

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(&request_url))
        .respond_with(
          wiremock::ResponseTemplate::new(200).set_body_raw(node_tgz, "application/octet-stream"),
        )
        .mount(&mock_server)
        .await;

      let shasums_path = self
        .fixtures
        .join("node")
        .join(format!("v{}", v))
        .join("SHASUMS256.txt");

      let shasums_url = format!("/v{version}/SHASUMS256.txt", version = v);

      println!(
        r#"
      URI   : {}{}
      PATH  : {:?}
      EXISTS: {:?}
  "#,
        mock_server.uri(),
        shasums_url,
        shasums_path,
        shasums_path.exists()
      );

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(format!(
          "/v{version}/SHASUMS256.txt",
          version = v
        )))
        .respond_with(
          wiremock::ResponseTemplate::new(200).set_body_raw(
            std::fs::read(&shasums_path)
              .with_context(|| format!("Can not found {:?}", &shasums_path))?,
            "text/plain",
          ),
        )
        .mount(&mock_server)
        .await;
    }
    Ok(())
  }

  async fn setup_pm(
    &self,
    mock_server: &wiremock::MockServer,
    pm_name: &str,
    versions: &Vec<String>,
  ) -> anyhow::Result<()> {
    for v in versions {
      let version = semver::Version::parse(v).with_context(|| format!("Invalid version: {}", v))?;
      let req = semver::VersionReq::parse(">1")?;
      let name = if pm_name == "yarn" {
        if req.matches(&version) {
          "@yarnpkg/cli-dist"
        } else {
          "yarn"
        }
      } else {
        pm_name
      };

      let json_path_bug = self.fixtures.join(pm_name).join(format!("{}.json", v));

      let json_request_url = format!("/{name}/{version}", name = name, version = v);

      let json = std::fs::read_to_string(&json_path_bug)
        .with_context(|| format!("Can not found {:?}", &json_path_bug))?;

      println!(
        r#"
        URI   : {}{}
        PATH  : {:?}
        EXISTS: {:?}
    "#,
        mock_server.uri(),
        json_request_url,
        json_path_bug,
        json_path_bug.exists()
      );

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(&json_request_url))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(json, "application/json"))
        .mount(&mock_server)
        .await;

      let tgz_path_buf = self
        .fixtures
        .join(pm_name)
        .join(format!("{}-{}.tgz", pm_name, v));

      let tgz = std::fs::read(&tgz_path_buf)
        .with_context(|| format!("Can not found {:?}", &tgz_path_buf))?;

      let request_url = format!(
        "/{name}/-/{name2}-{version}.tgz",
        name = name,
        name2 = name.split("/").last().unwrap(),
        version = v
      );

      println!(
        r#"
        URI   : {}{}
        PATH  : {:?}
        EXISTS: {:?}
    "#,
        mock_server.uri(),
        request_url,
        tgz_path_buf,
        tgz_path_buf.exists()
      );

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(&request_url))
        .respond_with(
          wiremock::ResponseTemplate::new(200).set_body_raw(tgz, "application/octet-stream"),
        )
        .mount(&mock_server)
        .await;
    }
    Ok(())
  }

  pub async fn build(&self) -> anyhow::Result<wiremock::MockServer> {
    let mock_server = wiremock::MockServer::start().await;

    // 添加调试信息
    println!("=== Mock Server Debug Info ===");
    println!("URI: {}", mock_server.uri());
    println!("Address: {}", mock_server.address());

    self.setup_node_index(&mock_server).await?;
    self.setup_node(&mock_server).await?;

    let npm_versions = vec!["9.0.0".to_string()];
    let pnpm_versions = vec!["9.0.0".to_string()];
    let yarn_versions = vec!["4.0.0".to_string(), "1.22.22".to_string()];

    self.setup_pm(&mock_server, "npm", &npm_versions).await?;
    self.setup_pm(&mock_server, "pnpm", &pnpm_versions).await?;
    self.setup_pm(&mock_server, "yarn", &yarn_versions).await?;

    let node_url = format!(
      "{}/v{version}/node-v{version}-{os}-{arch}.{ext}",
      mock_server.uri(),
      version = "20.0.0",
      os = self.os,
      arch = self.arch,
      ext = self.ext
    );

    // 测试服务器连接性
    let test_response = reqwest::Client::new()
      .get(node_url)
      .timeout(Duration::from_secs(60))
      .send()
      .await;
    println!("Connection test result: {:?}", test_response);

    wiremock::Mock::given(wiremock::matchers::any())
      .respond_with(move |req: &wiremock::Request| {
        println!("404 Request: {} {}", req.method, req.url);
        wiremock::ResponseTemplate::new(404)
      })
      .mount(&mock_server)
      .await;

    Ok(mock_server)
  }
}
