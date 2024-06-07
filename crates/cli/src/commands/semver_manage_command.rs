use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SemverManageCommands {
    Prerelease {
        #[arg(help = "Need to set the npm version number as the default version.")]
        version: String,
    },
    Release {
        #[arg(help = "The version number of npm to be installed")]
        version: String,
    },
}
