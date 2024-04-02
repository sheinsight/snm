use std::path::PathBuf;

use semver::Version;
use snm_core::{
    exec_child_process,
    model::{package_json_model::PackageManager, PackageJson, SnmError},
};
use snm_pm::get_manager_bin_file_path;

use super::snm::SnmTrait;

pub struct Yarn {
    version_parsed: PackageManager,
    bin: PathBuf,
    is_ge_2: bool,
}

impl Yarn {
    pub async fn new() -> Result<Self, SnmError> {
        let version_parsed = PackageJson::from_file_path(None)?.parse_package_manager()?;
        let bin = get_manager_bin_file_path(&version_parsed.package_manager).await?;
        let x = Version::parse("2.0.0")?;
        let y = Version::parse(&version_parsed.version)?;
        let is_ge_2 = y.ge(&x);
        Ok(Self {
            bin,
            is_ge_2,
            version_parsed,
        })
    }
}

impl SnmTrait for Yarn {
    fn install(&self, args: super::snm::InstallCommandArgs) -> Result<(), SnmError> {
        let x = if args.frozen_lockfile {
            if self.is_ge_2 {
                vec!["install".to_string(), "--immutable".to_string()]
            } else {
                vec!["install".to_string(), "--frozen-lockfile".to_string()]
            }
        } else {
            vec!["install".to_string()]
        };

        exec_child_process!(&self.bin, x);

        Ok(())
    }

    fn add(&self, args: super::snm::AddCommandArgs) -> Result<(), SnmError> {
        todo!()
    }
}
