use std::path::Path;

use anyhow::bail;
use colored::Colorize;
use package_json_parser::PackageJsonParser;
use snm_config::snm_config::SnmConfig;
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

    let resolver = snm_package_manager::PackageManagerResolver::from(self.snm_config.clone());

    let Ok(package_manager) = resolver.find_up_package_manager() else {
      // 考虑到 npx 这种情况，找不要必须要透传
      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &self.paths,
        true,
      );
    };

    // let Some(spm) = SPM::from_config_file(&self.snm_config) else {
    //   if self.snm_config.strict {
    //     bail!("You have not correctly configured packageManager in package.json");
    //   }
    //   return exec_cli(
    //     &[&[bin_name.clone(), command.to_owned()], args].concat(),
    //     &self.paths,
    //     true,
    //   );
    // };

    // 处理配置了包管理器的情况
    // let spm = SPM::try_from(&self.snm_config.workspace, &self.snm_config)?;
    // let pm = &spm.pm;

    // 传进来的有可能是绝对路径, 如果是绝对路径的的话，取 file_name 判断一下。
    // 同时需要保证直取命令的名称，方便 后续的 json.get_bin_with_name(bin_name) 获取到对应 js 的真实路径
    // 主要用来拦截处理 snm 自己创建的 symlink , windows 下 symlink 拿到的是绝对路径
    let bin_name = if Path::new(bin_name).is_absolute() {
      Path::new(bin_name)
        .file_name()
        .and_then(|f| f.to_str())
        .map(|name| {
          name
            .strip_suffix(".cmd")
            .or_else(|| name.strip_suffix(".exe"))
            .unwrap_or(name)
        })
        .unwrap_or(bin_name)
    } else {
      bin_name
    };

    let matched = &[package_manager.name(), "npx", "pnpx"].contains(&bin_name);
    if !matched {
      bail!(
        "Package manager mismatch, expect: {}, actual: {}",
        package_manager.name().green(),
        bin_name.red()
      );
    }

    let dir = resolver.ensure_package_manager(&package_manager).await?;

    let json = PackageJsonParser::parse(dir.join("package.json"))?;

    // let json = PJson::from(dir)?;

    let map = json.bin_to_hash_map()?;

    // let x = map.get(bin_name);

    if let Some(file) = map.get(bin_name) {
      let file = dir.join(file);
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

    // if let Ok(file) = json.get_bin_with_name(bin_name) {
    //   exec_cli(
    //     &[
    //       &[
    //         "node".to_string(),
    //         file.to_string_lossy().into_owned(),
    //         command.to_owned(),
    //       ],
    //       args,
    //     ]
    //     .concat(),
    //     &self.paths,
    //     true,
    //   )?;
    // } else {
    //   exec_cli(
    //     &[&[bin_name.to_string(), command.to_owned()], args].concat(),
    //     &self.paths,
    //     true,
    //   )?;
    // }

    Ok(())
  }
}
