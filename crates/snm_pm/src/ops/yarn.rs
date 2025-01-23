use super::{
  command_builder::CommandBuilder,
  flag::Flag,
  ops::{InstallArgs, PackageManagerOps, RemoveArgs},
};
use crate::pm_metadata::PackageManagerMetadata;

pub struct YarnCommandLine<'a> {
  pub metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> YarnCommandLine<'a> {
  pub fn new(pm: &'a PackageManagerMetadata) -> Self {
    Self { metadata: pm }
  }
}

impl<'a> PackageManagerOps for YarnCommandLine<'a> {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
    match (args.package_spec.is_empty(), args.frozen) {
      // CI
      (true, true) => CommandBuilder::new(self.metadata.name.clone(), "install")
        .with_addon_opts(vec![Flag::new(args.frozen, "--frozen-lockfile")])
        .build(),
      // init install
      (true, false) => CommandBuilder::new(self.metadata.name.clone(), "install").build(),
      // add library
      (false, _) => CommandBuilder::new(self.metadata.name.clone(), "add")
        .with_args(args.package_spec)
        .with_exclu_opts(vec![
          // Flag::new(args.save_prod, ""),
          Flag::new(args.save_dev, "--dev"),
          Flag::new(args.save_peer, "--peer"),
          Flag::new(args.save_optional, "--optional"),
        ])
        .with_addon_opts(vec![Flag::new(args.save_exact, "--exact")])
        .build(),
    }
  }

  fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
    CommandBuilder::new(self.metadata.name.clone(), "remove")
      .with_args(args.package_spec)
      .build()
  }
}
