use snm_core::model::SnmError;

use super::trait_transform_args::{AddCommandArgs, CommandArgsCreatorTrait, InstallCommandArgs};

pub struct YarnPkgArgsTransform;

impl CommandArgsCreatorTrait for YarnPkgArgsTransform {
    fn get_install_command(&self, args: InstallCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["install".to_string()];
        if args.frozen_lockfile {
            process_args.push("--immutable".to_string());
        }

        Ok(process_args)
    }

    fn get_add_command(&self, args: AddCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["add".to_string(), args.package_spec];
        if args.save_prod {
            process_args.push("--save".to_string());
        } else if args.save_dev {
            process_args.push("--dev".to_string());
        } else if args.save_optional {
            process_args.push("--optional".to_string());
        } else if args.save_exact {
            process_args.push("--exact".to_string());
        } else if args.save_peer {
            process_args.push("--peer".to_string());
        } else if args.global {
            unimplemented!("global is not supported by yarnpkg")
        }
        Ok(process_args)
    }

    fn get_delete_command(
        &self,
        args: super::trait_transform_args::DeleteCommandArgs,
    ) -> Result<Vec<String>, SnmError> {
        let process_args = vec!["remove".to_string(), args.package_spec];
        Ok(process_args)
    }

    fn get_dlx_command(
        &self,
        args: super::trait_transform_args::DlxCommandArgs,
    ) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["dlx".to_string()];
        process_args.append(&mut args.package_spec.clone());
        Ok(process_args)
    }
}
