use std::path::PathBuf;

use snm_core::{
    exec_child_process,
    model::{package_json_model::PackageManager, PackageJson, SnmError},
};
use snm_pm::get_manager_bin_file_path;

use super::snm::SnmTrait;

pub struct Pnpm {
    version_parsed: PackageManager,
    bin: PathBuf,
}

impl Pnpm {
    pub async fn new() -> Result<Self, SnmError> {
        let version_parsed = PackageJson::from_file_path(None)?.parse_package_manager()?;
        let bin = get_manager_bin_file_path(&version_parsed.package_manager).await?;
        Ok(Self {
            bin,
            version_parsed,
        })
    }
}

impl SnmTrait for Pnpm {
    fn install(&self, args: super::snm::InstallCommandArgs) -> Result<(), SnmError> {
        let mut process_args = vec!["install".to_string()];
        if args.frozen_lockfile {
            process_args.push("--frozen-lockfile".to_string());
        }
        exec_child_process!(&self.bin, process_args);
        Ok(())
    }

    fn add(&self, args: super::snm::AddCommandArgs) -> Result<(), SnmError> {
        let mut process_args = vec!["add".to_string(), args.package_spec.to_string()];
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
        exec_child_process!(&self.bin, process_args);
        Ok(())
    }
}
