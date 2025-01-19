use clap::Subcommand;
use serde::Serialize;
#[derive(Subcommand, Debug, Serialize)]
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
