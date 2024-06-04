use std::{env, fs::create_dir_all, path::PathBuf};

static SNM_BASE_DIR_KEY: &str = "SNM_BASE_DIR";

static SNM_NODE_BIN_DIR: &str = "SNM_NODE_BIN_DIR";
static SNM_DOWNLOAD_DIR: &str = "SNM_DOWNLOAD_DIR";
static SNM_NODE_MODULES_DIR: &str = "SNM_NODE_MODULES_DIR";

static SNM_NPM_REGISTRY_HOST_KEY: &str = "SNM_NPM_REGISTRY_HOST";
static SNM_YARN_REGISTRY_HOST_KEY: &str = "SNM_YARN_REGISTRY_HOST_KEY";
static SNM_YARN_REPO_HOST_KEY: &str = "SNM_YARN_REPO_HOST_KEY";
static SNM_NODEJS_DIST_URL_KEY: &str = "SNM_NODEJS_DIST_URL_KEY";
static SNM_NODEJS_GITHUB_RESOURCE_HOST_KEY: &str = "SNM_NODEJS_GITHUB_RESOURCE_HOST_KEY";

static SNM_STRICT: &str = "SNM_STRICT";

// strategy  ask | panic | auto
static SNM_NODE_INSTALL_STRATEGY: &str = "SNM_NODE_INSTALL_STRATEGY";

static SNM_PACKAGE_MANAGER_INSTALL_STRATEGY: &str = "SNM_PACKAGE_MANAGER_INSTALL_STRATEGY";

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

pub struct SnmConfig {}

impl SnmConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {
        self.init_strict();

        self.create_dir_all(self.get_base_dir_path_buf());
        self.create_dir_all(self.get_node_bin_dir_path_buf());
        self.create_dir_all(self.get_download_dir_path_buf());
        self.create_dir_all(self.get_node_modules_dir_path_buf());

        self.init_url_config();
    }

    fn create_dir_all(&self, path_buf: PathBuf) {
        create_dir_all(&path_buf)
            .expect(format!("create dir error {:?}", path_buf.display().to_string()).as_str());
    }

    pub fn get_strict(&self) -> bool {
        let value = env::var(SNM_STRICT).unwrap_or(false.to_string());
        value.parse::<bool>().unwrap_or(false)
    }

    pub fn get_base_dir_path_buf(&self) -> PathBuf {
        let home_dir = dirs::home_dir().expect("get home dir error.");
        let base_dir_name = env::var(SNM_BASE_DIR_KEY).unwrap_or(".snm".to_string());
        home_dir.join(base_dir_name)
    }

    pub fn get_node_bin_dir_path_buf(&self) -> PathBuf {
        let base_dir = self.get_base_dir_path_buf();
        let node_bin_dir_name = env::var(SNM_NODE_BIN_DIR).unwrap_or("node_bin".to_string());
        base_dir.join(node_bin_dir_name)
    }

    pub fn get_download_dir_path_buf(&self) -> PathBuf {
        let base_dir = self.get_base_dir_path_buf();
        let download_dir_name = env::var(SNM_DOWNLOAD_DIR).unwrap_or("download".to_string());
        base_dir.join(download_dir_name)
    }

    pub fn get_node_modules_dir_path_buf(&self) -> PathBuf {
        let base_dir = self.get_base_dir_path_buf();
        let node_modules_dir_name =
            env::var(SNM_NODE_MODULES_DIR).unwrap_or("node_modules".to_string());
        base_dir.join(node_modules_dir_name)
    }

    pub fn get_npm_registry_host(&self) -> String {
        env::var(SNM_NPM_REGISTRY_HOST_KEY).unwrap_or("https://registry.npmjs.org".to_string())
    }

    pub fn get_yarn_registry_host(&self) -> String {
        env::var(SNM_YARN_REGISTRY_HOST_KEY).unwrap_or("https://registry.yarnpkg.com".to_string())
    }

    pub fn get_yarn_repo_host(&self) -> String {
        env::var(SNM_YARN_REPO_HOST_KEY).unwrap_or("https://repo.yarnpkg.com".to_string())
    }

    pub fn get_nodejs_dist_url_prefix(&self) -> String {
        env::var(SNM_NODEJS_DIST_URL_KEY).unwrap_or("https://nodejs.org/dist".to_string())
    }

    pub fn get_nodejs_github_resource_host(&self) -> String {
        env::var(SNM_NODEJS_GITHUB_RESOURCE_HOST_KEY)
            .unwrap_or("https://raw.githubusercontent.com".to_string())
    }

    pub fn get_node_install_strategy(&self) -> InstallStrategy {
        let value = env::var(SNM_NODE_INSTALL_STRATEGY).unwrap_or("ask".to_string());
        InstallStrategy::from_str(&value)
    }

    pub fn get_package_manager_install_strategy(&self) -> InstallStrategy {
        let value = env::var(SNM_PACKAGE_MANAGER_INSTALL_STRATEGY).unwrap_or("ask".to_string());
        InstallStrategy::from_str(&value)
    }

    fn init_strict(&self) {
        if let Err(_) = env::var(SNM_STRICT) {
            env::set_var(SNM_STRICT, false.to_string());
        }
    }

    fn init_url_config(&self) {
        self.var(SNM_NPM_REGISTRY_HOST_KEY, "https://registry.npmjs.org");
        self.var(SNM_YARN_REGISTRY_HOST_KEY, "https://registry.yarnpkg.com");
        self.var(SNM_YARN_REPO_HOST_KEY, "https://repo.yarnpkg.com");
        self.var(SNM_NODEJS_DIST_URL_KEY, "https://nodejs.org/dist");
        self.var(
            SNM_NODEJS_GITHUB_RESOURCE_HOST_KEY,
            "https://raw.githubusercontent.com",
        );
    }

    fn var(&self, key: &str, val: &str) {
        if let Err(_) = env::var(key) {
            env::set_var(key, val);
        }
    }
}
