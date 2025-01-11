use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum NodeManageCommands {
  /// Set default npm version
  Default(snm_node::factory::DefaultArgs),
  /// Install npm
  Install(snm_node::factory::InstallArgs),
  /// Uninstall npm
  Uninstall(snm_node::factory::UninstallArgs),
  /// List installed npm versions
  List(snm_node::factory::ListArgs),
}
