use snm_config::SnmConfig;

pub struct PackageManagerFactory<'a> {
    config: &'a SnmConfig,
}

impl<'a> PackageManagerFactory<'a> {
    pub fn new(config: &'a SnmConfig) -> Self {
        Self { config }
    }

    pub async fn install(&self) -> anyhow::Result<()> {
        todo!()
    }
}
