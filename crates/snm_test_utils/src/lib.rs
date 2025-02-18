use std::{
  collections::HashMap,
  env::{self, current_dir},
  fs,
  path::PathBuf,
};

use anyhow::Context;
use test_context::AsyncTestContext;
use textwrap::dedent;
use wiremock::MockServer;

pub enum ResponseSource {
  Raw(Vec<u8>),
  File(PathBuf),
}

pub struct SnmMockServerArg {
  pub path: String,
  pub mime: String,
  pub resp: ResponseSource,
}

pub struct SnmTestContext {
  id: String,
  // name: String,
  counter: usize,
  cwd: String,
  temp_dir: PathBuf,
  pub env_vars: HashMap<String, String>,
  snapshots: Vec<String>,
}

impl AsyncTestContext for SnmTestContext {
  async fn setup() -> Self {
    let temp_dir = tempfile::tempdir().unwrap();

    let env_vars = Self::setup_env_vars();

    let cwd = current_dir().unwrap();

    Self {
      id: uuid::Uuid::new_v4().to_string(),
      // name: "".to_string(),
      counter: 0,
      cwd: cwd.to_string_lossy().to_string(),
      temp_dir: temp_dir.into_path(),
      env_vars,
      snapshots: vec![],
    }
  }

  async fn teardown(self) {
    for (key, _) in self.env_vars {
      unsafe {
        env::remove_var(key);
      }
    }
    fs::remove_dir_all(self.temp_dir.clone()).unwrap();
  }
}

impl SnmTestContext {
  fn setup_env_vars() -> HashMap<String, String> {
    let env_path = env!("PATH");
    let path_sep = if cfg!(windows) { ";" } else { ":" };
    let debug_dir = dunce::canonicalize(Self::get_debug_dir())
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let new_path = format!("{}{}{}", debug_dir, path_sep, env_path);
    HashMap::from([
      ("PATH".to_string(), new_path),
      ("RUST_BACKTRACE".to_string(), "0".to_string()),
    ])
  }

  fn get_debug_dir() -> PathBuf {
    let e2e_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let root_dir = e2e_dir
      .parent()
      .expect("Failed to get root dir")
      .parent()
      .expect("Failed to get root dir");

    root_dir.join("target/debug")
  }
}

impl SnmTestContext {
  // pub fn set_name(&mut self, name: &str) {
  //   self.name = name.to_owned();
  //   self.snapshots.push(format!("name: {:?}", name));
  // }

  pub fn set_cwd(&mut self, path: &PathBuf) {
    self.cwd = path.to_string_lossy().to_string();
  }

  pub fn set_envs(&mut self, envs: &[(String, String)]) {
    for (key, value) in envs {
      self.env_vars.insert(key.to_owned(), value.to_owned());
      unsafe {
        env::set_var(key, value);
      }
    }
  }

  pub fn get_temp_dir(&self) -> &PathBuf {
    &self.temp_dir
  }

  pub fn get_id(&self) -> &str {
    &self.id
  }
}

