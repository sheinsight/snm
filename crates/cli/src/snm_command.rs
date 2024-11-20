use super::manage_command::ManageCommands;
use clap::Subcommand;
use snm_package_json::ops::ops::{AddArgs, InstallArgs, RemoveArgs};

#[derive(Subcommand, Debug)]
pub enum SnmCommands {
    #[command(about = "Used to install all dependencies for a project.")]
    I(InstallArgs),

    #[command(about = "Alias to snm install --frozen-lockfile.")]
    C(InstallArgs),

    #[command(about = "Add a package and any packages that it depends on.")]
    A(AddArgs),

    #[command(about = "Remove a package.")]
    D(RemoveArgs),

    // #[command(about = "Hot load a package, and runs whatever default command binary it exposes..")]
    // X(XArgs),

    // #[command(about = "Run a command from a local package.")]
    // E(EArgs),

    // #[command(about = "Run some script.")]
    // R(RArgs),
    #[command(about = "Manage node versions.")]
    Node {
        #[command(subcommand)]
        command: ManageCommands,
    },

    #[command(about = "write fig spec to autocomplete build directory.")]
    FigSpec,
}
