use clap::Parser;
use serde::Serialize;
#[derive(Parser, Debug, Clone, Serialize)]
pub struct InstallArgs {
  #[arg(
    long,
    help = "If true, pnpm skips lockfile generation, failing install if the lockfile is out of sync or missing."
  )]
  pub frozen_lockfile: bool,
}

#[derive(Parser, Debug, Clone, Serialize)]
pub struct AddArgs {
  #[arg(help = "The package spec to install.")]
  pub package_spec: Vec<String>,
  #[arg(short = 'P', long, help = "Save into dependencies")]
  pub save_prod: bool,
  #[arg(long, help = "Save into peerDependencies")]
  pub save_peer: bool,
  #[arg(short = 'D', long, help = "Save into devDependencies")]
  pub save_dev: bool,
  #[arg(short = 'O', long, help = "Save into optionalDependencies")]
  pub save_optional: bool,
  #[arg(short = 'E', long, help = "Use exact version to save")]
  pub save_exact: bool,
}

#[derive(Parser, Debug, Clone, Serialize)]
pub struct RemoveArgs {
  #[arg(help = "The package spec to remove.")]
  pub package_spec: Vec<String>,
}

pub trait PackageManagerOps {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>>;

  fn add(&self, args: AddArgs) -> anyhow::Result<Vec<String>>;

  fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>>;
}
