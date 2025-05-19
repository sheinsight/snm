use std::path::Path;

use anyhow::Context;
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
    let first_arg = self.args.first().context("No command provided")?;

    let bin_name = if Path::new(first_arg).is_absolute() {
      // 如果是绝对路径，直接使用文件名
      Path::new(first_arg)
        .file_name()
        .context(format!("Invalid absolute path {:#?}", first_arg))?
        .to_string_lossy()
        .into_owned()
    } else {
      first_arg.to_string()
    };

    let args = [&[bin_name], &self.args[1..]].concat();

    exec_cli(&args, &self.paths, true)?;

    Ok(())
  }
}
