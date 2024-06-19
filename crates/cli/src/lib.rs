use clap::{command, Parser};
use snm_command::SnmCommands;

pub mod fig;
pub mod manage_command;
pub mod snm_command;

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
