use std::{env::var, path::Path};

use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_pm::{package_json::PJson, pm::SPM};
use snm_utils::exec::exec_cli;

pub struct PmShim {
  pub args: Vec<String>,
  pub paths: Vec<String>,
  pub snm_config: SnmConfig,
}

impl PmShim {
  pub fn new(args: Vec<String>, paths: Vec<String>, snm_config: SnmConfig) -> Self {
    Self {
      args,
      paths,
      snm_config,
    }
  }

  pub async fn proxy(&self) -> anyhow::Result<()> {
    let [bin_name, command, args @ ..] = self.args.as_slice() else {
      bail!(r#"deconstruct args failed, args: {:?}"#, self.args);
    };

    let is_escape = var("e")
      .map(|item| item == "1".to_string())
      .unwrap_or(false);

    if !PJson::exists(&self.snm_config.workspace) || is_escape {
      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &self.paths,
        true,
      );
    }

    if !SPM::exists(&self.snm_config.workspace)? {
      if self.snm_config.strict {
        bail!("You have not correctly configured packageManager in package.json");
      }

      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &self.paths,
        true,
      );
    }

    // 处理配置了包管理器的情况
    let spm = SPM::try_from(&self.snm_config.workspace, &self.snm_config)?;
    let pm = &spm.pm;

    // 传进来的有可能是绝对路径, 如果是绝对路径的的话，取 file_name 判断一下。
    // 同时需要保证直取命令的名称，方便 后续的 json.get_bin_with_name(bin_name) 获取到对应 js 的真实路径
    let bin_name = Path::new(bin_name)
      .file_name()
      .and_then(|f| f.to_str())
      .map(|name| {
        name
          .strip_suffix(".cmd")
          .or_else(|| name.strip_suffix(".exe"))
          .unwrap_or(name)
      })
      .unwrap_or(bin_name);

    if bin_name != pm.name() && bin_name != "npx" {
      bail!(
        "Package manager mismatch, expect: {}, actual: {}",
        pm.name().green(),
        bin_name.red()
      );
    }

    let dir = spm.ensure_bin_dir().await?;
    let json = PJson::from(dir)?;

    if let Ok(file) = json.get_bin_with_name(bin_name) {
      exec_cli(
        &[
          &[
            "node".to_string(),
            file.to_string_lossy().into_owned(),
            command.to_owned(),
          ],
          args,
        ]
        .concat(),
        &self.paths,
        true,
      )?;
    } else {
      exec_cli(
        &[&[bin_name.to_string(), command.to_owned()], args].concat(),
        &self.paths,
        true,
      )?;
    }

    Ok(())
  }
}
