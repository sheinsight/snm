use config::{Config, Environment};
use serde::Deserialize;
use snm_node_version::NodeVersionReader;
// use snm_node_version::{parse_node_version, NodeVersion};
use snm_npmrc::NpmrcReader;
use snm_package_json::{package_manager_meta::PackageManager, package_manager_raw::PackageJson};
use snm_utils::snm_error::SnmError;
use std::{
    env,
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
const SNM_NODE_VERSION_ENV_KEY: &str = "SNM_NODE_VERSION";
const SNM_PACKAGE_MANAGER_NAME_ENV_KEY: &str = "SNM_PACKAGE_MANAGER_NAME";
const SNM_PACKAGE_MANAGER_VERSION_ENV_KEY: &str = "SNM_PACKAGE_MANAGER_VERSION";

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
    pub lang: String,
}

impl SnmConfig {
    fn from<P: AsRef<Path>>(workspace: P) -> Result<Self, SnmError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("SNM"))
            .build()?;

        let config: EnvSnmConfig = config.try_deserialize()?;

        let npm_registry = NpmrcReader::from(&workspace).read_registry_with_default();

        let node_version = NodeVersionReader::from(&workspace).read_version();

        if let Some(ref v) = node_version {
            env::set_var(SNM_NODE_VERSION_ENV_KEY, v);
        }

        let base_dir = Path::new(".snm");

        let node_bin_dir = {
            let node_bin_dir = match config.node_bin_dir {
                Some(node_bin_dir) => node_bin_dir,
                None => "node_bin".to_string(),
            };
            base_dir.join(node_bin_dir)
        };

        let download_dir = {
            let download_dir = match config.download_dir {
                Some(download_dir) => download_dir,
                None => "downloads".to_string(),
            };
            base_dir.join(download_dir)
        };

        let cache_dir = {
            let cache_dir = match config.cache_dir {
                Some(cache_dir) => cache_dir,
                None => "cache".to_string(),
            };
            base_dir.join(cache_dir)
        };

        let node_modules_dir = {
            let node_modules_dir = match config.node_modules_dir {
                Some(node_modules_dir) => node_modules_dir,
                None => "node_modules".to_string(),
            };
            base_dir.join(node_modules_dir)
        };

        let node_dist_url = {
            let node_dist_url = match config.node_dist_url {
                Some(node_dist_url) => node_dist_url,
                None => "https://nodejs.org/dist".to_string(),
            };
            node_dist_url
        };

        let node_github_resource_host = {
            let node_github_resource_host = match config.node_github_resource_host {
                Some(node_github_resource_host) => node_github_resource_host,
                None => "https://raw.githubusercontent.com".to_string(),
            };
            node_github_resource_host
        };

        let node_white_list = {
            let node_white_list = match config.node_white_list {
                Some(node_white_list) => node_white_list,
                None => "".to_string(),
            };
            node_white_list
        };

        Ok(Self {
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
        })
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct EnvSnmConfig {
    node_bin_dir: Option<String>,

    download_dir: Option<String>,

    cache_dir: Option<String>,

    lang: Option<String>,

    node_modules_dir: Option<String>,

    node_dist_url: Option<String>,

    node_github_resource_host: Option<String>,

    node_install_strategy: Option<InstallStrategy>,

    node_white_list: Option<String>,

    download_timeout_secs: Option<u64>,

    npm_registry: Option<String>,
    workspace: Option<String>,
    // snm_package_json: Option<PackageJson>,

    // snm_node_version: Option<String>,
}

impl EnvSnmConfig {
    fn get_base_dir(&self) -> Result<PathBuf, SnmError> {
        match env::var(SNM_HOME_DIR_KEY) {
            Ok(dir) => Ok(PathBuf::from(dir)),
            Err(_) => match dirs::home_dir() {
                Some(dir) => Ok(dir.join(".snm")),
                None => {
                    return Err(SnmError::GetHomeDirError);
                }
            },
        }
    }

