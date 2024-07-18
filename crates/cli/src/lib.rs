use clap::{command, crate_authors, crate_name, crate_version, Parser};
use snm_command::SnmCommands;

pub mod fig;
pub mod manage_command;
pub mod node_manager;
pub mod snm_command;

pub mod execute_cli;

#[derive(Parser, Debug)]
#[
    command(
        name = crate_name!(),
        author = crate_authors!(),
        version = crate_version!(),
        about = "snm = ni + fnm + corepack"
    )
]
pub struct SnmCli {
    #[command(subcommand)]
    pub command: SnmCommands,
}
