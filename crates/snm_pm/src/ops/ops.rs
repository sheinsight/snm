use clap::{ArgGroup, Parser};
use serde::Serialize;

const SAVE_OPTIONS_HELP_HEADING: &str = r#"Save Options"#;
const INSTALL_OPTIONS_HELP_HEADING: &str = r#"Install Options"#;
const SAVE_TYPE_GROUP_NAME: &str = "save_type";

#[derive(Parser, Debug, Clone, Serialize)]
#[command(group(
  ArgGroup::new(SAVE_TYPE_GROUP_NAME)
      .args(["save_prod", "save_dev", "save_peer", "save_optional"])
      .required(false)
))]
pub struct InstallArgs {
  #[arg(help = "The package spec to install.", display_order = 0)]
  pub package_spec: Vec<String>,

  #[arg(
    long,
    short = 'f',
    help = "If true, pnpm skips lockfile generation, failing install if the lockfile is out of sync or missing.",
    display_order = 1,
    conflicts_with = "package_spec",
    help_heading = INSTALL_OPTIONS_HELP_HEADING
  )]
  pub frozen: bool,

  #[arg(
    short = 'S',
    long,
    help = "Save into dependencies",
    display_order = 2,
    group = SAVE_TYPE_GROUP_NAME,
    requires = "package_spec",
    help_heading = SAVE_OPTIONS_HELP_HEADING
  )]
  pub save_prod: bool,

  #[arg(
    short = 'P',
    long,
    help = "Save into peerDependencies",
    display_order = 3,
    group = SAVE_TYPE_GROUP_NAME,
    requires = "package_spec",
    help_heading = SAVE_OPTIONS_HELP_HEADING
  )]
  pub save_peer: bool,

  #[arg(
    short = 'D',
    long,
    help = "Save into devDependencies",
    display_order = 4,
    group = SAVE_TYPE_GROUP_NAME,
    requires = "package_spec",
    help_heading = SAVE_OPTIONS_HELP_HEADING
  )]
  pub save_dev: bool,

  #[arg(
    short = 'O',
    long,
    help = "Save into optionalDependencies",
    display_order = 5,
    group = SAVE_TYPE_GROUP_NAME,
    requires = "package_spec",
    help_heading = SAVE_OPTIONS_HELP_HEADING
  )]
  pub save_optional: bool,

  #[arg(
    short = 'E',
    long,
    help = "Use exact version to save",
    display_order = 6,
    group = SAVE_TYPE_GROUP_NAME,
    requires = "package_spec",
    help_heading = SAVE_OPTIONS_HELP_HEADING
  )]
  pub save_exact: bool,
}

#[derive(Parser, Debug, Clone, Serialize)]
pub struct RemoveArgs {
  #[arg(help = "The package spec to remove.")]
  pub package_spec: Vec<String>,
}

#[derive(Parser, Debug, Clone, Serialize)]
#[command(
  // trailing_var_arg = true,
  after_help = r#"EXAMPLES:
    snm run dev -- --port=3000    # Run dev script with port 3000
    snm run test -- --watch     # Run test with watch mode"#
)]
pub struct RunArgs {
  #[arg(help = "The command to run.", required = false, default_value = "")]
  pub command: String,

  #[arg(
    help = "Arguments after -- will be passed directly to the command",
    last = true,
    raw = true,
    // allow_hyphen_values = true
  )]
  pub passthrough_args: Vec<String>,
}

pub trait PackageManagerOps {
  fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>>;

  fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>>;

  fn run(&self, args: RunArgs) -> anyhow::Result<Vec<String>>;
}
