use config::{Config, Environment};
use std::error::Error;

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq, Clone)]
pub struct SnmConfig {
    strict: Option<bool>,

    node_bin_dir: Option<String>,

    download_dir: Option<String>,

    node_modules_dir: Option<String>,
}

impl SnmConfig {
    pub fn get_strict(&self) -> bool {
        self.strict.unwrap_or(false)
    }

    pub fn get_node_bin_dir(&self) -> String {
        if let Some(node_bin_dir) = &self.node_bin_dir {
            node_bin_dir.to_string()
        } else {
            "/usr/local/bin".to_string()
        }
    }

    pub fn get_download_dir(&self) -> String {
        if let Some(download_dir) = &self.download_dir {
            download_dir.to_string()
        } else {
            "/tmp".to_string()
        }
    }

    pub fn get_node_modules_dir(&self) -> String {
        if let Some(node_modules_dir) = &self.node_modules_dir {
            node_modules_dir.to_string()
        } else {
            "node_modules".to_string()
        }
    }
}

pub fn parse_config() -> Result<SnmConfig, Box<dyn Error>> {
    let config = Config::builder()
        .add_source(Environment::with_prefix("SNM"))
        .build()?;

    let config: SnmConfig = config.try_deserialize()?;

    Ok(config)
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

    #[test]
    fn test_parse_config() {
        env::set_var("SNM_STRICT", "true");
        env::set_var("SNM_NODE_BIN_DIR", "/usr/local/bin");
        env::set_var("SNM_DOWNLOAD_DIR", "/tmp");
        env::set_var("SNM_NODE_MODULES_DIR", "node_modules");

        let config = parse_config().unwrap();

        assert_eq!(
            config,
            SnmConfig {
                strict: Some(true),
                node_bin_dir: Some("/usr/local/bin".to_string()),
                download_dir: Some("/tmp".to_string()),
                node_modules_dir: Some("node_modules".to_string())
            }
        );
    }
}
