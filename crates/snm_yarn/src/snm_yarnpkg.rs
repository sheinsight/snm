use std::{
    env::current_dir,
    fs::{self, File},
    io::BufReader,
    ops::Not,
    path::PathBuf,
};

use async_trait::async_trait;
use dialoguer::Confirm;
use snm_core::{
    config::SnmConfig,
    model::{
        manager::{ManagerTrait, SharedBehavior, ShimTrait},
        PackageJson, SnmError,
    },
};
use snm_npm::snm_npm::SnmNpm;

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

impl SharedBehavior for SnmYarnPkg {
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
impl ManagerTrait for SnmYarnPkg {
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
        Ok(self
            .snm_config
            .get_download_dir_path_buf()?
            .join(&self.prefix)
            .join(&v))
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix)
            .join(&v))
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix)
            .join(format!("{}-default", &v)))
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix))
    }

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError> {
        // let npm_registry = self.snm_config.get_npm_registry_host();
        // let download_url = format!("{}/{}/{}", npm_registry, &self.prefix, &v);

        // let value: Value = reqwest::get(&download_url).await?.json().await?;

        // let x = value
        //     .get("dist")
        //     .and_then(|dist| dist.get("shasum"))
        //     .and_then(|shasum| shasum.as_str())
        //     .map(|shasum| shasum.to_string())
        //     .ok_or(SnmError::NotFoundSha256ForNode(v.to_string()))?;

        // Ok(x)
        Ok("TODO".to_string())
    }

    async fn get_actual_shasum(
        &self,
        downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError> {
        // let file = File::open(downloaded_file_path_buf)?;
        // let mut reader = BufReader::new(file);
        // let mut hasher = Sha1::new();

        // let mut buffer = [0; 1024];
        // loop {
        //     let n = reader.read(&mut buffer)?;
        //     if n == 0 {
        //         break;
        //     }
        //     hasher.update(&buffer[..n]);
        // }
        // let result = hasher.finalize();
        // Ok(format!("{:x}", result))
        Ok("TODO".to_string())
    }

    fn get_host(&self) -> Option<String> {
        todo!("get_host")
    }

    async fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError> {
        let (dir_vec, _) = &dir_tuple;
        dir_vec.into_iter().for_each(|dir| {
            println!("{}", dir);
        });
        Ok(())
    }

    async fn show_list_remote(
        &self,
        dir_tuple: &(Vec<String>, Option<String>),
        all: bool,
    ) -> Result<(), SnmError> {
        todo!("show_list_remote")
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

    fn get_strict_shim_binary_path_buf(&self, version: &str) -> Result<PathBuf, SnmError> {
        let node_binary_path_buf = self.get_runtime_binary_file_path_buf(&version)?;
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
                .interact()?),
            snm_core::config::snm_config::InstallStrategy::Panic => {
                Err(SnmError::UnsupportedPackageManager {
                    name: self.prefix.to_string(),
                    version: version.to_string(),
                })
            }
            snm_core::config::snm_config::InstallStrategy::Auto => Ok(true),
        }
    }

    fn get_runtime_binary_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(self.prefix.to_string())
            .join(&v)
            .join("yarn.js"))
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
fn set_exec_permission(&self, bin_path: &PathBuf) -> anyhow::Result<()> {
    // Windows 上设置执行权限的方法不如 Unix 直接，
    // 通常是通过文件属性或直接关联到可执行程序去处理，
    // 暂时不需要复杂实现，因为执行权限通常默认存在
    Ok(())
}
