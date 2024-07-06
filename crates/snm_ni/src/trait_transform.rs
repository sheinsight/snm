use clap::Parser;

#[derive(Parser, Debug)]
pub struct AArgs {
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
pub struct IArgs {
    #[arg(
        long,
        help = "If true, pnpm skips lockfile generation, failing install if the lockfile is out of sync or missing."
    )]
    pub frozen_lockfile: bool,
}

#[derive(Parser, Debug)]
pub struct XArgs {
    #[arg(help = "The package spec to install.")]
    pub package_spec: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct EArgs {
    #[arg(help = "The package spec to install.")]
    pub package_spec: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct RArgs {
    #[arg(help = "script file path")]
    pub args: Vec<String>,
}

pub trait CommandArgsCreatorTrait {
    fn i(&self, args: IArgs) -> Vec<String>;

    fn a<'a>(&self, args: AArgs) -> Vec<String>;

    fn x(&self, args: XArgs) -> Vec<String>;

    fn e(&self, args: EArgs) -> Vec<String>;

    fn r(&self, args: RArgs) -> Vec<String>;
}
