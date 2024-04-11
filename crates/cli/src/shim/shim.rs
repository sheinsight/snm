use colored::*;
use snm_core::{
    exec_proxy_child_process,
    model::{
        manager::{ManagerTrait, ManagerTraitDispatcher},
        snm_error::handle_snm_error,
    },
    println_success,
};

pub async fn launch_shim(manager: Box<dyn ManagerTrait>) {
    let dispatcher = ManagerTraitDispatcher::new(manager);
    match dispatcher.launch_proxy().await {
        Ok((v, bin_path_buf)) => {
            println_success!("Use Node {}. ", v.green());
            exec_proxy_child_process!(&bin_path_buf);
        }
        Err(error) => {
            handle_snm_error(error);
        }
    }
}
