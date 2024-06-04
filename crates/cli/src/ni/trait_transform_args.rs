use clap::Parser;

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
        long,
        help = "If true, pnpm skips lockfile generation, failing install if the lockfile is out of sync or missing."
    )]
    pub frozen_lockfile: bool,
}

#[derive(Parser, Debug)]
pub struct DeleteCommandArgs {
    #[arg(
        help = "If true, pnpm skips lockfile generation, failing install if the lockfile is out of sync or missing."
    )]
    pub package_spec: String,
}

#[derive(Parser, Debug)]
pub struct DlxCommandArgs {
    #[arg(help = "The package spec to install.")]
    pub package_spec: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct ExecCommandArgs {
    #[arg(help = "The package spec to install.")]
    pub package_spec: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct RunCommandArgs {
    #[arg(help = "script file path")]
    pub args: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct SetCacheArgs {
    #[arg(help = "cache dir path")]
    pub cache_path: String,
}

pub trait CommandArgsCreatorTrait {
    fn get_install_command(&self, args: InstallCommandArgs) -> Vec<String>;

    fn get_add_command<'a>(&self, args: AddCommandArgs) -> Vec<String>;

    fn get_delete_command(&self, args: DeleteCommandArgs) -> Vec<String>;

    fn get_dlx_command(&self, args: DlxCommandArgs) -> Vec<String>;

    fn get_exec_command(&self, args: ExecCommandArgs) -> Vec<String>;

    fn get_run_command(&self, args: RunCommandArgs) -> Vec<String>;

    fn get_set_cache_command(&self, args: SetCacheArgs) -> Vec<String>;
}
