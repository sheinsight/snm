use super::snm::SnmTrait;
use snm_core::{exec_proxy_child_process, model::SnmError};

pub struct Npm {}

impl Npm {
    pub fn new(v: &str) -> Self {
        Self {}
    }
}

impl SnmTrait for Npm {
    fn install(&self, args: super::snm::InstallCommandArgs) -> Result<Vec<String>, SnmError> {
        let process_args = if args.frozen_lockfile {
            vec!["ci".to_string()]
        } else {
            vec!["install".to_string()]
        };

        // exec_child_process!(&self.bin, process_args);
        Ok(process_args)
    }

    fn add(&self, args: super::snm::AddCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["add".to_string(), args.package_spec];
        if args.save_prod {
            process_args.push("--save".to_string());
        } else if args.save_dev {
            process_args.push("--save-dev".to_string());
        } else if args.save_optional {
            process_args.push("--save-optional".to_string());
        } else if args.save_exact {
            process_args.push("--save-exact".to_string());
        } else if args.global {
            process_args.push("--global".to_string());
        }
        // exec_child_process!(&self.bin, process_args);
        Ok(process_args)
    }
}
