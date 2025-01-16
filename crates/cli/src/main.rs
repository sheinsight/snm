use std::env::current_dir;

// use clap::Parser;
// use snm::{execute_cli::execute_cli, SnmCli};
use snm_config::SnmConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let dir = current_dir()?;

  let snm_config = SnmConfig::from(dir)?;

  let nm = snm_node::factory::NodeFactory::new(&snm_config);
  nm.install(snm_node::factory::InstallArgs {
    version: "20.0.0".to_string(),
  })
  .await
  // match run().await {
  //   Ok(_) => ExitCode::SUCCESS,
  //   Err(e) => {
  //     eprintln!("Error: {}", e);
  //     ExitCode::FAILURE
  //   }
  // }
}

// async fn run() -> anyhow::Result<()> {
//   let dir = current_dir()?;

//   let snm_config = SnmConfig::from(dir)?;

//   // let cli = SnmCli::parse();

//   // execute_cli(cli, snm_config).await

//   let nm = snm_node::factory::NodeFactory::new(&snm_config);
//   nm.install(snm_node::factory::InstallArgs {
//     version: "20.0.0".to_string(),
//   })
//   .await
// }
