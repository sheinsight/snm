use std::{env::current_dir, path::PathBuf};

use anyhow::Context;

pub struct HttpMocker {
  // mock_server: wiremock::MockServer,
  node_versions: Vec<String>,
  npm_versions: Vec<String>,
  pnpm_versions: Vec<String>,
  yarn_versions: Vec<String>,
  os: &'static str,
  arch: &'static str,
  ext: &'static str,
  fixtures: PathBuf,
}

impl HttpMocker {
  pub fn builder() -> anyhow::Result<Self> {
    let fixtures = current_dir()?.join("src").join("fixtures");
    Ok(Self {
      node_versions: Default::default(),
      npm_versions: Default::default(),
      pnpm_versions: Default::default(),
      yarn_versions: Default::default(),
      os: snm_utils::consts::os(),
      arch: snm_utils::consts::arch(),
      ext: snm_utils::consts::ext(),
      fixtures: fixtures,
    })
  }

  pub fn with_node(mut self, versions: Vec<&str>) -> Self {
    self.node_versions = versions.into_iter().map(String::from).collect();
    self
  }

  pub fn with_npm(mut self, versions: Vec<&str>) -> Self {
    self.npm_versions = versions.into_iter().map(String::from).collect();
    self
  }

  pub fn with_pnpm(mut self, versions: Vec<&str>) -> Self {
    self.pnpm_versions = versions.into_iter().map(String::from).collect();
    self
  }

  pub fn with_yarn(mut self, versions: Vec<&str>) -> Self {
    self.yarn_versions = versions.into_iter().map(String::from).collect();
    self
  }

  async fn setup_node_index(&self, mock_server: &wiremock::MockServer) -> anyhow::Result<()> {
    let file_path = self.fixtures.join("node").join("index.json");

    let index_body =
      std::fs::read(&file_path).with_context(|| format!("Can not found {:?}", &file_path))?;

    wiremock::Mock::given(wiremock::matchers::any())
      .and(wiremock::matchers::path("index.json"))
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
      let fixture_path = self
        .fixtures
        .join("node")
        .join(format!("v{}", v))
        .join(format!(
          "node-v{}-{}-{}.{}",
          v, self.os, self.arch, self.ext
        ));

      println!(" node fixture_path: {:?}", fixture_path);

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(format!(
          "/v{version}/node-v{version}-{os}-{arch}.{ext}",
          version = v,
          os = snm_utils::consts::os(),
          arch = snm_utils::consts::arch(),
          ext = snm_utils::consts::ext()
        )))
        .respond_with(
          wiremock::ResponseTemplate::new(200).set_body_raw(
            std::fs::read(&fixture_path)
              .with_context(|| format!("Can not found {:?}", &fixture_path))?,
            "application/x-xz",
          ),
        )
        .mount(&mock_server)
        .await;

      let shasums_path = self
        .fixtures
        .join("node")
        .join(format!("v{}", v))
        .join("SHASUMS256.txt");

      wiremock::Mock::given(wiremock::matchers::method("GET"))
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

      let p = self.fixtures.join(pm_name).join(format!("{}.json", v));

      let json = std::fs::read_to_string(&p).with_context(|| format!("Can not found {:?}", &p))?;

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(format!(
          "/{name}/{version}",
          name = name,
          version = v
        )))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(json, "application/json"))
        .mount(&mock_server)
        .await;

      let tgz_p = self
        .fixtures
        .join(pm_name)
        .join(format!("{}-{}.tgz", pm_name, v));

      let tgz = std::fs::read(&tgz_p).with_context(|| format!("Can not found {:?}", &tgz_p))?;

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(format!(
          "/{name}/-/{name2}-{version}.tgz",
          name = name,
          name2 = name.split("/").last().unwrap(),
          version = v
        )))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(tgz, "application/x-xz"))
        .mount(&mock_server)
        .await;
    }
    Ok(())
  }

  pub async fn build(&self) -> anyhow::Result<wiremock::MockServer> {
    let mock_server = wiremock::MockServer::start().await;

    self.setup_node_index(&mock_server).await?;
    self.setup_node(&mock_server).await?;

    let npm_versions = vec!["9.0.0".to_string()];
    let pnpm_versions = vec!["9.0.0".to_string()];
    let yarn_versions = vec!["4.0.0".to_string(), "1.22.22".to_string()];

    self.setup_pm(&mock_server, "npm", &npm_versions).await?;
    self.setup_pm(&mock_server, "pnpm", &pnpm_versions).await?;
    self.setup_pm(&mock_server, "yarn", &yarn_versions).await?;

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

// pub const fn get_tarball_ext() -> &'static str {
//     #[cfg(target_os = "windows")]
//     {
//         "zip"
//     }
//     #[cfg(any(target_os = "linux", target_os = "macos"))]
//     {
//         "tar.xz"
//     }
//     #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
//     {
//         "unknown"
//     }
// }

// pub const fn get_arch() -> &'static str {
//     #[cfg(target_arch = "x86")]
//     {
//         "x86"
//     }
//     #[cfg(target_arch = "x86_64")]
//     {
//         "x64"
//     }
//     #[cfg(target_arch = "arm")]
//     {
//         "armv7l"
//     }
//     #[cfg(target_arch = "aarch64")]
//     {
//         "arm64"
//     }
//     #[cfg(target_arch = "powerpc64")]
//     {
//         "ppc64"
//     }
//     #[cfg(target_arch = "powerpc64le")]
//     {
//         "ppc64le"
//     }
//     #[cfg(target_arch = "s390x")]
//     {
//         "s390x"
//     }
//     #[cfg(not(any(
//         target_arch = "x86",
//         target_arch = "x86_64",
//         target_arch = "arm",
//         target_arch = "aarch64",
//         target_arch = "powerpc64",
//         target_arch = "powerpc64le",
//         target_arch = "s390x"
//     )))]
//     {
//         "unknown"
//     }
// }

// pub const fn get_os() -> &'static str {
//     #[cfg(target_os = "macos")]
//     {
//         "darwin"
//     }
//     #[cfg(target_os = "windows")]
//     {
//         "win"
//     }
//     #[cfg(target_os = "linux")]
//     {
//         "linux"
//     }
//     #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
//     {
//         "unknown"
//     }
// }