    fn get_dir(&self, dir: &Option<String>, default: &str) -> Result<PathBuf, SnmError> {
        let base_dir = self.get_base_dir()?;
        Ok(match dir {
            Some(dir) => base_dir.join(dir),
            None => base_dir.join(default),
        })
    }

    pub fn get_lang(&self) -> Option<String> {
        self.lang.clone()
    }

    pub fn get_runtime_node_version(&self) -> Option<String> {
        env::var(SNM_NODE_VERSION_ENV_KEY).ok()
    }

    pub fn get_runtime_package_manager(&self) -> Option<PackageManager> {
        let name = match env::var(SNM_PACKAGE_MANAGER_NAME_ENV_KEY) {
            Ok(v) => v,
            Err(_) => {
                return None;
            }
        };
        let version = match env::var(SNM_PACKAGE_MANAGER_VERSION_ENV_KEY) {
            Ok(v) => v,
            Err(_) => {
                return None;
            }
        };
        Some(PackageManager {
            name,
            version,
            hash: None,
            raw: "".to_string(),
        })
    }

    // pub fn get_snm_node_version(&self) -> Option<String> {
    //     self.snm_node_version.clone()
    // }

    // pub fn get_snm_package_json(&self) -> Option<PackageJson> {
    //     self.snm_package_json.clone()
    // }

    pub fn get_download_timeout_secs(&self) -> u64 {
        self.download_timeout_secs.unwrap_or(30)
    }

    pub fn get_workspace(&self) -> Result<PathBuf, SnmError> {
        match &self.workspace {
            Some(workspace) => Ok(PathBuf::from(workspace)),
            None => Err(SnmError::GetWorkspaceError),
        }
    }

    pub fn get_node_white_list(&self) -> Vec<String> {
        if let Some(white_list) = &self.node_white_list {
            return white_list.split(",").map(|s| s.to_string()).collect();
        }
        return vec![].to_vec();
    }

    pub fn get_node_bin_dir(&self) -> Result<PathBuf, SnmError> {
        self.get_dir(&self.node_bin_dir, "node_bin")
    }

    pub fn get_download_dir(&self) -> Result<PathBuf, SnmError> {
        self.get_dir(&self.download_dir, "downloads")
    }

    pub fn get_cache_dir(&self) -> Result<PathBuf, SnmError> {
        self.get_dir(&self.cache_dir, "cache")
    }

    pub fn get_node_modules_dir(&self) -> Result<PathBuf, SnmError> {
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
            .unwrap_or(InstallStrategy::Ask)
    }
}

pub fn parse_snm_config(workspace: &PathBuf) -> Result<EnvSnmConfig, SnmError> {
    let config = Config::builder()
        .add_source(Environment::with_prefix("SNM"))
        .build()?;

    let mut config: EnvSnmConfig = config.try_deserialize()?;

    let npmrc = NpmrcReader::from(&workspace);

    let registry = npmrc.read_registry_with_default();

    let node_version_reader = NodeVersionReader::from(workspace);

    let node_version = node_version_reader.read_version();

    if let Some(ref v) = node_version {
        env::set_var(SNM_NODE_VERSION_ENV_KEY, v);
    }

    let package_json_raw = PackageJson::from(workspace);

    if let Some(ref p) = package_json_raw {
        if let Some(pm_name) = p.get_pm_name() {
            env::set_var(SNM_PACKAGE_MANAGER_NAME_ENV_KEY, pm_name.clone());
        }

        if let Some(pm_version) = p.get_pm_version() {
            env::set_var(SNM_PACKAGE_MANAGER_VERSION_ENV_KEY, pm_version.clone());
        }
    }

    config.npm_registry = Some(registry);
    config.workspace = Some(workspace.to_string_lossy().to_string());
    // config.snm_node_version = node_version;
    // config.snm_package_json = package_json_raw;

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
                "https://registry.npmjs.org/".to_string()
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
