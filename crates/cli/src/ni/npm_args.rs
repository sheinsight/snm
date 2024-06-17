use super::trait_transform_args::{
    AddCommandArgs, CommandArgsCreatorTrait, DeleteCommandArgs, DlxCommandArgs, ExecCommandArgs,
    InstallCommandArgs, SetCacheArgs,
};

pub struct NpmArgsTransform;

impl CommandArgsCreatorTrait for NpmArgsTransform {
    fn get_install_command(&self, args: InstallCommandArgs) -> Vec<String> {
        let process_args = if args.frozen_lockfile {
            vec!["ci".to_string()]
        } else {
            vec!["install".to_string()]
        };
        process_args
    }

    fn get_add_command(&self, args: AddCommandArgs) -> Vec<String> {
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
            unimplemented!("save-peer is not supported by npm")
        } else if args.global {
            process_args.push("--global".to_string());
        }
        process_args
    }

    fn get_delete_command(&self, args: DeleteCommandArgs) -> Vec<String> {
        let process_args = vec!["uninstall".to_string(), args.package_spec];
        process_args
    }

    fn get_dlx_command(&self, args: DlxCommandArgs) -> Vec<String> {
        let mut process_args = vec!["exec".to_string()];

        process_args.append(&mut args.package_spec.clone());

        process_args.append(&mut vec!["-y".to_string()]);

        process_args
    }

    fn get_exec_command(&self, args: ExecCommandArgs) -> Vec<String> {
        let mut process_args = vec!["exec".to_string()];

        process_args.append(&mut args.package_spec.clone());

        process_args.append(&mut vec!["-n".to_string()]);

        process_args
    }

    fn get_run_command(&self, args: super::trait_transform_args::RunCommandArgs) -> Vec<String> {
        let mut process_args = vec!["run".to_string()];

        process_args.append(&mut args.args.clone());

        process_args
    }

    fn get_set_cache_command(&self, args: SetCacheArgs) -> Vec<String> {
        let process_args = vec![
            "config".to_string(),
            "set".to_string(),
            "cache".to_string(),
            args.cache_path,
        ];
        process_args
    }
}
