use std::{env::current_dir, error::Error, path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub struct Executor {
  pub current: PathBuf,
  pub envs: Vec<(String, String)>,
}

impl Executor {
  pub fn exec(&self, shell: &str) -> Result<String, Box<dyn Error>> {
    let shell_vec = shell
      .split(" ")
      .map(|item| item.trim())
      .collect::<Vec<&str>>();

    if let Some((bin_name, args)) = shell_vec.split_first() {
      let output = Command::new(bin_name)
        .envs(self.envs.clone())
        .args(args)
        .current_dir(&self.current)
        .output()?;

      let stdout = String::from_utf8_lossy(&output.stdout).to_string();
      let stderr = String::from_utf8_lossy(&output.stderr).to_string();

      println!(
        r##"
Exec shell: {}
Stdout: {}
Stderr: {}
        "##,
        shell, stdout, stderr
      );
      Ok(stdout)
    } else {
      Err("Invalid shell command".into())
    }
  }
}

pub struct ExecBuilder {
  executor: Executor,
}

impl ExecBuilder {
  pub fn builder() -> Self {
    Self {
      executor: Executor {
        current: PathBuf::new(),
        envs: vec![],
      },
    }
  }

  pub fn current(&mut self, current: &PathBuf) -> &mut Self {
    self.executor.current = current.clone();
    self
  }

  pub fn envs(&mut self, envs: Vec<(String, String)>) -> &mut Self {
    let binary_path = current_dir().unwrap().join("tests");
    self.executor.envs = vec![
      vec![("PATH".to_string(), binary_path.display().to_string())],
      envs,
    ]
    .concat();
    self
  }

  pub fn build(&self) -> Executor {
    self.executor.clone()
  }
}
