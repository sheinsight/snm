use std::{env::current_dir, ops::Not, path::PathBuf};

use snm_core::{
    config::SnmConfig,
    model::{PackageJson, SnmError},
};
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};

pub trait Shim {
    fn get_anchor_path_buf(&self) -> Result<PathBuf, SnmError>;

    fn get_node_modules_anchor_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_prefix(&self) -> String;

    fn get_default_version(&self) -> Result<(Vec<String>, Option<String>), SnmError>;

    fn get_configured_version(&self) -> Result<String, SnmError>;

    async fn ask_strategy(&self, v: &str) -> Result<(), SnmError>;

    async fn auto_strategy(&self, v: &str) -> Result<(), SnmError>;

    async fn panic_strategy(&self, v: &str) -> Result<(), SnmError>;

    fn get_npm_binary_path(&self, npm_package_json_path_buf: &PathBuf)
        -> Result<PathBuf, SnmError>;
}

const PACKAGE_JSON_FILE: &str = "package.json";

pub struct NpmShim {
    snm_config: SnmConfig,
    snm_npm: SnmNpm,
    prefix: String,
}

impl NpmShim {
    pub fn new<T: SnmNpmTrait>(prefix: &str, client: T) -> Self {
        Self {
            snm_config: SnmConfig::new(),
            snm_npm: SnmNpm::new(),
            prefix: prefix.to_string(),
        }
    }
}

impl Shim for NpmShim {
    fn get_npm_binary_path(
        &self,
        npm_package_json_path_buf: &PathBuf,
    ) -> Result<PathBuf, SnmError> {
        let npm_package_json = PackageJson::from_file_path(&npm_package_json_path_buf)?;
        let npm_bin_path_buf = npm_package_json
            .bin_to_hashmap()?
            .remove(self.get_prefix().as_str())
            .ok_or(SnmError::UnknownError)?;
        Ok(npm_bin_path_buf)
    }

    fn get_anchor_path_buf(&self) -> Result<PathBuf, SnmError> {
        let package_json_path_buf = current_dir()?.join(PACKAGE_JSON_FILE);
        Ok(package_json_path_buf)
    }

    fn get_prefix(&self) -> String {
        return self.prefix.clone();
    }

    async fn ask_strategy(&self, v: &str) -> Result<(), SnmError> {
        if self.snm_npm.ask_download(&v)? {
            let tar = self.snm_npm.download(&v).await?;
            self.snm_npm.decompress(&tar, &v)?;

            return Ok(());
        }
        Err(SnmError::NotFoundPackageManager {
            name: self.get_prefix(),
            version: v.to_string(),
        })?
    }

    async fn auto_strategy(&self, v: &str) -> Result<(), SnmError> {
        let tar = self.snm_npm.download(&v).await?;
        self.snm_npm.decompress(&tar, &v)?;
        Ok(())
    }

    async fn panic_strategy(&self, v: &str) -> Result<(), SnmError> {
        Err(SnmError::UnsupportedPackageManager {
            name: self.get_prefix(),
            version: v.to_string(),
        })?
    }

    fn get_node_modules_anchor_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        let npm_package_json_path_buf = self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(format!("{}@{}", self.get_prefix(), &v))
            .join(PACKAGE_JSON_FILE);
        Ok(npm_package_json_path_buf)
    }

    fn get_configured_version(&self) -> Result<String, SnmError> {
        let package_json_path_buf = self.get_anchor_path_buf()?;
        let package_json = PackageJson::from_file_path(&package_json_path_buf)?;
        let package_manager = package_json.parse_package_manager()?;
        if package_manager.name != self.get_prefix() {
            Err(SnmError::NotMatchPackageManager {
                expect: package_manager.name,
                actual: self.get_prefix(),
            })?;
        }
        Ok(package_manager.version)
    }

    fn get_default_version(&self) -> Result<(Vec<String>, Option<String>), SnmError> {
        Ok(self.snm_npm.read_bin_dir()?)
    }
}

// 调度器

pub async fn dispatch_shim<T: Shim>(shim: T) -> Result<(String, PathBuf), SnmError> {
    let snm_config = SnmConfig::new();

    let anchor_path_buf = shim.get_anchor_path_buf()?;

    if anchor_path_buf.exists().not() {
        if snm_config.get_strict() {
            Err(SnmError::NotFoundPackageJsonFileError {
                package_json_file_path: "package_json_path_buf.display()".to_string(),
            })?;
        }
        let (npm_vec, default_version) = shim.get_default_version()?;

        if npm_vec.is_empty() {
            Err(SnmError::EmptyPackageManagerList {
                name: shim.get_prefix(),
            })?;
        }

        let version = default_version.ok_or(SnmError::NotFoundDefaultPackageManager {
            name: shim.get_prefix(),
        })?;

        let npm_package_json_path_buf = shim.get_node_modules_anchor_path_buf(&version)?;

        let npm_bin_path_buf = shim.get_npm_binary_path(&npm_package_json_path_buf)?;

        return Ok((version, npm_bin_path_buf));
    }

    let v = shim.get_configured_version()?;

    let npm_package_json_path_buf = shim.get_node_modules_anchor_path_buf(&v)?;

    if npm_package_json_path_buf.exists().not() {
        match snm_config.get_package_manager_install_strategy()? {
            snm_core::config::snm_config::InstallStrategy::Ask => shim.ask_strategy(&v).await?,
            snm_core::config::snm_config::InstallStrategy::Auto => shim.auto_strategy(&v).await?,
            snm_core::config::snm_config::InstallStrategy::Panic => shim.panic_strategy(&v).await?,
        };
    }

    let npm_bin_path_buf = shim.get_npm_binary_path(&npm_package_json_path_buf)?;

    return Ok((v, npm_bin_path_buf));
}
