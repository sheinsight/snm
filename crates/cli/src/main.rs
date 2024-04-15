use clap::{command, CommandFactory, Parser};
use manage_command::ManageCommands;
use ni::{
    npm_args::NpmArgsTransform, pnpm_args::PnpmArgsTransform,
    trait_transform_args::CommandArgsCreatorTrait, yarn_args::YarnArgsTransform,
    yarnpkg_args::YarnPkgArgsTransform,
};
use semver::Version;
use snm_command::SnmCommands;
use snm_core::{
    config::SnmConfig,
    model::{
        dispatch_manage::DispatchManage, package_json::PackageManager, snm_error::handle_snm_error,
        trait_manage::ManageTrait, PackageJson, SnmError,
    },
};
use snm_node::snm_node::SnmNode;
use snm_npm::snm_npm::SnmNpm;
use snm_pnpm::snm_pnpm::SnmPnpm;
use snm_yarn::{snm_yarn::SnmYarn, snm_yarnpkg::SnmYarnPkg};
use std::process::{Command, Stdio};
mod manage_command;
mod ni;
mod snm_command;

#[derive(Parser, Debug)]
struct SnmCli {
    #[command(subcommand)]
    command: SnmCommands,
}

#[tokio::main]
async fn main() -> Result<(), SnmError> {
    SnmConfig::new().init()?;

    if let Err(error) = execute_cli().await {
        handle_snm_error(error);
    }

    Ok(())
}

async fn execute_cli() -> Result<(), SnmError> {
    let cli = SnmCli::parse();

    match cli.command {
        // manage start
        SnmCommands::Yarn { command } => {
            exec_manage(command, Box::new(SnmYarn::new())).await?;
        }
        SnmCommands::Pnpm { command } => {
            exec_manage(command, Box::new(SnmPnpm::new())).await?;
        }
        SnmCommands::Npm { command } => {
            exec_manage(command, Box::new(SnmNpm::new())).await?;
        }
        SnmCommands::Node { command } => {
            exec_manage(command, Box::new(SnmNode::new())).await?;
        }
        // manage end

        // snm command start
        SnmCommands::Install(args) => {
            execute_command(|creator| creator.get_install_command(args)).await?;
        }
        SnmCommands::Add(args) => {
            execute_command(|creator| creator.get_add_command(args)).await?;
        }

        SnmCommands::Query => todo!(),
        SnmCommands::Delete => todo!(),
        // snm command end
        SnmCommands::FigSpec => clap_complete::generate(
            clap_complete_fig::Fig,
            &mut SnmCli::command(),
            "snm",
            &mut std::io::stdout(),
        ),

        SnmCommands::CI(_) => todo!(),
    }
    Ok(())
}

pub async fn exec_manage(
    command: ManageCommands,
    manager: Box<dyn ManageTrait>,
) -> Result<(), SnmError> {
    let dispatcher = DispatchManage::new(manager);

    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();

    match command {
        ManageCommands::Default { version } => {
            dispatcher.set_default(&trim_version(version)).await?
        }
        ManageCommands::Install { version } => {
            dispatcher.install(&trim_version(version)).await?;
        }
        ManageCommands::Uninstall { version } => {
            dispatcher.un_install(&trim_version(version)).await?;
        }
        ManageCommands::List => {
            dispatcher.list().await?;
        }
        ManageCommands::ListRemote { all } => {
            dispatcher.list_remote(all).await?;
        }
    };
    Ok(())
}

async fn execute_command<F>(get_command_args: F) -> Result<(), SnmError>
where
    F: FnOnce(&dyn CommandArgsCreatorTrait) -> Result<Vec<String>, SnmError>,
{
    let package_manager = get_package_manager()?;

    let manager = get_manage(&package_manager).await?;

    let command_args_creator = get_command_args_creator(&package_manager)?;

    let args = get_command_args(command_args_creator.as_ref())?;

    let dispatcher = DispatchManage::new(manager);
    let (_, bin_path_buf) = dispatcher.ensure_strict_package_manager().await?;

    println!("{} {:?}", bin_path_buf.display(), args);

    Command::new(bin_path_buf.display().to_string())
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .and_then(|process| process.wait_with_output())?;
    Ok(())
}

fn get_package_manager() -> Result<PackageManager, SnmError> {
    let package_json = PackageJson::from_dir_path(None)?;
    package_json.parse_package_manager()
}

fn get_command_args_creator(
    package_manager: &PackageManager,
) -> Result<Box<dyn CommandArgsCreatorTrait>, SnmError> {
    let x: Box<dyn CommandArgsCreatorTrait> = match package_manager.name.as_str() {
        "npm" => Box::new(NpmArgsTransform {}),
        "pnpm" => Box::new(PnpmArgsTransform {}),
        "yarn" => {
            if get_is_less_2(&package_manager.version)? {
                Box::new(YarnArgsTransform {})
            } else {
                Box::new(YarnPkgArgsTransform {})
            }
        }
        _ => panic!("Unsupported package manager"),
    };
    Ok(x)
}

async fn get_manage(package_manager: &PackageManager) -> Result<Box<dyn ManageTrait>, SnmError> {
    let manager: Box<dyn ManageTrait> = match package_manager.name.as_str() {
        "npm" => {
            let manager = SnmNpm::new();
            Box::new(manager)
        }
        "pnpm" => Box::new(SnmPnpm::new()),
        "yarn" => {
            if get_is_less_2(&package_manager.version)? {
                Box::new(SnmYarn::new())
            } else {
                Box::new(SnmYarnPkg::new())
            }
        }
        _ => {
            return Err(SnmError::UnsupportedPackageManager {
                name: package_manager.name.to_string(),
                version: package_manager.version.to_string(),
            })
        }
    };
    Ok(manager)
}

fn get_is_less_2(v: &str) -> Result<bool, SnmError> {
    let ver = Version::parse(v)?;
    let is_less_2 = ver < Version::parse("2.0.0")?;
    Ok(is_less_2)
}
