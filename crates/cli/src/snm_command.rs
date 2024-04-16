use clap::Subcommand;

use crate::{
    manage_command::ManageCommands,
    ni::trait_transform_args::{AddCommandArgs, DeleteCommandArgs, InstallCommandArgs},
};

#[derive(Subcommand, Debug)]
pub enum SnmCommands {
    #[command(about = "Manage node versions.")]
    Node {
        #[command(subcommand)]
        command: ManageCommands,
    },
    #[command(about = "Manage npm versions.")]
    Npm {
        #[command(subcommand)]
        command: ManageCommands,
    },
    #[command(about = "Manager yarn versions.")]
    Yarn {
        #[command(subcommand)]
        command: ManageCommands,
    },
    #[command(about = "Manage pnpm versions.")]
    Pnpm {
        #[command(subcommand)]
        command: ManageCommands,
    },
    #[command(alias = "i", about = "Used to install all dependencies for a project.")]
    Install(InstallCommandArgs),
    #[command(about = "Alias to snm install --frozen-lockfile.")]
    CI(InstallCommandArgs),
    #[command(about = "Installs a package and any packages that it depends on.")]
    Add(AddCommandArgs),
    #[command(about = "Delete packages from node_modules and from the project's package.json.")]
    Delete(DeleteCommandArgs),

    Bump,

    Query,
    FigSpec,
}