impl SnmTestContext {
  fn get_builtin_request() -> anyhow::Result<Vec<SnmMockServerArg>> {
    let current = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(vec![
      SnmMockServerArg {
        path: "/index.json".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/node/index.json")),
      },
      SnmMockServerArg {
        path: "/nodejs/Release/main/schedule.json".to_string(),
        mime: "application/json".to_string(),
        resp: ResponseSource::File(current.join("src/fixtures/node/schedule.json")),
      },
      SnmMockServerArg {
        path: "/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(
          current.join("src/fixtures/node/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz"),
        ),
      },
      SnmMockServerArg {
        path: "/v20.0.0/node-v20.0.0-linux-x64.tar.xz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(
          current.join("src/fixtures/node/v20.0.0/node-v20.0.0-linux-x64.tar.xz"),
        ),
      },
      SnmMockServerArg {
        path: "/v20.0.0/node-v20.0.0-linux-arm64.tar.xz".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(
          current.join("src/fixtures/node/v20.0.0/node-v20.0.0-linux-arm64.tar.xz"),
        ),
      },
      SnmMockServerArg {
        path: "/v20.0.0/node-v20.0.0-win-x64.zip".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(
          current.join("src/fixtures/node/v20.0.0/node-v20.0.0-win-x64.zip"),
        ),
      },
      SnmMockServerArg {
        path: "/v20.0.0/node-v20.0.0-win-x86.zip".to_string(),
        mime: "application/octet-stream".to_string(),
        resp: ResponseSource::File(
          current.join("src/fixtures/node/v20.0.0/node-v20.0.0-win-x86.zip"),
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
  }

  pub async fn start_server(&mut self) -> anyhow::Result<MockServer> {
    let mock_server = wiremock::MockServer::start().await;

    let builtin_request = Self::get_builtin_request()?;

    for SnmMockServerArg { path, mime, resp } in builtin_request {
      let resp = match resp {
        ResponseSource::Raw(vec) => vec,
        ResponseSource::File(path_buf) => {
          fs::read(&path_buf).with_context(|| format!("Can not found {:?}", &path_buf))?
        }
      };

      wiremock::Mock::given(wiremock::matchers::any())
        .and(wiremock::matchers::path(path))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(resp, &mime))
        .mount(&mock_server)
        .await;
    }

    let mock_server_uri = mock_server.uri().to_string();

    self.set_envs(&[
      (
        "SNM_HOME_DIR".to_string(),
        self.get_temp_dir().to_string_lossy().to_string(),
      ),
      ("SNM_NODE_DIST_URL".to_string(), mock_server_uri.clone()),
      ("SNM_NPM_REGISTRY".to_string(), mock_server_uri.clone()),
      (
        "SNM_NODE_GITHUB_RESOURCE_HOST".to_string(),
        mock_server_uri.clone(),
      ),
    ]);

    Ok(mock_server)
  }
}

impl SnmTestContext {
  pub fn exec(&self, command: &str) -> anyhow::Result<String> {
    let mut cmd = if cfg!(windows) {
      let command = if command.starts_with("snm") {
        command.replace("snm", "snm.exe")
      } else {
        command.to_string()
      };
      let mut cmd = std::process::Command::new("cmd");
      cmd.args(["/C", &command]);
      cmd
    } else {
      let mut cmd = std::process::Command::new("sh");
      cmd.args(["-c", &command]);
      cmd
    };

    self.env_vars.iter().for_each(|(key, value)| {
      cmd.env(key, value);
    });

    cmd.current_dir(self.cwd.clone());

    let output = cmd.output()?;

    let res = format!(
      r#"
stdout:{}
stderr:{}
"#,
      String::from_utf8(output.stdout.clone())?.trim().to_string(),
      String::from_utf8(output.stderr.clone())?.trim().to_string()
    );

    Ok(res)
  }

  pub fn add_snapshot(&mut self, command: &str) -> anyhow::Result<&mut Self> {
    self.counter += 1;
    let res = self.exec(command)?;
    let res = dedent(&format!(
      r#"
id: {}
is: {}

{}"#,
      self.counter, command, res
    ));
    self.snapshots.push(res);
    Ok(self)
  }

  pub fn assert_snapshots<F>(&mut self, f: F) -> anyhow::Result<()>
  where
    F: Fn(&str),
  {
    // let envs = self
    //   .env_vars
    //   .iter()
    //   .filter_map(|(k, v)| {
    //     if vec!["SNM_STRICT".to_string()].contains(k) {
    //       Some(format!("{}:{}", k, v))
    //     } else {
    //       None
    //     }
    //   })
    //   .collect::<Vec<_>>()
    //   .join("\n");

    // self.snapshots.insert(0, envs);

    let res = self.snapshots.join("\n");

    f(&res);

    Ok(())
  }
}
