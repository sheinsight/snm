use clap::Subcommand;
use serde::Serialize;
use snm_pm::ops::ops::{InstallArgs, RemoveArgs, RunArgs};

use super::manage_command::NodeManageCommands;

#[derive(Subcommand, Debug, Serialize)]
pub enum SnmCommands {
  #[command(
    visible_aliases = ["i"],
    about = "Used to install all dependencies for a project.",
    // visible_alias = "i"
  )]
  Install(InstallArgs),

  #[command(
    visible_aliases=["un"],
    about = "Remove a package.",
  )]
  Uninstall(RemoveArgs),

  #[command(about = "Run a command.")]
  Run(RunArgs),

  #[command(about = "Manage node versions.")]
  Node {
    #[command(subcommand)]
    command: NodeManageCommands,
  },

  #[command(name = "setup", about = "Setup snm." , visible_aliases = ["st"])]
  SetUp,
  // #[command(name = "ai-commit", about = "Commit ai.")]
  // AiCommit,
}
