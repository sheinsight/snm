use super::snm::SnmTrait;
use snm_core::{
    exec_child_process,
    model::{package_json_model::PackageManager, PackageJson, SnmError},
};
use snm_pm::get_manager_bin_file_path;
use std::path::PathBuf;

pub struct Npm {
    version_parsed: PackageManager,
    bin: PathBuf,
}

impl Npm {
    pub async fn new() -> Result<Self, SnmError> {
        let version_parsed = PackageJson::from_file_path(None)?.parse_package_manager()?;
        let bin = get_manager_bin_file_path(&version_parsed.package_manager).await?;
        Ok(Self {
            bin,
            version_parsed,
        })
    }
}

impl SnmTrait for Npm {
    fn install(&self, args: super::snm::InstallCommandArgs) -> Result<(), SnmError> {
        let process_args = if args.frozen_lockfile {
            vec!["ci".to_string()]
        } else {
            vec!["install".to_string()]
        };

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
