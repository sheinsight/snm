use clap::{command, Parser};
use commands::snm_command::SnmCommands;

pub mod bump;
pub mod commands;
pub mod fig;
pub mod ni;

pub mod execute_cli;

#[derive(Parser, Debug)]
#[clap(
    name = "snm",
    version = "1.0.0",
    author = "Author ityuany <519495771@qq.com>"
)]
pub struct SnmCli {
    #[command(subcommand)]
    pub command: SnmCommands,
}
