use snm_core::model::SnmError;

use super::trait_transform_args::{AddCommandArgs, CommandArgsCreatorTrait, InstallCommandArgs};

pub struct PnpmArgsTransform;

impl CommandArgsCreatorTrait for PnpmArgsTransform {
    fn get_install_command(&self, args: InstallCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["install".to_string()];
        if args.frozen_lockfile {
            process_args.push("--frozen-lockfile".to_string());
        }

        Ok(process_args)
    }

    fn get_add_command(&self, args: AddCommandArgs) -> Result<Vec<String>, SnmError> {
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
        Ok(process_args)
    }
}
