use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::{package_json::PJson, pm::SPM};
use snm_utils::{exec::exec_cli, trace_if};
use tracing::trace;
const NPM_COMMANDS: [&str; 2] = ["npm", "npx"];

fn is_npm_command(command: &str) -> bool {
  NPM_COMMANDS.contains(&command)
}

fn handle_npm_command(
  bin_name: &str,
  command: &str,
  args: &[String],
  node_bin_dir: &std::path::Path,
  paths: &Vec<String>,
) -> anyhow::Result<()> {
  let bin_name = if cfg!(windows) {
    format!("{}.cmd", bin_name)
  } else {
    bin_name.to_string()
  };

  let pm_bin_file = node_bin_dir.join(bin_name);

  trace_if!(|| {
    trace!("Default bin file: {:?}", pm_bin_file);
  });

  let mut exec_args = vec![
    pm_bin_file.to_string_lossy().to_string(),
    command.to_owned(),
  ];
  exec_args.extend(args.iter().cloned());

  exec_cli(&exec_args, paths, true)
}

pub async fn load_pm(snm_config: &SnmConfig, args: &Vec<String>) -> anyhow::Result<()> {
  let [bin_name, command, args @ ..] = args.as_slice() else {
    bail!(
      r#"No binary name provided in arguments
args: {:?}"#,
      args
    );
  };

  trace_if!(|| {
    trace!(
      r#"Load pm shim , exe_name: {}, args: {}"#,
      bin_name,
      args.join(" ")
    );
  });

  let snode = SNode::try_from(&snm_config)?;

  let node_bin_dir = snode.get_bin_dir().await?;

  let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

  let paths = vec![node_bin_dir_str];

  // 如果没有 package.json,直接执行命令
  if !PJson::exists(&snm_config.workspace) {
    return if is_npm_command(bin_name) {
      handle_npm_command(bin_name, command, args, &node_bin_dir, &paths)
    } else {
      exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      )
    };
  }

  // 检查是否配置了包管理器
  if !SPM::exists(&snm_config.workspace)? {
    if snm_config.strict {
      bail!("You have not correctly configured packageManager in package.json");
    }

    return if is_npm_command(bin_name) {
      handle_npm_command(bin_name, command, args, &node_bin_dir, &paths)
    } else {
      exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &paths,
        true,
      )
    };
  }

  // 处理配置了包管理器的情况
  let spm = SPM::try_from(&snm_config.workspace, &snm_config)?;
  let pm = &spm.pm;

  if pm.name() != bin_name && snm_config.restricted_list.contains(command) {
    bail!(
      "Package manager mismatch, expect: {}, actual: {} . Restricted list: {}",
      pm.name().green(),
      bin_name.red(),
      snm_config.restricted_list.join(", ").black()
    );
  }

  let dir = spm.ensure_bin_dir().await?;
  let json = PJson::from(dir)?;
  let file = json.get_bin_with_name(bin_name)?;

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

  Ok(())
}
