use snm_config::snm_config::SnmConfig;

pub struct PackageManagerFactory<'a> {
  config: &'a SnmConfig,
}

impl<'a> PackageManagerFactory<'a> {
  pub fn new(config: &'a SnmConfig) -> Self {
    Self { config }
  }

  pub async fn install() {}

  pub async fn uninstall() {}

  pub async fn list() {}

  pub async fn set_default() {}
}
