use super::{
  command_builder::CommandBuilder,
  flag::Flag,
  ops::{AddArgs, InstallArgs, PackageManagerOps, RemoveArgs},
};
use crate::pm_metadata::PackageManagerMetadata;

pub struct PnpmCommandLine<'a> {
  pub metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> PnpmCommandLine<'a> {
  pub fn new(pm: &'a PackageManagerMetadata) -> Self {
    Self { metadata: pm }
  }
}

impl<'a> PackageManagerOps for PnpmCommandLine<'a> {
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
    CommandBuilder::new(self.metadata.name.clone(), "remove")
      .with_args(args.package_spec)
      .build()
  }
}
