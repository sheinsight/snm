use std::{
  env::{join_paths, split_paths},
  process::{exit, Command, Stdio},
};

use anyhow::bail;
use tracing::trace;

/// 使用当前进程环境执行命令，保持原有调用行为不变。
pub fn exec_cli(args: &Vec<String>, paths: &Vec<String>, check_snm: bool) -> anyhow::Result<()> {
  exec_cli_with_envs(args, paths, check_snm, &[])
}

/// 仅为子进程附加环境变量，避免通过修改全局环境影响同进程中的其他任务。
pub fn exec_cli_with_envs(
  args: &[String],
  paths: &[String],
  check_snm: bool,
  env_overrides: &[(&str, &str)],
) -> anyhow::Result<()> {
  trace!("exec_cli args: {:#?}", args);

  let [bin_name, args @ ..] = args else {
    bail!("No binary name provided in arguments");
  };

  let new_path = create_path_with_additional_dirs(paths.to_owned())?;

  let cwd = std::env::current_dir()?;

  let binaries = which::which_in_all(&bin_name, Some(&new_path), cwd)?.collect::<Vec<_>>();

  if check_snm {
    check_snm_binary(bin_name, &binaries)?;
  }

  if let Some(program) = binaries.first() {
    trace!("which first binary:{:#?}", program);

    if program.is_symlink() {
      trace!("program is symlink");
      let target = std::fs::read_link(program)?;
      trace!("target: {:#?}", target);
    }

    let output = Command::new(program)
      .args(args)
      .env("PATH", new_path.clone())
      // 覆盖值只作用于即将启动的子进程，父进程环境保持不变。
      .envs(env_overrides.iter().copied())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .stdin(Stdio::inherit())
      .output()?;

    if output.status.success() {
      Ok(())
    } else {
      exit(output.status.code().unwrap_or(1));
    }
  } else {
    bail!(r#"No binary found in PATH , You can try to install it by `snm setup`"#);
  }
}

fn create_path_with_additional_dirs(additional_paths: Vec<String>) -> anyhow::Result<String> {
  let o = std::env::var("PATH").unwrap_or_default();

  let n = split_paths(&o).map(|p| p.to_string_lossy().into_owned());

  let path_chunks = additional_paths.into_iter().chain(n);

  let n = join_paths(path_chunks)?.to_string_lossy().into_owned();

  trace!(
    r#"Diff PATH ENV
NEW: {}
OLD: {}"#,
    n,
    o
  );

  Ok(n)
}

fn check_snm_binary(bin_name: &str, binaries: &[std::path::PathBuf]) -> anyhow::Result<()> {
  let snm = which::which("snm")
    .ok()
    .and_then(|p| p.parent().map(|p| p.to_owned()));

  trace!("Binaries: {:?}", binaries);
  trace!("Snm: {:?}", snm);

  if binaries
    .first()
    .and_then(|b| b.parent())
    .zip(snm.clone())
    .map_or(false, |(p1, p2)| p1 == p2)
  {
    bail!("'{}' is not a valid command", bin_name);
  }
  Ok(())
}
