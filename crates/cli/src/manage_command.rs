use clap::Subcommand;
use serde::Serialize;
#[derive(Subcommand, Debug, Serialize)]
pub enum NodeManageCommands {
  /// Set default npm version
  Default(crate::node::DefaultArgs),
  /// Install npm
  Install(crate::node::InstallArgs),
  /// Uninstall npm
  Uninstall(crate::node::UninstallArgs),
  /// List installed npm versions
  List(crate::node::ListArgs),
}
