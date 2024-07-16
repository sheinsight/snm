use clap::Subcommand;
use snm_ni::trait_transform::{AArgs, EArgs, IArgs, RArgs, XArgs};

use super::manage_command::ManageCommands;

#[derive(Subcommand, Debug)]
pub enum SnmCommands {
    #[command(about = "Manage node versions.")]
    Node {
        #[command(subcommand)]
        command: ManageCommands,
    },

    #[command(about = "Used to install all dependencies for a project.")]
    I(IArgs),

    #[command(about = "Alias to snm install --frozen-lockfile.")]
    C(IArgs),

    #[command(about = "Add a package and any packages that it depends on.")]
    A(AArgs),

    #[command(about = "Hot load a package, and runs whatever default command binary it exposes..")]
    X(XArgs),

    #[command(about = "Run a command from a local package.")]
    E(EArgs),

    #[command(about = "Run some script.")]
    R(RArgs),

    #[command(about = "write fig spec to autocomplete build directory.")]
    FigSpec,
}
