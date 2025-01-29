use snm_utils::trace_if;
use tracing::trace;

use super::ops::{InstallArgs, PackageManagerOps, RemoveArgs};

pub struct NpmCommandLine {
  pub name: String,
}

impl NpmCommandLine {
  pub fn new() -> Self {
    Self {
      name: String::from("npm"),
    }
  }
}

impl PackageManagerOps for NpmCommandLine {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
    let mut command = vec![self.name.clone()];

    if args.frozen {
      command.push(String::from("ci"));
      return Ok(command);
    }

    command.push(String::from("install"));

    if args.package_spec.is_empty() {
      return Ok(command);
    }

    command.push(args.package_spec.join(" "));

    {
      if args.save_prod {
        command.push(String::from("--save-prod"));
      }

      if args.save_peer {
        command.push(String::from("--save-peer"));
      }

      if args.save_dev {
        command.push(String::from("--save-dev"));
      }

      if args.save_optional {
        command.push(String::from("--save-optional"));
      }
    }

    if args.save_exact {
      command.push(String::from("--save-exact"));
    }

    return Ok(command);
  }

  fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
    let command = vec![self.name.clone(), String::from("uninstall")]
      .into_iter()
      .chain(args.package_spec)
      .collect();
    Ok(command)
  }

  fn run(&self, args: super::ops::RunArgs) -> anyhow::Result<Vec<String>> {
    trace_if!(|| trace!(r#"Ops run args:{:?}"#, &args));

    let command = vec![
      self.name.clone(),
      String::from("run"),
      args.command,
      String::from("--"),
    ]
    .into_iter()
    .chain(args.passthrough_args.clone())
    .collect();

    trace_if!(|| trace!(r#"Ops run cmd:{:?}"#, command,));

    Ok(command)
  }
}

#[cfg(test)]
mod tests {
  // use std::path::PathBuf;

  // use snm_config::snm_config::SnmConfig;
  // use snm_utils::consts::SNM_PREFIX;

  use super::*;
  use crate::pm::PackageManager;

  #[tokio::test]
  async fn should_parse_npm_command() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let cmd = pm.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    println!("{:?}", cmd);

    // let npm = NpmCommandLine::new(&pm);

    Ok(())
  }
}
