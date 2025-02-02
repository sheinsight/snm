use std::{collections::HashMap, env, path::PathBuf};

use test_context::AsyncTestContext;

pub struct SnmTestContext {
  temp_dir: PathBuf,
  env_vars: HashMap<String, String>,
  id: String,
}

impl AsyncTestContext for SnmTestContext {
  async fn setup() -> Self {
    let temp_dir = tempfile::tempdir().unwrap();
    Self {
      temp_dir: temp_dir.into_path(),
      env_vars: HashMap::new(),
      id: uuid::Uuid::new_v4().to_string(),
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
