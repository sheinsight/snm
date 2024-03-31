use snm_core::config::init_config;
use snm_npm::snm_npm::SnmNpm;

mod commands;

#[tokio::main]
async fn main() {
    env_logger::init();
    init_config().unwrap();
    // todo!("execute_command")
    let snm_npm = SnmNpm::new(None);
    snm_npm.install("6.14.4").await.unwrap();

    // snm_npm.uninstall("6.14.4").unwrap();

    // snm_npm.list().unwrap();

    // snm_npm.default("6.14.4").await.unwrap();

    // SnmNpm;
}
