use std::path::PathBuf;

use semver::Version;
use snm_core::model::SnmError;

use super::snm::SnmTrait;

pub struct Yarn {
    is_ge_2: bool,
}

impl Yarn {
    pub fn new(v: &str) -> Self {
        Self { is_ge_2: false }
    }
}

impl SnmTrait for Yarn {
    fn install(&self, args: super::snm::InstallCommandArgs) -> Result<Vec<String>, SnmError> {
        let process_args = if args.frozen_lockfile {
            if self.is_ge_2 {
                vec!["install".to_string(), "--immutable".to_string()]
            } else {
                vec!["install".to_string(), "--frozen-lockfile".to_string()]
            }
        } else {
            vec!["install".to_string()]
        };

        // exec_child_process!(&self.bin, x);

        Ok(process_args)
    }

    fn add(&self, args: super::snm::AddCommandArgs) -> Result<Vec<String>, SnmError> {
        todo!()
    }
}
