use anyhow::bail;
use snm_utils::trace_if;
use tracing::trace;

use super::{Command, InstallArgs, RemoveArgs, RunArgs};

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

impl Command for PnpmCommandLine {
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

    if let Some(flag) = self.get_save_flag(&args)? {
      command.push(flag);
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

  fn run(&self, args: RunArgs) -> anyhow::Result<Vec<String>> {
    trace_if!(|| trace!(r#"Ops run args:{:?}"#, &args));

    let command = vec![self.name.clone(), String::from("run"), args.command]
      .into_iter()
      .chain(args.passthrough_args.clone())
      .collect();

    trace_if!(|| trace!(r#"Ops run cmd:{:?}"#, command));

    Ok(command)
  }

  fn get_save_flag(&self, args: &InstallArgs) -> anyhow::Result<Option<String>> {
    let save_flags = [
      (args.save_prod, "--save-prod"),
      (args.save_dev, "--save-dev"),
      (args.save_peer, "--save-peer"),
      (args.save_optional, "--save-optional"),
    ];

    let active_flags: Vec<_> = save_flags
      .iter()
      .filter(|(condition, _)| *condition)
      .collect();

    if active_flags.len() > 1 {
      bail!(
        "Only one of --save-prod, --save-dev, --save-peer, or --save-optional can be specified at a time"
      );
    }

    Ok(active_flags.first().map(|(_, flag)| flag.to_string()))
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[tokio::test]
  async fn should_parse_pnpm_command() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["pnpm", "add", "express"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_frozen() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec![],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: true,
    })?;

    assert_eq!(cmd, vec!["pnpm", "install", "--frozen-lockfile"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_save_prod() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: true,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["pnpm", "add", "express", "--save-prod"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_save_peer() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: true,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["pnpm", "add", "express", "--save-peer"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_save_dev() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: true,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["pnpm", "add", "express", "--save-dev"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_save_optional() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: true,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["pnpm", "add", "express", "--save-optional"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_save_exact() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: true,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["pnpm", "add", "express", "--save-exact"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_run() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.run(RunArgs {
      command: "start".to_string(),
      passthrough_args: vec![],
    })?;

    assert_eq!(cmd, vec!["pnpm", "run", "start"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_run_with_passthrough_args() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.run(RunArgs {
      command: "start".to_string(),
      passthrough_args: vec!["--foo".to_string(), "--bar".to_string()],
    })?;

    assert_eq!(cmd, vec!["pnpm", "run", "start", "--foo", "--bar"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_fail_when_save_peer_and_optional_are_set() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let result = command.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: true,
      save_dev: false,
      save_optional: true,
      save_exact: false,
      frozen: false,
    });

    assert!(result.is_err());
    Ok(())
  }

  // #[tokio::test]
  // async fn should_parse_pnpm_command_with_empty_command() -> anyhow::Result<()> {
  //   let pm = PackageManager::from_str("pnpm@8.0.0")?;
  //   let ops = pm.get_ops();

  //   let result = ops.run(RunArgs {
  //     command: "".to_string(),
  //     passthrough_args: vec!["--foo".to_string(), "--bar".to_string()],
  //   });

  //   assert!(result.is_err());
  //   Ok(())
  // }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_remove_multiple_packages() -> anyhow::Result<()> {
    let command = PnpmCommandLine::new();

    let cmd = command.remove(RemoveArgs {
      package_spec: vec!["express".to_string(), "lodash".to_string()],
    })?;

    assert_eq!(cmd, vec!["pnpm", "remove", "express", "lodash"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_pnpm_command_with_special_characters_in_package_spec() -> anyhow::Result<()>
  {
    let command = PnpmCommandLine::new();

    let cmd = command.install(InstallArgs {
      package_spec: vec![
        "@scope/package".to_string(),
        "package-with-space".to_string(),
      ],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(
      cmd,
      vec!["pnpm", "add", "@scope/package package-with-space"]
    );
    Ok(())
  }
}
