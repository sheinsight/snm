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
        todo!("yarnpkg add !!!")
    }
}
