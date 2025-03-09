use std::env::var;

use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::{package_json::PJson, pm::SPM};
use snm_utils::exec::exec_cli;

pub struct PmShim {
  pub args: Vec<String>,
  pub snm_config: SnmConfig,
}

impl PmShim {
  pub async fn proxy(&self) -> anyhow::Result<()> {
    let [bin_name, command, args @ ..] = self.args.as_slice() else {
      bail!(r#"deconstruct args failed, args: {:?}"#, self.args);
    };

    let s_node = SNode::try_from(&self.snm_config)?;

    let node_bin_dir = s_node.get_bin_dir().await?;

    let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

    let paths = vec![node_bin_dir_str];

    let is_escape = match var("e") {
      Ok(val) => val == "1",
      Err(_) => false,
    };

    if !PJson::exists(&self.snm_config.workspace) || is_escape {
      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      );
    }

    if !SPM::exists(&self.snm_config.workspace)? {
      if self.snm_config.strict {
        bail!("You have not correctly configured packageManager in package.json");
      }

      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      );
    }

    // 处理配置了包管理器的情况
    let spm = SPM::try_from(&self.snm_config.workspace, &self.snm_config)?;
    let pm = &spm.pm;

    if pm.name() != bin_name && bin_name != "npx" {
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
            String::from("node"),
            file.to_string_lossy().into_owned(),
            command.to_owned(),
          ],
          args,
        ]
        .concat(),
        &paths,
        true,
      )?;
    } else {
      exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      )?;
    }

    Ok(())
  }
}
