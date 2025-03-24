use std::{
  env::{join_paths, split_paths},
  process::{Command, Stdio},
  time::Duration,
};

use anyhow::bail;
use tracing::trace;
use wait_timeout::ChildExt;

use crate::trace_if;

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

  trace_if!(|| {
    if let Some(binary) = binaries.first() {
      trace!(
        r#"which {}
first binary: {}"#,
        bin_name,
        binary.to_string_lossy()
      );
      if binary.is_symlink() {
        trace!("Binary {} is symlink", binary.to_string_lossy());
        if let Ok(target) = std::fs::read_link(binary) {
          trace!(
            r#"Symlink:
{} symlink from {:?}"#,
            binary.to_string_lossy(),
            target.to_string_lossy()
          );
        }
      }
    }
  });

  let program = binaries.first().unwrap();

  let mut c = Command::new(program)
    .args(args)
    .env("PATH", new_path.clone())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .spawn()?;

  trace_if!(|| {
    trace!(
      r#"exec command: 
    {} {}"#,
      &bin_name,
      args.join(" ")
    );
  });

  let res = c.wait_timeout(Duration::from_secs(6000))?;

  if let Some(status) = res {
    if status.success() {
      return Ok(());
    } else {
      bail!("command {} failed", bin_name);
    }
  } else {
    bail!("command {} timeout", bin_name);
  }
}

fn create_path_with_additional_dirs(additional_paths: Vec<String>) -> anyhow::Result<String> {
  let o = std::env::var("PATH").unwrap_or_default();

  let n = split_paths(&o).map(|p| p.to_string_lossy().into_owned());

  let path_chunks = additional_paths.into_iter().chain(n);

  let n = join_paths(path_chunks)?.to_string_lossy().into_owned();

  trace_if!(|| {
    trace!(
      r#"Diff PATH ENV
NEW: {}
OLD: {}"#,
      n,
      o
    );
  });

  Ok(n)
}

fn check_snm_binary(bin_name: &str, binaries: &[std::path::PathBuf]) -> anyhow::Result<()> {
  let snm = which::which("snm")
    .ok()
    .and_then(|p| p.parent().map(|p| p.to_owned()));

  trace_if!(|| {
    trace!("Binaries: {:?}", binaries);
    trace!("Snm: {:?}", snm);
  });

  if binaries
    .first()
    .and_then(|b| b.parent())
    .zip(snm)
    .map_or(false, |(p1, p2)| p1 == p2)
  {
    bail!("'{}' is not a valid command", bin_name);
  }
  Ok(())
}
