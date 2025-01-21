use std::process::{Command, Stdio};

use anyhow::{bail, Context};
use tracing::trace;

use crate::trace_if;

pub fn exec_cli(paths: Vec<String>, args: Vec<String>) -> anyhow::Result<()> {
  let bin_name = args.get(0).context("bin name not found")?.to_owned();

  let original_path = std::env::var("PATH").unwrap_or_default();

  #[cfg(target_os = "windows")]
  let separator = ";";

  #[cfg(not(target_os = "windows"))]
  let separator = ":";

  let new_path: String = format!("{}{}{}", paths.join(separator), separator, original_path);

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
    .args(
      args
        .iter()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<_>>(),
    )
    .env("PATH", new_path.clone())
    // .env("Path", new_path.clone())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .status()?;

  Ok(())
}
