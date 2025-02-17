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
  name: String,
  counter: usize,
  temp_dir: PathBuf,
  env_vars: HashMap<String, String>,
  snapshots: Vec<String>,
}

impl AsyncTestContext for SnmTestContext {
  async fn setup() -> Self {
    let temp_dir = tempfile::tempdir().unwrap();

    let env_vars = Self::setup_env_vars();

    Self {
      id: uuid::Uuid::new_v4().to_string(),
      name: "".to_string(),
      counter: 0,
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
    HashMap::from([("PATH".to_string(), new_path)])
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
  pub fn name(&mut self, name: &str) {
    self.name = name.to_owned();
  }

  pub fn cwd(&mut self, path: &PathBuf) {
    self.temp_dir = path.to_owned();
  }

  pub fn vars(&mut self, envs: &[(String, String)]) {
    for (key, value) in envs {
      self.env_vars.insert(key.to_owned(), value.to_owned());
      unsafe {
        env::set_var(key, value);
      }
    }
  }

  pub fn temp_dir(&self) -> &PathBuf {
    &self.temp_dir
  }

  pub fn id(&self) -> &str {
    &self.id
  }
}

impl SnmTestContext {
  pub async fn start_server(&self, metadata: Vec<SnmMockServerArg>) -> anyhow::Result<MockServer> {
    let mock_server = wiremock::MockServer::start().await;

    for SnmMockServerArg { path, mime, resp } in metadata {
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

    cmd.current_dir(self.temp_dir.clone());

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

  pub fn assert_snapshots<F>(&self, f: F) -> anyhow::Result<()>
  where
    F: Fn(&str, &str),
  {
    let res = self.snapshots.join("\n");

    f(&self.name, &res);

    Ok(())
  }
}
