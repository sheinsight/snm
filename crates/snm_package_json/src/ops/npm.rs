use crate::pm_metadata::PackageManagerMetadata;

use super::{
    command_builder::CommandBuilder,
    flag::Flag,
    ops::{AddArgs, InstallArgs, PackageManagerOps, RemoveArgs},
};

pub struct NpmCommandLine<'a> {
    pub metadata: &'a PackageManagerMetadata,
}

impl<'a> NpmCommandLine<'a> {
    pub fn new(pm: &'a PackageManagerMetadata) -> Self {
        Self { metadata: pm }
    }
}

impl<'a> PackageManagerOps for NpmCommandLine<'a> {
    fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
        CommandBuilder::new(self.metadata.name.clone(), "install")
            .with_addon_opts(vec![Flag::new(args.frozen_lockfile, "--frozen-lockfile")])
            .build()
    }

    fn add(&self, args: AddArgs) -> anyhow::Result<Vec<String>> {
        CommandBuilder::new(self.metadata.name.clone(), "add")
            .with_args(args.package_spec)
            .with_exclu_opts(vec![
                Flag::new(args.save_dev, "--save-dev"),
                Flag::new(args.save_peer, "--save-peer"),
                Flag::new(args.save_optional, "--save-optional"),
            ])
            .with_addon_opts(vec![Flag::new(args.save_exact, "--save-exact")])
            .build()
    }

    fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
        CommandBuilder::new(self.metadata.name.clone(), "uninstall")
            .with_args(args.package_spec)
            .build()
    }
}
