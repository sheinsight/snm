use std::{
  env::{join_paths, split_paths},
  process::{exit, Command, Stdio},
};

use anyhow::bail;
use tracing::trace;

pub fn exec_cli(args: &Vec<String>, paths: &Vec<String>, check_snm: bool) -> anyhow::Result<()> {
  trace!("exec_cli args: {:#?}", args);

  let [bin_name, args @ ..] = args.as_slice() else {
    bail!("No binary name provided in arguments");
  };

  let new_path = create_path_with_additional_dirs(paths.clone())?;

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
