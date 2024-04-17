use clap::Subcommand;

use crate::{
    manage_command::ManageCommands,
    ni::trait_transform_args::{
        AddCommandArgs, DeleteCommandArgs, DlxCommandArgs, InstallCommandArgs,
    },
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
    #[command(about = "Add a package and any packages that it depends on.")]
    Add(AddCommandArgs),
    #[command(about = "Delete packages from node_modules and from the project's package.json.")]
    Delete(DeleteCommandArgs),
    #[command(about = "bump version.")]
    Bump,
    #[command(
        about = "Fetches a package from the registry without installing it as a dependency, hotloads it, and runs whatever default command binary it exposes.."
    )]
    Dlx(DlxCommandArgs),
    #[command(about = "Run a command from a local package.")]
    Exec,

    Query,
    #[command(about = "write fig spec to autocomplete build directory.")]
    FigSpec,
}
