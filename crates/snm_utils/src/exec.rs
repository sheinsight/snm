use std::{
  env::{join_paths, split_paths},
  process::{Command, Stdio},
};

use anyhow::bail;
use tracing::trace;

use crate::trace_if;

pub fn exec_cli(paths: Vec<String>, args: Vec<String>) -> anyhow::Result<()> {
  trace_if!(|| {
    trace!("Exec cli, paths: {:?}, args: {:?}", paths, args);
  });

  let [bin_name, args @ ..] = args.as_slice() else {
    bail!("bin name not found");
  };

  let original_path = std::env::var("PATH").unwrap_or_default();

  let path_chunks = paths
    .into_iter()
    .chain(split_paths(&original_path).map(|p| p.to_string_lossy().into_owned()));

  let new_path = join_paths(path_chunks)?.to_string_lossy().into_owned();

  trace_if!(|| {
    trace!(
      r#"Diff PATH ENV
NEW: {}
OLD: {}"#,
      new_path,
      original_path
    );
  });

  let cwd = std::env::current_dir()?;

  let binaries = which::which_in_all(&bin_name, Some(&new_path), cwd)?.collect::<Vec<_>>();

  let snm = which::which("snm")
    .ok()
    .and_then(|p| p.parent().map(|p| p.to_owned()));

  trace_if!(|| {
    trace!("Binaries: {:?} ", binaries);
    trace!("Snm: {:?} ", snm);
  });

  if binaries
    .first()
    .and_then(|b| b.parent())
    .zip(snm)
    .map_or(false, |(p1, p2)| p1 == p2)
  {
    bail!("{} is not a command", bin_name);
  }

  trace_if!(|| {
    trace!("Args: {:?} ", args);
  });

  trace_if!(|| {
    if let Some(binary) = binaries.first() {
      if binary.is_symlink() {
        trace!("Binary is symlink");
        if let Ok(target) = std::fs::read_link(binary) {
          trace!("Symlink target: {:?}", target);
        }
      }
      trace!("Binary path: {:?}", binary);
    }
  });

  // #[cfg(not(target_os = "windows"))]
  // Command::new("sh")
  //   .args(["-c", args.join(" ").as_str()])
  //   .env("PATH", new_path.clone())
  //   .stdout(Stdio::inherit())
  //   .stderr(Stdio::inherit())
  //   .stdin(Stdio::inherit())
  //   .status()?;

  // #[cfg(target_os = "windows")]
  // Command::new("cmd")
  //   .args(["/C", args.join(" ").as_str()])
  //   .env("PATH", new_path.clone())
  //   .stdout(Stdio::inherit())
  //   .stderr(Stdio::inherit())
  //   .stdin(Stdio::inherit())
  //   .status()?;

  Command::new(&bin_name)
    .args(args)
    .env("PATH", new_path.clone())
    // .env("Path", new_path.clone())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .status()?;

  Ok(())
}
