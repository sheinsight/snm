use snm_core::{config::init_config, model::snm_error::handle_snm_error};
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};
use snm_yarn::snm_yarn::SnmYarn;

mod commands;

#[tokio::main]
async fn main() {
    env_logger::init();
    init_config().unwrap();

    let snm_npm = SnmNpm::new();
    let snm_yarn = SnmYarn::new();

    // snm_yarn.install("1.22.4").await.unwrap();

    snm_yarn.install("4.0.2").await.unwrap();

    // todo!("execute_command")

    // match snm_npm.use_bin("npm", None).await {
    //     Ok(_) => todo!(),
    //     Err(error) => handle_snm_error(error),
    // }

    // snm_npm.install("6.14.4").await.unwrap();

    // snm_npm.uninstall("6.14.4").unwrap();

    // snm_npm.list().unwrap();

    // snm_npm.default("6.14.4").await.unwrap();

    // SnmNpm;
}
