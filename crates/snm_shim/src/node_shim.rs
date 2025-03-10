use snm_utils::exec::exec_cli;

pub struct NodeShim {
  pub args: Vec<String>,
  pub paths: Vec<String>,
}

impl NodeShim {
  pub fn new(args: Vec<String>, paths: Vec<String>) -> Self {
    Self { args, paths }
  }

  pub async fn proxy(&self) -> anyhow::Result<()> {
    exec_cli(&self.args, &self.paths, true)?;

    Ok(())
  }
}
