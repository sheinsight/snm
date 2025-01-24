use snm_utils::trace_if;
use tracing::trace;

use super::ops::{InstallArgs, PackageManagerOps, RemoveArgs};
use crate::pm_metadata::PackageManagerMetadata;

pub struct YarnCommandLine<'a> {
  pub metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> YarnCommandLine<'a> {
  pub fn new(pm: &'a PackageManagerMetadata) -> Self {
    Self { metadata: pm }
  }
}

impl<'a> PackageManagerOps for YarnCommandLine<'a> {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
    let mut command = vec![self.metadata.name.clone(), String::from("install")];

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
        // nothing
      }
      if args.save_dev {
        command.push(String::from("--dev"));
      }
      if args.save_peer {
        command.push(String::from("--peer"));
      }
      if args.save_optional {
        command.push(String::from("--optional"));
      }
    }

    if args.save_exact {
      command.push(String::from("--exact"));
    }

    return Ok(command);
  }

  fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
    let command = vec![self.metadata.name.clone(), String::from("remove")]
      .into_iter()
      .chain(args.package_spec)
      .collect();
    Ok(command)
  }

  fn run(&self, args: super::ops::RunArgs) -> anyhow::Result<Vec<String>> {
    trace_if!(|| trace!(r#"Ops run args:{:?}"#, &args));

    let command = vec![
      self.metadata.name.clone(),
      String::from("run"),
      args.command,
    ]
    .into_iter()
    .chain(args.passthrough_args.clone())
    .collect();

    trace_if!(|| trace!(r#"Ops run cmd:{:?}"#, command));
    Ok(command)
  }
}
