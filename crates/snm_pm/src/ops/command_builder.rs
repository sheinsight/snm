use anyhow::bail;

use super::flag::Flag;

pub struct CommandBuilder {
  name: String,
  command: String,
  package_spec: Vec<String>,
  exclu_opts: Vec<Flag>,
  addon_opts: Vec<Flag>,
}

impl CommandBuilder {
  pub fn new(name: String, command: &str) -> Self {
    Self {
      name,
      command: command.to_string(),
      package_spec: Vec::new(),
      exclu_opts: Vec::new(),
      addon_opts: Vec::new(),
    }
  }

  pub fn with_args(mut self, args: Vec<String>) -> Self {
    self.package_spec = args;
    self
  }

  pub fn with_exclu_opts(mut self, opts: Vec<Flag>) -> Self {
    self.exclu_opts = opts;
    self
  }

  pub fn with_addon_opts(mut self, opts: Vec<Flag>) -> Self {
    self.addon_opts = opts;
    self
  }

  pub fn build(self) -> anyhow::Result<Vec<String>> {
    let mut cmd = vec![self.name, self.command];

    // 添加包名（如果有）
    if !self.package_spec.is_empty() {
      cmd.extend(self.package_spec);
    }

    // 处理互斥标志
    let active_excl: Vec<_> = self
      .exclu_opts
      .into_iter()
      .filter(|f| f.condition)
      .collect();

    match active_excl.len() {
      0 => (),
      1 => cmd.extend(active_excl.into_iter().map(|f| f.flag.to_string())),
      _ => bail!("Only one dependency type flag can be used at a time"),
    }

    // 处理附加标志
    cmd.extend(
      self
        .addon_opts
        .into_iter()
        .filter(|f| f.condition)
        .map(|f| f.flag.to_string()),
    );

    Ok(cmd)
  }
}
