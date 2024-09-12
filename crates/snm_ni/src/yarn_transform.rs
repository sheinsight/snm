use semver::{Version, VersionReq};

use crate::trait_transform::{AArgs, CommandArgsCreatorTrait, EArgs, IArgs, RArgs, XArgs};

pub struct YarnArgsTransform {
    pub version: String,
}

impl YarnArgsTransform {
    fn is_greater_than_1(&self) -> bool {
        let req = VersionReq::parse(">1").unwrap();
        let version = Version::parse(&self.version).unwrap();
        req.matches(&version)
    }
}

impl CommandArgsCreatorTrait for YarnArgsTransform {
    fn i(&self, args: IArgs) -> Vec<String> {
        if self.is_greater_than_1() {
            let mut process_args = vec!["install".to_string()];
            if args.frozen_lockfile {
                process_args.push("--immutable".to_string());
            }

            process_args
        } else {
            let mut process_args = vec!["install".to_string()];
            if args.frozen_lockfile {
                process_args.push("--frozen-lockfile".to_string());
            }

            process_args
        }
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
            process_args.push("--save-peer".to_string());
        } else if args.global {
            process_args.push("--global".to_string());
        }
        process_args
    }

    fn x(&self, args: XArgs) -> Vec<String> {
        if self.is_greater_than_1() {
            let mut process_args = vec!["dlx".to_string()];
            process_args.append(&mut args.package_spec.clone());
            process_args
        } else {
            let err_msg = format!(" {} Unsupported command: yarn dlx ", self.version);
            panic!("{err_msg}");
        }
    }

    fn e(&self, args: EArgs) -> Vec<String> {
        if self.is_greater_than_1() {
            let mut process_args: Vec<String> = vec!["exec".to_string()];
            process_args.append(&mut args.package_spec.clone());
            process_args
        } else {
            let err_msg = format!(" {} Unsupported command: yarn exec ", self.version);
            panic!("{err_msg}");
        }
    }

    fn r(&self, args: RArgs) -> Vec<String> {
        let mut process_args: Vec<String> = vec!["run".to_string()];
        process_args.append(&mut args.args.clone());
        process_args
    }
}
