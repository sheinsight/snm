use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::node_manager::node_manager::{ListArgs, ListRemoteArgs, NodeManager};
use crate::snm_command::SnmCommands;
use crate::SnmCli;
use snm_atom::node_atom::NodeAtom;
use snm_config::SnmConfig;
use snm_ni::trait_transform::IArgs;
use snm_ni::{CommandArgsCreatorTrait, NpmArgsTransform, PnpmArgsTransform, YarnArgsTransform};
use snm_package_json::package_json::PackageJson;
use snm_utils::exec::exec_cli;
use snm_utils::snm_error::SnmError;

fn create_transform(name: &str, version: &str) -> anyhow::Result<Box<dyn CommandArgsCreatorTrait>> {
    match name {
        "npm" => Ok(Box::new(NpmArgsTransform {})),
        "pnpm" => Ok(Box::new(PnpmArgsTransform {})),
        "yarn" => Ok(Box::new(YarnArgsTransform {
            version: version.to_string(),
        })),
        _ => anyhow::bail!("Unsupported package manager: {}", name),
    }
}

// fn handle_command(transform: &dyn CommandArgsCreatorTrait, command: SnmCommands) -> Vec<String> {
//     match command {
//         // snm command start
//         SnmCommands::I(args) => transform.i(args),
//         SnmCommands::C(_) => transform.i(IArgs {
//             frozen_lockfile: true,
//         }),
//         SnmCommands::A(args) => transform.a(args),
//         SnmCommands::X(args) => transform.x(args),
//         SnmCommands::E(args) => transform.e(args),
//         SnmCommands::R(args) => transform.r(args),
//         _ => unreachable!("unreachable"),
//     }
// }

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
    match cli.command {
        // manage start
        SnmCommands::Node { command } => {
            let node_atom = NodeAtom::new(snm_config);
            let node_manager = NodeManager::new(&node_atom);
            match command {
                ManageCommands::Default { version } => {
                    node_manager.set_default(version.as_str()).await?;
                }
                ManageCommands::Install { version } => {
                    node_manager.install(version.as_str()).await?;
                }
                ManageCommands::Uninstall { version } => {
                    node_manager.un_install(version.as_str()).await?;
                }
                ManageCommands::List { offline } => {
                    node_manager.list(ListArgs { offline }).await?;
                }
                ManageCommands::ListRemote { all } => {
                    node_manager.list_remote(ListRemoteArgs { all }).await?;
                }
            }
        }
        // manage end
        SnmCommands::I(_) | SnmCommands::C(_) | SnmCommands::A(_) => {
            if let Some(package_json) = PackageJson::from(snm_config.workspace) {
                // let (pm_name, pm_version) = package_json
                //     .get_pm()
                //     .map(|pm| (pm.name().to_string(), pm.version().to_string()))
                //     .ok_or(SnmError::NotFondPackageManagerConfigError {})?;

                if let Some(pm) = package_json.get_pm() {
                    let command = match cli.command {
                        // SnmCommands::Node { command } => todo!(),
                        SnmCommands::I(iargs) => pm.install(iargs),
                        SnmCommands::C(iargs) => pm.install(iargs),
                        SnmCommands::A(aargs) => pm.add(aargs),
                        // SnmCommands::FigSpec => todo!(),
                        _ => unreachable!("unreachable"),
                    }?;
                    // println!("{:?}", command.iter().join(" "));

                    let args = command.iter().skip(1).collect::<Vec<_>>();

                    println!("{} {:?}", pm.name(), args);

                    exec_cli(vec![], pm.name(), args)?;
                }

                // package_json.get_package_manager()

                // let transform = create_transform(&pm_name, &pm_version)?;

                // let args = handle_command(&*transform.as_ref(), cli.command);

                // exec_cli(vec![], pm_name, args)?;
            }
        }

        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
    }

    Ok(())
}
