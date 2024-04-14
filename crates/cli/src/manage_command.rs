use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ManageCommands {
    /// Set default npm version
    Default {
        #[arg(help = "Need to set the npm version number as the default version.")]
        version: String,
    },
    /// Install npm
    Install {
        #[arg(help = "The version number of npm to be installed")]
        version: String,
    },
    /// Uninstall npm
    Uninstall {
        #[arg(help = "The npm version number to be deleted")]
        version: String,
    },
    /// List installed npm versions
    List,
    /// List remote npm versions
    ListRemote {
        #[arg(
            short,
            long,
            // default_value = "true",
            help = "List all available versions"
        )]
        all: bool,
    },
}
