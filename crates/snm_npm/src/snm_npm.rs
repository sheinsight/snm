use async_trait::async_trait;
use colored::*;
use dialoguer::Confirm;
use serde_json::Value;
use sha1::Digest;
use sha1::Sha1;
use snm_core::{
    config::SnmConfig,
    model::{
        trait_manage::ManageTrait, trait_shared_behavior::SharedBehaviorTrait,
        trait_shim::ShimTrait, PackageJson, SnmError,
    },
    utils::tarball::decompress_tgz,
};
use std::{
    env::current_dir,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct SnmNpm {
    snm_config: SnmConfig,
    prefix: String,
}

impl SnmNpm {
    pub fn new() -> Self {
        Self {
            snm_config: SnmConfig::new(),
            prefix: "npm".to_string(),
        }
    }

    pub fn from_prefix(prefix: &str) -> Self {
        Self {
            snm_config: SnmConfig::new(),
            prefix: prefix.to_string(),
        }
    }
}

impl SharedBehaviorTrait for SnmNpm {
    fn get_anchor_file_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
            .join("package.json")
    }
}

#[async_trait(?Send)]
impl ManageTrait for SnmNpm {
    fn get_download_url(&self, v: &str) -> String {
        let npm_registry = self.snm_config.get_npm_registry_host();
        format!(
            "{}/{}/-/{}-{}.tgz",
            npm_registry, &self.prefix, &self.prefix, &v
        )
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_download_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
            .join(format!("{}@{}.tgz", &self.prefix, &v))
    }

    fn get_downloaded_dir_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_download_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
            .join(format!("{}-default", &v))
    }

    fn get_runtime_base_dir_path_buf(&self) -> PathBuf {
        self.snm_config
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
    }

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError> {
        let npm_registry = self.snm_config.get_npm_registry_host();
        let download_url = format!("{}/{}/{}", npm_registry, &self.prefix, &v);

        let value: Value = reqwest::get(&download_url)
            .await
            .expect(format!("download error {}", &download_url).as_str())
            .json()
            .await
            .expect(format!("json error {}", &download_url).as_str());

        let x = value
            .get("dist")
            .and_then(|dist| dist.get("shasum"))
            .and_then(|shasum| shasum.as_str())
            .map(|shasum| shasum.to_string())
            .ok_or(SnmError::NotFoundSha256ForNode(v.to_string()))?;

        Ok(x)
    }

    async fn get_actual_shasum(
        &self,
        downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError> {
        let file = File::open(downloaded_file_path_buf).expect(
            format!(
                "get_actual_shasum File::open error {:?}",
                &downloaded_file_path_buf.display()
            )
            .as_str(),
        );
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();

        let mut buffer = [0; 1024];
        loop {
            let n = reader
                .read(&mut buffer)
                .expect("get_actual_shasum read error");
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    fn get_host(&self) -> Option<String> {
        todo!("get_host")
    }

    async fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError> {
        let (dir_vec, default_v) = &dir_tuple;

        dir_vec.into_iter().for_each(|dir| {
            let prefix = if Some(dir) == default_v.as_ref() {
                "⛳️"
            } else {
                " "
            };
            println!("{:<2} {:<10}", prefix, dir.bright_green());
        });
        Ok(())
    }

    async fn show_list_remote(
        &self,
        _dir_tuple: &(Vec<String>, Option<String>),
        _all: bool,
    ) -> Result<(), SnmError> {
        todo!("show_list_remote")
    }

    fn get_shim_trait(&self) -> Box<dyn ShimTrait> {
        Box::new(SnmNpm::from_prefix(&self.prefix))
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress_tgz(&input_file_path_buf, &output_dir_path_buf, |output| {
            output.join("package")
        })?;
        Ok(())
    }
}

impl ShimTrait for SnmNpm {
    fn get_strict_shim_version(&self) -> Result<String, SnmError> {
        let package_json_path_buf = current_dir()
            .expect("get current dir failed")
            .join("package.json");

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
                .expect(
                    "
                download Confirm error",
                )),
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
        let package_json_buf_path = self
            .snm_config
            .get_node_modules_dir_path_buf()
            .join(self.prefix.to_string())
            .join(&version)
            .join("package.json");

        let mut hashmap = PackageJson::from_file_path(&package_json_buf_path)?.bin_to_hashmap()?;

        if let Some(bin) = hashmap.remove(bin_name) {
            return Ok(bin);
        } else {
            return Err(SnmError::NotFoundBinaryFromPackageJsonBinProperty {
                bin_name: bin_name.to_string(),
                file_path: package_json_buf_path,
            });
        }
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
