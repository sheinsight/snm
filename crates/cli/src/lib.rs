use clap::{command, Parser};
use snm_command::SnmCommands;

pub mod bump;
pub mod fig;

pub mod manage_command;
pub mod ni;
pub mod snm_command;

pub mod execute_cli;

#[derive(Parser, Debug)]
pub struct SnmCli {
    #[command(subcommand)]
    pub command: SnmCommands,
}
