use async_trait::async_trait;
use snm_npm::snm_npm::SnmNpmTrait;

pub struct SnmPnpm {
    prefix: String,
}

impl SnmPnpm {
    pub fn new() -> Self {
        Self {
            prefix: "pnpm".to_string(),
        }
    }
}

#[async_trait(?Send)]
impl SnmNpmTrait for SnmPnpm {
    fn get_prefix(&self) -> String {
        self.prefix.clone()
    }
}
