use config::{Config, Environment};
use serde::Deserialize;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct SnmConfig {
    strict: Option<bool>,

    node_bin_dir: Option<String>,

    download_dir: Option<String>,

    node_modules_dir: Option<String>,
}

impl SnmConfig {
    fn get_base_dir(&self) -> Result<PathBuf, String> {
        match dirs::home_dir() {
            Some(home_dir) => Ok(home_dir.join(".snm")),
            None => {
                return Err("Could not get home directory".to_string());
            }
        }
    }

    fn get_dir(&self, dir: &Option<String>, default: &str) -> Result<PathBuf, String> {
        let base_dir = self.get_base_dir()?;
        Ok(match dir {
            Some(dir) => base_dir.join(dir),
            None => base_dir.join(default),
        })
    }

    pub fn get_strict(&self) -> bool {
        self.strict.unwrap_or(false)
    }

    pub fn get_node_bin_dir(&self) -> Result<PathBuf, std::string::String> {
        self.get_dir(&self.node_bin_dir, "node_bin")
    }

    pub fn get_download_dir(&self) -> Result<PathBuf, std::string::String> {
        self.get_dir(&self.download_dir, "downloads")
    }

    pub fn get_node_modules_dir(&self) -> Result<PathBuf, std::string::String> {
        self.get_dir(&self.node_modules_dir, "node_modules")
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
    use super::*;
    use std::env;
    #[test]
    fn test_parse_config() {
        env::set_var("SNM_STRICT", "true");
        env::set_var("SNM_NODE_BIN_DIR", "node_bin_demo");
        env::set_var("SNM_DOWNLOAD_DIR", "downloads_demo");
        env::set_var("SNM_NODE_MODULES_DIR", "node_modules_demo");

        let config = parse_config().unwrap();

        assert_eq!(
            config,
            SnmConfig {
                strict: Some(true),
                node_bin_dir: Some("node_bin_demo".to_string()),
                download_dir: Some("downloads_demo".to_string()),
                node_modules_dir: Some("node_modules_demo".to_string())
            }
        );

        assert_eq!(
            config.get_download_dir().unwrap(),
            dirs::home_dir().unwrap().join(".snm/downloads_demo")
        );

        assert_eq!(
            config.get_node_bin_dir().unwrap(),
            dirs::home_dir().unwrap().join(".snm/node_bin_demo")
        );

        assert_eq!(
            config.get_node_modules_dir().unwrap(),
            dirs::home_dir().unwrap().join(".snm/node_modules_demo")
        );
    }
}
