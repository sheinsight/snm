use crate::pm_metadata::PackageManagerMetadata;

use super::{
    command_builder::CommandBuilder,
    flag::Flag,
    ops::{AddArgs, InstallArgs, PackageManagerOps, RemoveArgs},
};

pub struct YarnBerryCommandLine<'a> {
    pub metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> YarnBerryCommandLine<'a> {
    pub fn new(pm: &'a PackageManagerMetadata) -> Self {
        Self { metadata: pm }
    }
}

impl<'a> PackageManagerOps for YarnBerryCommandLine<'a> {
    fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
        CommandBuilder::new(self.metadata.name.clone(), "install")
            .with_addon_opts(vec![Flag::new(args.frozen_lockfile, "--immutable")])
            .build()
    }

    fn add(&self, args: AddArgs) -> anyhow::Result<Vec<String>> {
        CommandBuilder::new(self.metadata.name.clone(), "add")
            .with_args(args.package_spec)
            .with_exclu_opts(vec![
                Flag::new(args.save_dev, "--dev"),
                Flag::new(args.save_peer, "--peer"),
                Flag::new(args.save_optional, "--optional"),
            ])
            .with_addon_opts(vec![Flag::new(args.save_exact, "--exact")])
            .build()
    }

    fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
        CommandBuilder::new(self.metadata.name.clone(), "remove")
            .with_args(args.package_spec)
            .build()
    }
}
