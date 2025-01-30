use anyhow::bail;
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

    let save_flags = [
      (args.save_prod, "--save-prod"),
      (args.save_dev, "--save-dev"),
      (args.save_peer, "--save-peer"),
      (args.save_optional, "--save-optional"),
    ];

    let active_flags: Vec<&str> = save_flags
      .iter()
      .filter(|(condition, _)| *condition)
      .map(|(_, flag)| *flag)
      .collect();

    if active_flags.len() > 1 {
      bail!("Only one of --save-prod, --save-dev, --save-peer, or --save-optional can be specified at a time");
    }

    if let Some(flag) = active_flags.first() {
      command.push(flag.to_string());
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

    if args.command.is_empty() {
      bail!("Command cannot be empty");
    }

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

  use super::*;
  use crate::{ops::ops::RunArgs, pm::PackageManager};

  #[tokio::test]
  async fn should_parse_npm_command() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["npm", "install", "express"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_frozen() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: true,
    })?;

    assert_eq!(cmd, vec!["npm", "ci"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_save_prod() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: true,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["npm", "install", "express", "--save-prod"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_save_peer() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: true,
      save_dev: false,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["npm", "install", "express", "--save-peer"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_save_dev() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: true,
      save_optional: false,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["npm", "install", "express", "--save-dev"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_save_optional() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: true,
      save_exact: false,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["npm", "install", "express", "--save-optional"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_save_exact() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
      package_spec: vec!["express".to_string()],
      save_prod: false,
      save_peer: false,
      save_dev: false,
      save_optional: false,
      save_exact: true,
      frozen: false,
    })?;

    assert_eq!(cmd, vec!["npm", "install", "express", "--save-exact"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_run() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.run(RunArgs {
      command: "start".to_string(),
      passthrough_args: vec![],
    })?;

    assert_eq!(cmd, vec!["npm", "run", "start", "--"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_run_with_passthrough_args() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;

    let ops = pm.get_ops();

    let cmd = ops.run(RunArgs {
      command: "start".to_string(),
      passthrough_args: vec!["--foo".to_string(), "--bar".to_string()],
    })?;

    assert_eq!(cmd, vec!["npm", "run", "start", "--", "--foo", "--bar"]);

    Ok(())
  }

  #[tokio::test]
  async fn should_fail_when_save_peer_and_optional_are_set() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;
    let ops = pm.get_ops();

    let result = ops.install(InstallArgs {
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

  #[tokio::test]
  async fn should_parse_npm_command_with_empty_command() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;
    let ops = pm.get_ops();

    let result = ops.run(RunArgs {
      command: "".to_string(),
      passthrough_args: vec!["--foo".to_string(), "--bar".to_string()],
    });

    assert!(result.is_err());
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_remove_multiple_packages() -> anyhow::Result<()> {
    let pm = PackageManager::from_str("npm@8.0.0")?;
    let ops = pm.get_ops();

    let cmd = ops.remove(RemoveArgs {
      package_spec: vec!["express".to_string(), "lodash".to_string()],
    })?;

    assert_eq!(cmd, vec!["npm", "uninstall", "express", "lodash"]);
    Ok(())
  }

  #[tokio::test]
  async fn should_parse_npm_command_with_special_characters_in_package_spec() -> anyhow::Result<()>
  {
    let pm = PackageManager::from_str("npm@8.0.0")?;
    let ops = pm.get_ops();

    let cmd = ops.install(InstallArgs {
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
      vec!["npm", "install", "@scope/package package-with-space"]
    );
    Ok(())
  }
}
