use clap::Subcommand;
use snm_node_version::{DefaultArgs, InstallArgs, ListArgs, UninstallArgs};

#[derive(Subcommand, Debug)]
pub enum ManageCommands {
    /// Set default npm version
    Default(DefaultArgs),
    /// Install npm
    Install(InstallArgs),
    /// Uninstall npm
    Uninstall(UninstallArgs),
    /// List installed npm versions
    List(ListArgs),
    // List remote npm versions
    // ListRemote {
    //     #[arg(
    //         short,
    //         long,
    //         // default_value = "true",
    //         help = "List all available versions"
    //     )]
    //     all: bool,
    // },
}
