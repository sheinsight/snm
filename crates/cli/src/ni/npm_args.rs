use snm_core::model::SnmError;

use super::trait_transform_args::{AddCommandArgs, CommandArgsCreatorTrait, InstallCommandArgs};

pub struct NpmArgsTransform;

impl CommandArgsCreatorTrait for NpmArgsTransform {
    fn get_install_command(&self, args: InstallCommandArgs) -> Result<Vec<String>, SnmError> {
        let process_args = if args.frozen_lockfile {
            vec!["ci".to_string()]
        } else {
            vec!["install".to_string()]
        };
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
        } else if args.save_peer {
            // process_args.push("--save-peer".to_string());
            unimplemented!("save-peer is not supported by npm")
        } else if args.global {
            process_args.push("--global".to_string());
        }
        Ok(process_args)
    }

    fn get_delete_command(
        &self,
        args: super::trait_transform_args::DeleteCommandArgs,
    ) -> Result<Vec<String>, SnmError> {
        let process_args = vec!["uninstall".to_string(), args.package_spec];
        Ok(process_args)
    }

    fn get_dlx_command(
        &self,
        args: super::trait_transform_args::DlxCommandArgs,
    ) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["exec".to_string()];

        process_args.append(&mut args.package_spec.clone());

        process_args.append(&mut vec!["-n".to_string()]);

        Ok(process_args)
    }
}
