use crate::trait_transform::{AArgs, CommandArgsCreatorTrait, DArgs, EArgs, IArgs, RArgs, XArgs};

pub struct NpmArgsTransform;

impl CommandArgsCreatorTrait for NpmArgsTransform {
    fn i(&self, args: IArgs) -> Vec<String> {
        let process_args = if args.frozen_lockfile {
            vec!["ci".to_string()]
        } else {
            vec!["install".to_string()]
        };
        process_args
    }

    fn a(&self, args: AArgs) -> Vec<String> {
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

    fn d(&self, args: DArgs) -> Vec<String> {
        let process_args = vec!["uninstall".to_string(), args.package_spec];
        process_args
    }

    fn x(&self, args: XArgs) -> Vec<String> {
        let mut process_args = vec!["exec".to_string()];

        process_args.append(&mut args.package_spec.clone());

        process_args.append(&mut vec!["-y".to_string()]);

        process_args
    }

    fn e(&self, args: EArgs) -> Vec<String> {
        let mut process_args = vec!["exec".to_string()];

        process_args.append(&mut args.package_spec.clone());

        process_args.append(&mut vec!["-n".to_string()]);

        process_args
    }

    fn r(&self, args: RArgs) -> Vec<String> {
        let mut process_args = vec!["run".to_string()];

        process_args.append(&mut args.args.clone());

        process_args
    }
}
