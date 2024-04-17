use async_trait::async_trait;
use dialoguer::Confirm;
use snm_core::{
    config::SnmConfig,
    model::{
        trait_manage::ManageTrait, trait_shared_behavior::SharedBehaviorTrait,
        trait_shim::ShimTrait, PackageJson, SnmError,
    },
};
use snm_npm::snm_npm::SnmNpm;
use std::{env::current_dir, fs, ops::Not, path::PathBuf};

pub struct SnmYarnPkg {
    snm_config: SnmConfig,
    prefix: String,
    snm_npm: SnmNpm,
}

impl SnmYarnPkg {
    pub fn new() -> Self {
        Self {
            snm_config: SnmConfig::new(),
            prefix: "yarn".to_string(),
            snm_npm: SnmNpm::from_prefix("yarn"),
        }
    }
}

impl SharedBehaviorTrait for SnmYarnPkg {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix)
            .join(&v)
            .join("yarn.js"))
    }
}

#[async_trait(?Send)]
impl ManageTrait for SnmYarnPkg {
    fn get_download_url(&self, v: &str) -> Result<String, SnmError> {
        let npm_repo = self.snm_config.get_yarn_repo_host();
        return Ok(format!(
            "{}/{}/packages/yarnpkg-cli/bin/yarn.js",
            npm_repo, v
        ));
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.get_downloaded_dir_path_buf(v)?.join("yarn.js"))
    }

    fn get_downloaded_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_npm.get_downloaded_dir_path_buf(v)?)
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_npm.get_runtime_dir_path_buf(v)?)
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_npm.get_runtime_dir_for_default_path_buf(v)?)
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        Ok(self.snm_npm.get_runtime_base_dir_path_buf()?)
    }

    async fn get_expect_shasum(&self, _v: &str) -> Result<String, SnmError> {
        Ok("TODO".to_string())
    }

    async fn get_actual_shasum(
        &self,
        _downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError> {
        Ok("TODO".to_string())
    }

    fn get_host(&self) -> Option<String> {
        todo!("get_host")
    }

    async fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError> {
        self.snm_npm.show_list(dir_tuple).await?;
        Ok(())
    }

    async fn show_list_remote(
        &self,
        dir_tuple: &(Vec<String>, Option<String>),
        all: bool,
    ) -> Result<(), SnmError> {
        self.snm_npm.show_list_remote(dir_tuple, all).await?;
        Ok(())
    }

    fn get_shim_trait(&self) -> Box<dyn ShimTrait> {
        Box::new(SnmYarnPkg::new())
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        if output_dir_path_buf.exists().not() {
            fs::create_dir_all(&output_dir_path_buf)?;
        }
        fs::copy(&input_file_path_buf, &output_dir_path_buf.join("yarn.js"))?;
        Ok(())
    }
}

impl ShimTrait for SnmYarnPkg {
    fn get_strict_shim_version(&self) -> Result<String, SnmError> {
        let package_json_path_buf = current_dir()?.join("package.json");

        let package_json = PackageJson::from_file_path(&package_json_path_buf)?;

        let package_manager = package_json.parse_package_manager()?;

        let version = package_manager.version;

        Ok(version)
    }

    fn get_strict_shim_binary_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError> {
        let node_binary_path_buf = self.get_runtime_binary_file_path_buf(&bin_name, &version)?;
        set_exec_permission(&node_binary_path_buf)?;
        Ok(node_binary_path_buf)
    }

    fn download_condition(&self, version: &str) -> Result<bool, SnmError> {
        match self.snm_config.get_package_manager_install_strategy()? {
            snm_core::config::snm_config::InstallStrategy::Ask => Ok(Confirm::new()
                .with_prompt(format!(
                    "🤔 {} is not installed, do you want to install it ?",
                    &version
                ))
                .interact()
                .expect("download Confirm error")),
            snm_core::config::snm_config::InstallStrategy::Panic => {
                Err(SnmError::UnsupportedPackageManager {
                    name: self.prefix.to_string(),
                    version: version.to_string(),
                })
            }
            snm_core::config::snm_config::InstallStrategy::Auto => Ok(true),
        }
    }

    fn get_runtime_binary_file_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(self.prefix.to_string())
            .join(&version)
            .join(bin_name))
    }

    fn check_default_version(
        &self,
        tuple: &(Vec<String>, Option<String>),
    ) -> Result<String, SnmError> {
        let (_, default_v_dir) = tuple;
        if let Some(v) = default_v_dir {
            return Ok(v.to_string());
        } else {
            return Err(SnmError::NotFoundDefaultPackageManager {
                name: self.prefix.to_string(),
            });
        }
    }
}

#[cfg(target_family = "unix")]
fn set_exec_permission(bin_path: &PathBuf) -> Result<(), SnmError> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::metadata(&bin_path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(permissions.mode() | 0o111); // UNIX: 增加所有用户的执行权限
    fs::set_permissions(&bin_path, permissions)?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn set_exec_permission(bin_path: &PathBuf) -> Result<(), SnmError> {
    // Windows 上设置执行权限的方法不如 Unix 直接，
    // 通常是通过文件属性或直接关联到可执行程序去处理，
    // 暂时不需要复杂实现，因为执行权限通常默认存在
    Ok(())
}
