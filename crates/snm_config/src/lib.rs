use config::{Config, Environment};
use serde::Deserialize;
use snm_npmrc::NpmrcReader;
use snm_utils::snm_error::SnmError;
use std::{
    env::{self},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum InstallStrategy {
    Ask,
    Auto,
}

impl InstallStrategy {
    pub fn from_str(s: &str) -> Self {
        match s {
            "ask" => InstallStrategy::Ask,
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
            InstallStrategy::Auto => "auto",
        }
    }
}

const SNM_HOME_DIR_KEY: &str = "SNM_HOME_DIR";
// const SNM_NODE_VERSION_ENV_KEY: &str = "SNM_NODE_VERSION";

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SnmConfig {
    pub node_bin_dir: PathBuf,
    pub download_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub node_modules_dir: PathBuf,
    pub node_dist_url: String,
    pub node_github_resource_host: String,
    pub node_install_strategy: InstallStrategy,
    pub node_white_list: String,
    pub download_timeout_secs: u64,
    pub npm_registry: String,
    pub workspace: PathBuf,
    pub lang: String,
    pub restricted_list: Vec<String>,
    pub strict: bool,
}

impl SnmConfig {
    pub fn from<P: AsRef<Path>>(workspace: P) -> Result<Self, SnmError> {
        let config = EnvSnmConfig::parse()?;

        let npm_registry = NpmrcReader::from(&workspace).read_registry_with_default();

        let base_dir = env::var(SNM_HOME_DIR_KEY)
            .map(PathBuf::from)
            .or_else(|_| dirs::home_dir().ok_or(SnmError::GetHomeDirError))
            .map(|dir| dir.join(".snm"))?;

        if !base_dir.try_exists()? {
            fs::create_dir_all(&base_dir)?;
        }

        let node_bin_dir = base_dir.join(config.node_bin_dir.unwrap_or(String::from("node_bin")));

        if !node_bin_dir.try_exists()? {
            fs::create_dir_all(&node_bin_dir)?;
        }

        let download_dir = base_dir.join(config.download_dir.unwrap_or(String::from("downloads")));

        if !download_dir.try_exists()? {
            fs::create_dir_all(&download_dir)?;
        }

        let cache_dir = base_dir.join(config.cache_dir.unwrap_or(String::from("cache")));

        if !cache_dir.try_exists()? {
            fs::create_dir_all(&cache_dir)?;
        }

        let node_modules_dir = base_dir.join(
            config
                .node_modules_dir
                .unwrap_or(String::from("node_modules")),
        );

        if !node_modules_dir.try_exists()? {
            fs::create_dir_all(&node_modules_dir)?;
        }

        let node_dist_url = config
            .node_dist_url
            .unwrap_or(String::from("https://nodejs.org/dist"));

        let node_github_resource_host = config
            .node_github_resource_host
            .unwrap_or(String::from("https://raw.githubusercontent.com"));

        let node_white_list = config.node_white_list.unwrap_or(String::from(""));

        let restricted_list = {
            let map = |v: String| -> Vec<String> {
                v.split(',').map(|s| s.to_string()).collect::<Vec<String>>()
            };
            config.restricted_list.map(map).unwrap_or(Vec::new())
        };

        let strict = config.strict.unwrap_or(false);

        Ok(Self {
            restricted_list,
            workspace: workspace.as_ref().to_path_buf(),
            node_bin_dir: node_bin_dir,
            download_dir: download_dir,
            cache_dir: cache_dir,
            lang: config.lang.unwrap_or("en".to_string()),
            node_modules_dir: node_modules_dir,
            node_dist_url: node_dist_url,
            node_github_resource_host: node_github_resource_host,
            node_install_strategy: config.node_install_strategy.unwrap_or(InstallStrategy::Ask),
            node_white_list: node_white_list,
            download_timeout_secs: config.download_timeout_secs.unwrap_or(30),
            npm_registry: npm_registry,
            strict: strict,
        })
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct EnvSnmConfig {
    node_bin_dir: Option<String>,

    download_dir: Option<String>,

    cache_dir: Option<String>,

    lang: Option<String>,

    restricted_list: Option<String>,

    node_modules_dir: Option<String>,

    node_dist_url: Option<String>,

    node_github_resource_host: Option<String>,

    node_install_strategy: Option<InstallStrategy>,

    node_white_list: Option<String>,

    download_timeout_secs: Option<u64>,

    npm_registry: Option<String>,

    workspace: Option<String>,

    strict: Option<bool>,
}

impl EnvSnmConfig {
    pub fn parse() -> Result<Self, SnmError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("SNM"))
            .build()?;

        let config: EnvSnmConfig = config.try_deserialize()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{self, current_dir};
    #[test]
    fn test_parse_config() {
        env::set_var(SNM_HOME_DIR_KEY, "");
        env::set_var("SNM_STRICT", "true");
        env::set_var("SNM_NODE_BIN_DIR", "node_bin_demo");
        env::set_var("SNM_DOWNLOAD_DIR", "downloads_demo");
        env::set_var("SNM_LANG", "en");
        env::set_var("SNM_NODE_MODULES_DIR", "node_modules_demo");
        env::set_var("SNM_CACHE_DIR", "cache_demo");
        env::set_var("SNM_NODE_DIST_URL", "https://nodejs.org/dist");
        env::set_var("SNM_DOWNLOAD_TIMEOUT_SECS", "60");
        env::set_var(
            "SNM_NODE_GITHUB_RESOURCE_HOST",
            "https://raw.githubusercontent.com",
        );
        env::set_var("SNM_NODE_INSTALL_STRATEGY", "auto");
        env::set_var("SNM_NODE_WHITE_LIST", "1.1.0,1.2.0");

        if let Ok(dir) = current_dir() {
            let snm_config = match SnmConfig::from(dir) {
                Ok(snm_config) => snm_config,
                Err(_) => {
                    assert!(false);
                    return;
                }
            };
            assert_eq!(snm_config.node_bin_dir.to_str(), Some(".snm/node_bin_demo"));
            assert_eq!(
                snm_config.download_dir.to_str(),
                Some(".snm/downloads_demo")
            );
            assert_eq!(snm_config.lang, "en");
            assert_eq!(snm_config.cache_dir.to_str(), Some(".snm/cache_demo"));
            assert_eq!(
                snm_config.node_modules_dir.to_str(),
                Some(".snm/node_modules_demo")
            );
            assert_eq!(
                snm_config.node_dist_url,
                "https://nodejs.org/dist".to_string()
            );
            assert_eq!(
                snm_config.node_github_resource_host,
                "https://raw.githubusercontent.com".to_string()
            );
            assert_eq!(snm_config.download_timeout_secs, 60);
            assert_eq!(
                snm_config.npm_registry,
                "https://registry.npmjs.org".to_string()
            );
        }

        // let config = parse_snm_config(&current_dir().unwrap()).unwrap();

        // assert_eq!(
        //     config,
        //     EnvSnmConfig {
        //         node_bin_dir: Some("node_bin_demo".to_string()),
        //         download_dir: Some("downloads_demo".to_string()),
        //         lang: Some("en".to_string()),
        //         cache_dir: Some("cache_demo".to_string()),
        //         node_modules_dir: Some("node_modules_demo".to_string()),
        //         node_dist_url: Some("https://nodejs.org/dist".to_string()),
        //         node_github_resource_host: Some("https://raw.githubusercontent.com".to_string()),
        //         node_install_strategy: Some(InstallStrategy::Auto),
        //         node_white_list: Some("1.1.0,1.2.0".to_string()),
        //         download_timeout_secs: Some(60),
        //         npm_registry: Some("https://registry.npmjs.org/".to_string()),
        //         workspace: Some(current_dir().unwrap().to_string_lossy().to_string()),
        //         snm_node_version: None,
        //         snm_package_json: None,
        //     }
        // );

        // assert_eq!(
        //     config.get_download_dir().unwrap(),
        //     dirs::home_dir().unwrap().join(".snm/downloads_demo")
        // );

        // assert_eq!(
        //     config.get_node_bin_dir().unwrap(),
        //     dirs::home_dir().unwrap().join(".snm/node_bin_demo")
        // );

        // assert_eq!(
        //     config.get_node_modules_dir().unwrap(),
        //     dirs::home_dir().unwrap().join(".snm/node_modules_demo")
        // );
    }
}
