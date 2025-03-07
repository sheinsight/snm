use std::{env::var, path::Path};

use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::{package_json::PJson, pm::SPM};
use snm_utils::{consts::SNM_PREFIX, exec::exec_cli};
// use tracing::trace;
// const NPM_COMMANDS: [&str; 2] = ["npm", "npx"];

// fn is_npm_command(command: &str) -> bool {
//   NPM_COMMANDS.contains(&command)
// }

// fn handle_npm_command(
//   bin_name: &str,
//   command: &str,
//   args: &[String],
//   node_bin_dir: &std::path::Path,
//   paths: &Vec<String>,
// ) -> anyhow::Result<()> {
//   let bin_name = if cfg!(windows) {
//     format!("{}.cmd", bin_name)
//   } else {
//     bin_name.to_string()
//   };

//   let pm_bin_file = node_bin_dir.join(bin_name);

//   trace_if!(|| {
//     trace!("Default bin file: {:?}", pm_bin_file);
//   });

//   let mut exec_args = vec![
//     pm_bin_file.to_string_lossy().to_string(),
//     command.to_owned(),
//   ];
//   exec_args.extend(args.iter().cloned());

//   exec_cli(&exec_args, paths, true)
// }

// pub async fn load_pm(snm_config: &SnmConfig, args: &Vec<String>) -> anyhow::Result<()> {
//   let [bin_name, command, args @ ..] = args.as_slice() else {
//     bail!(
//       r#"No binary name provided in arguments
// args: {:?}"#,
//       args
//     );
//   };

//   trace_if!(|| {
//     trace!(
//       r#"Load pm shim , exe_name: {}, args: {}"#,
//       bin_name,
//       args.join(" ")
//     );
//   });

//   let snode = SNode::try_from(&snm_config)?;

//   let node_bin_dir = snode.get_bin_dir().await?;

//   let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

//   let paths = vec![node_bin_dir_str];

//   let is_escape = match var("e") {
//     Ok(val) => val == "1",
//     Err(_) => false,
//   };

//   // 如果没有 package.json,直接执行命令
//   if !PJson::exists(&snm_config.workspace) || is_escape {
//     // TODO 不记得当时为什么要做这个判断，感觉直接透传命令就行了。因为本来就会加载 node 的 bin 目录。
//     // return if is_npm_command(bin_name) {
//     //   handle_npm_command(bin_name, command, args, &node_bin_dir, &paths)
//     // } else {
//     //   exec_cli(
//     //     &[&[bin_name.clone(), command.to_owned()], args].concat(),
//     //     &paths,
//     //     true,
//     //   )
//     // };
//     return exec_cli(
//       &[&[bin_name.clone(), command.to_owned()], args].concat(),
//       &paths,
//       true,
//     );
//   }

//   // 检查是否配置了包管理器
//   if !SPM::exists(&snm_config.workspace)? {
//     if snm_config.strict {
//       bail!("You have not correctly configured packageManager in package.json");
//     }
//     // TODO 不记得当时为什么要做这个判断，感觉直接透传命令就行了。因为本来就会加载 node 的 bin 目录。
//     // return if is_npm_command(bin_name) {
//     //   handle_npm_command(bin_name, command, args, &node_bin_dir, &paths)
//     // } else {
//     //   exec_cli(
//     //     &[&[bin_name.clone(), command.to_owned()], args].concat(),
//     //     &paths,
//     //     true,
//     //   )
//     // };
//     return exec_cli(
//       &[&[bin_name.clone(), command.to_owned()], args].concat(),
//       &paths,
//       true,
//     );
//   }

//   // 处理配置了包管理器的情况
//   let spm = SPM::try_from(&snm_config.workspace, &snm_config)?;
//   let pm = &spm.pm;

//   if pm.name() != bin_name {
//     bail!(
//       "Package manager mismatch, expect: {}, actual: {}",
//       pm.name().green(),
//       bin_name.red()
//     );
//   }

//   let dir = spm.ensure_bin_dir().await?;
//   let json = PJson::from(dir)?;
//   let file = json.get_bin_with_name(bin_name)?;

//   exec_cli(
//     &[
//       &[
//         String::from("node"),
//         file.to_string_lossy().into_owned(),
//         command.to_owned(),
//       ],
//       args,
//     ]
//     .concat(),
//     &paths,
//     true,
//   )?;

//   Ok(())
// }

pub struct PmShim {
  pub args: Vec<String>,
}

impl PmShim {
  pub async fn proxy<T: AsRef<Path>>(&self, cwd: &T) -> anyhow::Result<()> {
    let snm_config = SnmConfig::from(SNM_PREFIX, &cwd)?;
    let [bin_name, command, args @ ..] = self.args.as_slice() else {
      bail!(r#"deconstruct args failed, args: {:?}"#, self.args);
    };

    let s_node = SNode::try_from(&snm_config)?;

    let node_bin_dir = s_node.get_bin_dir().await?;

    let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

    let paths = vec![node_bin_dir_str];

    let is_escape = match var("e") {
      Ok(val) => val == "1",
      Err(_) => false,
    };

    if !PJson::exists(&snm_config.workspace) || is_escape {
      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      );
    }

    if !SPM::exists(&snm_config.workspace)? {
      if snm_config.strict {
        bail!("You have not correctly configured packageManager in package.json");
      }
      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      );
    }

    // 处理配置了包管理器的情况
    let spm = SPM::try_from(&snm_config.workspace, &snm_config)?;
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
    // let file = json.get_bin_with_name(bin_name);

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
