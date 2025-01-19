use std::fmt::Display;

use clap::{command, crate_authors, crate_name, crate_version, Parser};
use serde::Serialize;

use crate::snm_command::SnmCommands;

#[derive(Parser, Debug, Serialize)]
#[
    command(
        name = crate_name!(),
        author = crate_authors!(),
        version = crate_version!(),
        about = "snm = ni + fnm + corepack",
        disable_version_flag = true,
        disable_help_subcommand = true
    )
]
pub struct SnmCli {
  #[command(subcommand)]
  pub command: SnmCommands,
  #[arg(
        short = 'v',
        long = "version",
        action = clap::ArgAction::Version
    )]
  version: Option<bool>,
}

impl Display for SnmCli {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Ok(json) = serde_json::to_string_pretty(self) {
      return write!(f, "{}", json);
    }
    write!(f, "{:?}", self)
  }
}
