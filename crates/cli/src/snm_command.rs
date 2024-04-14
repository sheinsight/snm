use clap::Subcommand;

use crate::{
    manage_command::ManageCommands,
    ni::trait_transform_args::{AddCommandArgs, InstallCommandArgs},
};

#[derive(Subcommand, Debug)]
pub enum SnmCommands {
    /// Manager node versions
    Node {
        #[command(subcommand)]
        command: ManageCommands,
    },
    /// Manager npm versions
    Npm {
        #[command(subcommand)]
        command: ManageCommands,
    },
    /// Manager yarn versions
    Yarn {
        #[command(subcommand)]
        command: ManageCommands,
    },
    /// Manager pnpm versions
    Pnpm {
        #[command(subcommand)]
        command: ManageCommands,
    },
    Install(InstallCommandArgs),
    Add(AddCommandArgs),
    Del,
    Query,
    FigSpec,
}
