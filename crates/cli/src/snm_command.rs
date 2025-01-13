use clap::Subcommand;
use snm_pm::{
  factory::PackageManagerFactoryCommands,
  ops::ops::{AddArgs, InstallArgs, RemoveArgs},
};

use super::manage_command::NodeManageCommands;

#[derive(Subcommand, Debug)]
pub enum SnmCommands {
  #[command(about = "Used to install all dependencies for a project.")]
  I(InstallArgs),

  #[command(about = "Alias to snm install --frozen-lockfile.")]
  C(InstallArgs),

  #[command(about = "Add a package and any packages that it depends on.")]
  A(AddArgs),

  #[command(about = "Remove a package.")]
  D(RemoveArgs),

  // #[command(about = "Hot load a package, and runs whatever default command binary it exposes..")]
  // X(XArgs),

  // #[command(about = "Run a command from a local package.")]
  // E(EArgs),

  // #[command(about = "Run some script.")]
  // R(RArgs),
  #[command(about = "Manage node versions.")]
  Node {
    #[command(subcommand)]
    command: NodeManageCommands,
  },

  #[command(about = "Manage package manager.")]
  Pnpm {
    #[command(subcommand)]
    command: PackageManagerFactoryCommands,
  },

  #[command(about = "Manage yarn.")]
  Yarn {
    #[command(subcommand)]
    command: PackageManagerFactoryCommands,
  },

  // #[command(about = "Manage yarn berry.")]
  // YarnBerry {
  //     #[command(subcommand)]
  //     command: PackageManagerFactoryCommands,
  // },
  #[command(about = "Manage npm.")]
  Npm {
    #[command(subcommand)]
    command: PackageManagerFactoryCommands,
  },

  #[command(about = "write fig spec to autocomplete build directory.")]
  FigSpec,

  #[command(name = "setup", about = "Setup snm.")]
  SetUp,
}
