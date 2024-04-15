use clap::Parser;
use snm_core::model::SnmError;

#[derive(Parser, Debug)]
pub struct AddCommandArgs {
    #[arg(help = "The package spec to install.")]
    pub package_spec: String,
    #[arg(short = 'P', long, help = "Save to dependencies")]
    pub save_prod: bool,
    #[arg(long, help = "Save to peerDependencies")]
    pub save_peer: bool,
    #[arg(short = 'D', long, help = "Save to devDependencies")]
    pub save_dev: bool,
    #[arg(short = 'O', long, help = "Save to optionalDependencies")]
    pub save_optional: bool,
    #[arg(short = 'E', long, help = "Save to exact version")]
    pub save_exact: bool,
    #[arg(short = 'g', long, help = "Install globally")]
    pub global: bool,
}

#[derive(Parser, Debug)]
pub struct InstallCommandArgs {
    #[arg(
        short,
        long,
        help = "If true, pnpm skips lockfile generation, failing install if the lockfile is out of sync or missing."
    )]
    pub frozen_lockfile: bool,
}

pub trait CommandArgsCreatorTrait {
    fn get_install_command(&self, args: InstallCommandArgs) -> Result<Vec<String>, SnmError>;

    fn get_add_command<'a>(&self, args: AddCommandArgs) -> Result<Vec<String>, SnmError>;
}
