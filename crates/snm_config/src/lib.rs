use config::{Config, Environment};
use serde::Deserialize;
use snm_node_version::{parse_node_version, NodeVersion};
use snm_npmrc::NpmrcReader;
use snm_package_json::{package_manager_meta::PackageManager, parse_package_json, PackageJson};
use snm_utils::snm_error::SnmError;
use std::{env, path::PathBuf};

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

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct Ini {
    default_node: Option<String>,

    default_npm: Option<String>,

    default_pnpm: Option<String>,

    default_yarn: Option<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Clone)]
pub struct SnmConfig {
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

    snm_package_json: Option<PackageJson>,

    snm_node_version: Option<NodeVersion>,
    // #[serde(skip)]
    // ini: Ini,
}

impl SnmConfig {
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

    pub fn get_snm_node_version(&self) -> Option<NodeVersion> {
        self.snm_node_version.clone()
    }

    pub fn get_snm_package_json(&self) -> Option<PackageJson> {
        self.snm_package_json.clone()
    }

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

pub fn parse_snm_config(workspace: &PathBuf) -> Result<SnmConfig, SnmError> {
    let config = Config::builder()
        .add_source(Environment::with_prefix("SNM"))
        .build()?;

    let mut config: SnmConfig = config.try_deserialize()?;

    let npmrc = NpmrcReader::from(&workspace);

    let registry = npmrc.read_registry_with_default();

    let node_version = parse_node_version(workspace)?;

    if let Some(ref v) = node_version {
        if let Some(v) = v.get_version() {
            env::set_var(SNM_NODE_VERSION_ENV_KEY, v);
        }
    }

    let package_json = parse_package_json(workspace)?;

    if let Some(ref p) = package_json {
        if let Some(ref pm) = p.package_manager {
            env::set_var(SNM_PACKAGE_MANAGER_NAME_ENV_KEY, pm.name.clone());
            env::set_var(SNM_PACKAGE_MANAGER_VERSION_ENV_KEY, pm.version.clone());
        }
    }

    // let file = config.get_base_dir()?.join("snm.ini");

    // if file.exists().not() {
    //     File::create(&file)?;
    // }

    // let ini: Ini = Config::builder()
    //     .add_source(config::File::from(file))
    //     .build()?
    //     .try_deserialize()?;

    // config.ini = ini;
    config.npm_registry = Some(registry);
    config.workspace = Some(workspace.to_string_lossy().to_string());
    config.snm_node_version = node_version;
    config.snm_package_json = package_json;

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

        let config = parse_snm_config(&current_dir().unwrap()).unwrap();

        assert_eq!(
            config,
            SnmConfig {
                node_bin_dir: Some("node_bin_demo".to_string()),
                download_dir: Some("downloads_demo".to_string()),
                lang: Some("en".to_string()),
                cache_dir: Some("cache_demo".to_string()),
                node_modules_dir: Some("node_modules_demo".to_string()),
                node_dist_url: Some("https://nodejs.org/dist".to_string()),
                node_github_resource_host: Some("https://raw.githubusercontent.com".to_string()),
                node_install_strategy: Some(InstallStrategy::Auto),
                node_white_list: Some("1.1.0,1.2.0".to_string()),
                download_timeout_secs: Some(60),
                npm_registry: Some("https://registry.npmjs.org/".to_string()),
                workspace: Some(current_dir().unwrap().to_string_lossy().to_string()),
                snm_node_version: None,
                snm_package_json: None,
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
