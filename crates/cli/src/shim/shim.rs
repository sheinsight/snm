use std::process::{Command, Stdio};

use colored::*;
use snm_core::{
    model::{
        dispatch_manage::DispatchManage, snm_error::handle_snm_error, trait_manage::ManageTrait,
    },
    println_success,
};

pub async fn launch_shim(manager: Box<dyn ManageTrait>) {
    let dispatcher = DispatchManage::new(manager);
    match dispatcher.proxy_process().await {
        Ok((v, bin_path_buf)) => {
            println_success!("Use Node {}. ", v.green());
            let args: Vec<String> = std::env::args().skip(1).collect();
            let _ = Command::new(&bin_path_buf)
                .args(&args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .stdin(Stdio::inherit())
                .spawn()
                .and_then(|process| process.wait_with_output());
        }
        Err(error) => {
            handle_snm_error(error);
        }
    }
}
