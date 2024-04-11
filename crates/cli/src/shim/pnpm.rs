use colored::*;
use snm_core::{
    exec_proxy_child_process,
    model::{manager::ManagerTraitDispatcher, snm_error::handle_snm_error},
    println_success,
};
use snm_npm::snm_npm::SnmNpm;

// mod shim;

#[tokio::main]
async fn main() {
    env_logger::init();

    let m = ManagerTraitDispatcher::new(Box::new(SnmNpm::new("pnpm")));

    match m.launch_proxy().await {
        Ok((v, bin_path_buf)) => {
            println_success!("Use Node {}. ", v.green());
            exec_proxy_child_process!(&bin_path_buf);
        }
        Err(error) => {
            handle_snm_error(error);
        }
    }
}
