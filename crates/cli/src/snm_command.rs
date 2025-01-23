use clap::Subcommand;
use serde::Serialize;
use snm_pm::ops::ops::{InstallArgs, RemoveArgs};

use super::manage_command::NodeManageCommands;

#[derive(Subcommand, Debug, Serialize)]
pub enum SnmCommands {
  #[command(
    visible_aliases = ["i"],
    about = "Used to install all dependencies for a project.",
    // visible_alias = "i"
  )]
  Install(InstallArgs),

  // #[command(about = "Alias to snm install --frozen-lockfile.")]
  // C(InstallArgs),
  // #[command(about = "Add a package and any packages that it depends on.")]
  // Add(AddArgs),
  #[command(
    visible_aliases=["un"],
    about = "Remove a package.",
  )]
  Uninstall(RemoveArgs),

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

  #[command(about = "write fig spec to autocomplete build directory.")]
  FigSpec,

  #[command(name = "setup", about = "Setup snm.")]
  SetUp,
}
