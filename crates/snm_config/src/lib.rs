use config::{Config, Environment};
use serde::Deserialize;
use snm_npmrc::parse_npmrc;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum InstallStrategy {
    Ask,
    Panic,
    Auto,
}

impl InstallStrategy {
    pub fn from_str(s: &str) -> Self {
        match s {
            "ask" => InstallStrategy::Ask,
            "panic" => InstallStrategy::Panic,
            "auto" => InstallStrategy::Auto,
            _ => {
                let msg = format!(
                    "Unsupported install strategy: {} , only support ask | panic | auto",
                    s
                );
                panic!("{msg}");
            }
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            InstallStrategy::Ask => "ask",
            InstallStrategy::Panic => "panic",
            InstallStrategy::Auto => "auto",
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct SnmConfig {
    strict: Option<bool>,

    node_bin_dir: Option<String>,

    download_dir: Option<String>,

    node_modules_dir: Option<String>,

    node_dist_url: Option<String>,

    node_github_resource_host: Option<String>,

    node_install_strategy: Option<InstallStrategy>,

    package_manager_install_strategy: Option<InstallStrategy>,

    npm_registry: Option<String>,
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

    pub fn get_npm_registry(&self) -> String {
        match &self.npm_registry {
            Some(npm_registry) => npm_registry.clone(),
            None => "https://registry.npmjs.org/".to_string(),
        }
    }

    pub fn get_node_dist_url(&self) -> String {
        match &self.node_dist_url {
            Some(node_dist_url) => node_dist_url.clone(),
            None => "https://nodejs.org/dist".to_string(),
        }
    }

    pub fn get_node_github_resource_host(&self) -> String {
        match &self.node_github_resource_host {
            Some(node_github_resource_host) => node_github_resource_host.clone(),
            None => "https://raw.githubusercontent.com".to_string(),
        }
    }

    pub fn get_node_install_strategy(&self) -> InstallStrategy {
        self.node_install_strategy
            .clone()
            .unwrap_or(InstallStrategy::Auto)
    }

    pub fn get_package_manager_install_strategy(&self) -> InstallStrategy {
        self.package_manager_install_strategy
            .clone()
            .unwrap_or(InstallStrategy::Auto)
    }
}

pub fn parse_snm_config(workspace: &PathBuf) -> Result<SnmConfig, Box<dyn Error>> {
    let config = Config::builder()
        .add_source(Environment::with_prefix("SNM"))
        .build()?;

    let mut config: SnmConfig = config.try_deserialize()?;

    let registry = match parse_npmrc(workspace) {
        Some(npmrc_config) => npmrc_config.get_string("registry").ok(),
        None => None,
    };

    config.npm_registry = registry;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{self, current_dir};
    #[test]
    fn test_parse_config() {
        env::set_var("SNM_STRICT", "true");
        env::set_var("SNM_NODE_BIN_DIR", "node_bin_demo");
        env::set_var("SNM_DOWNLOAD_DIR", "downloads_demo");
        env::set_var("SNM_NODE_MODULES_DIR", "node_modules_demo");
        env::set_var("SNM_NODE_DIST_URL", "https://nodejs.org/dist");
        env::set_var(
            "SNM_NODE_GITHUB_RESOURCE_HOST",
            "https://raw.githubusercontent.com",
        );
        env::set_var("SNM_NODE_INSTALL_STRATEGY", "auto");
        env::set_var("SNM_PACKAGE_MANAGER_INSTALL_STRATEGY", "auto");

        let config = parse_snm_config(&current_dir().unwrap()).unwrap();

        assert_eq!(
            config,
            SnmConfig {
                strict: Some(true),
                node_bin_dir: Some("node_bin_demo".to_string()),
                download_dir: Some("downloads_demo".to_string()),
                node_modules_dir: Some("node_modules_demo".to_string()),
                node_dist_url: Some("https://nodejs.org/dist".to_string()),
                node_github_resource_host: Some("https://raw.githubusercontent.com".to_string()),
                node_install_strategy: Some(InstallStrategy::Auto),
                package_manager_install_strategy: Some(InstallStrategy::Auto),
                npm_registry: None,
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
