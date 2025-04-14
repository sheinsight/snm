use clap::Parser;
use cli::SnmCli;
use snm_utils::log::init_snm_log;

mod cli;
mod manage_command;
mod node;
mod package_manager;
mod snm_command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_snm_log()?;

  SnmCli::parse().exec().await
}

// async fn xx() -> anyhow::Result<()> {
//   let args = snm_pm::ops::ops::InstallArgs {
//     package_spec: vec![],
//     frozen: true,
//     save_prod: false,
//     save_peer: false,
//     save_dev: false,
//     save_optional: false,
//     save_exact: false,
//   };

//   SnmCli::from(snm_command::SnmCommands::Install(args))
//     .exec()
//     .await?;

//   let args = snm_pm::ops::ops::RunArgs {
//     command: "build".to_string(),
//     passthrough_args: vec![],
//   };

//   SnmCli::from(snm_command::SnmCommands::Run(args))
//     .exec()
//     .await?;

//   Ok(())
// }
