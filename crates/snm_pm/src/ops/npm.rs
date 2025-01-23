use super::{
  command_builder::CommandBuilder,
  flag::Flag,
  ops::{InstallArgs, PackageManagerOps, RemoveArgs},
};
use crate::pm_metadata::PackageManagerMetadata;

pub struct NpmCommandLine<'a> {
  pub metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> NpmCommandLine<'a> {
  pub fn new(pm: &'a PackageManagerMetadata) -> Self {
    Self { metadata: pm }
  }
}

impl<'a> PackageManagerOps for NpmCommandLine<'a> {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
    match (args.package_spec.is_empty(), args.frozen) {
      // CI
      (true, true) => CommandBuilder::new(self.metadata.name.clone(), "ci").build(),
      // init install
      (true, false) => CommandBuilder::new(self.metadata.name.clone(), "install").build(),
      // add library
      (false, _) => CommandBuilder::new(self.metadata.name.clone(), "install")
        .with_args(args.package_spec)
        .with_exclu_opts(vec![
          Flag::new(args.save_prod, "--save-prod"),
          Flag::new(args.save_peer, "--save-peer"),
          Flag::new(args.save_dev, "--save-dev"),
          Flag::new(args.save_optional, "--save-optional"),
        ])
        .with_addon_opts(vec![Flag::new(args.save_exact, "--save-exact")])
        .build(),
    }

    // if args.package_spec.is_empty() {
    //   if args.frozen {
    //     CommandBuilder::new(self.metadata.name.clone(), "ci").build()
    //   } else {
    //     CommandBuilder::new(self.metadata.name.clone(), "install").build()
    //   }
    // } else {
    //   CommandBuilder::new(self.metadata.name.clone(), "install")
    //     .with_args(args.package_spec)
    //     .with_exclu_opts(vec![
    //       Flag::new(args.save_prod, "--save"),
    //       Flag::new(args.save_peer, "--save-peer"),
    //       Flag::new(args.save_dev, "--save-dev"),
    //       Flag::new(args.save_optional, "--save-optional"),
    //     ])
    //     .with_addon_opts(vec![Flag::new(args.save_exact, "--save-exact")])
    //     .build()
    // }
  }

  fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
    CommandBuilder::new(self.metadata.name.clone(), "uninstall")
      .with_args(args.package_spec)
      .build()
  }
}
