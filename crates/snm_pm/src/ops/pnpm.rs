use snm_utils::trace_if;
use tracing::trace;

use super::ops::{InstallArgs, PackageManagerOps, RemoveArgs};

pub struct PnpmCommandLine {
  pub name: String,
}

impl PnpmCommandLine {
  pub fn new() -> Self {
    Self {
      name: String::from("pnpm"),
    }
  }
}

impl PackageManagerOps for PnpmCommandLine {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
    let mut command = vec![self.name.clone()];

    if args.package_spec.is_empty() {
      command.push(String::from("install"));

      if args.frozen {
        command.push(String::from("--frozen-lockfile"));
      }

      return Ok(command);
    }

    command.push(String::from("add"));

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
    let command = vec![self.name.clone(), String::from("remove")]
      .into_iter()
      .chain(args.package_spec)
      .collect();
    Ok(command)
  }

  fn run(&self, args: super::ops::RunArgs) -> anyhow::Result<Vec<String>> {
    trace_if!(|| trace!(r#"Ops run args:{:?}"#, &args));

    let command = vec![self.name.clone(), String::from("run"), args.command]
      .into_iter()
      .chain(args.passthrough_args.clone())
      .collect();

    trace_if!(|| trace!(r#"Ops run cmd:{:?}"#, command));

    Ok(command)
  }
}
