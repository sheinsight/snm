use clap::Subcommand;

use crate::{
    manage_command::ManageCommands,
    ni::trait_transform_args::{
        AddCommandArgs, DeleteCommandArgs, DlxCommandArgs, ExecCommandArgs, InstallCommandArgs,
        RunCommandArgs, SetCacheArgs,
    },
    semver_manage_command::SemverManageCommands,
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

    #[command(about = "Manage pnpm versions.")]
    Pnpm {
        #[command(subcommand)]
        command: ManageCommands,
    },

    #[command(about = "Manage cwd package.json.")]
    Semver {
        #[command(subcommand)]
        command: SemverManageCommands,
    },

    #[command(about = "Used to install all dependencies for a project.")]
    I(InstallCommandArgs),

    #[command(about = "Alias to snm install --frozen-lockfile.")]
    C(InstallCommandArgs),

    #[command(about = "Add a package and any packages that it depends on.")]
    A(AddCommandArgs),

    #[command(about = "Delete packages from node_modules and from the project's package.json.")]
    D(DeleteCommandArgs),

    #[command(about = "Hot load a package, and runs whatever default command binary it exposes..")]
    X(DlxCommandArgs),

    #[command(about = "Run a command from a local package.")]
    E(ExecCommandArgs),

    #[command(about = "Run some script.")]
    R(RunCommandArgs),

    SetCache(SetCacheArgs),

    Query,

    #[command(about = "write fig spec to autocomplete build directory.")]
    FigSpec,
}
