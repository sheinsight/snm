use colored::*;
use dialoguer::Confirm;
use serde_json::Value;
use sha1::Digest;
use sha1::Sha1;
use snm_core::model::current_dir::cwd;
use snm_core::{
    snm_content::SnmContentHandler,
    traits::{manage::ManageTrait, shared_behavior::SharedBehaviorTrait, shim::ShimTrait},
    utils::tarball::decompress_tgz,
};
use snm_package_json::parse_package_json;
use std::future::Future;
use std::pin::Pin;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct SnmPackageManager {
    snm_content_handler: SnmContentHandler,
    prefix: String,
}

impl SnmPackageManager {
    pub fn from_prefix(prefix: &str, snm_content_handler: SnmContentHandler) -> Self {
        Self {
            prefix: prefix.to_string(),
            snm_content_handler,
        }
    }
}

impl SharedBehaviorTrait for SnmPackageManager {
    fn get_anchor_file_path_buf(&self, v: &str) -> PathBuf {
        self.snm_content_handler
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
            .join("package.json")
    }
}

impl ManageTrait for SnmPackageManager {
    fn get_download_url(&self, v: &str) -> String {
        let npm_registry = self.snm_content_handler.get_npm_registry();
        format!(
            "{}/{}/-/{}-{}.tgz",
            npm_registry, &self.prefix, &self.prefix, &v
        )
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> PathBuf {
        self.snm_content_handler
            .get_download_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
            .join(format!("{}@{}.tgz", &self.prefix, &v))
    }

    fn get_downloaded_dir_path_buf(&self, v: &str) -> PathBuf {
        self.snm_content_handler
            .get_download_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> PathBuf {
        self.snm_content_handler
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
            .join(&v)
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> PathBuf {
        self.snm_content_handler
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
            .join(format!("{}-default", &v))
    }

    fn get_runtime_base_dir_path_buf(&self) -> PathBuf {
        self.snm_content_handler
            .get_node_modules_dir_path_buf()
            .join(&self.prefix)
    }

    fn get_expect_shasum<'a>(
        &'a self,
        v: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            let npm_registry = self.snm_content_handler.get_npm_registry();
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
                .expect(format!("NotFoundSha256ForNode {}", v.to_string()).as_str());

            Some(x)
        })
    }

    fn get_actual_shasum<'a>(
        &'a self,
        downloaded_file_path_buf: &'a PathBuf,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
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
            Some(format!("{:x}", result))
        })
    }

    fn get_host(&self) -> Option<String> {
        todo!("get_host")
    }

    fn show_list<'a>(
        &'a self,
        dir_tuple: &'a (Vec<String>, Option<String>),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let (dir_vec, default_v) = &dir_tuple;

            dir_vec.into_iter().for_each(|dir| {
                let prefix = if Some(dir) == default_v.as_ref() {
                    "⛳️"
                } else {
                    " "
                };
                println!("{:<2} {:<10}", prefix, dir.bright_green());
            });
        })
    }

    fn show_list_offline<'a>(
        &'a self,
        _dir_tuple: &'a (Vec<String>, Option<String>),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        todo!("show_list_remote")
    }

    fn show_list_remote<'a>(
        &'a self,
        _dir_tuple: &'a (Vec<String>, Option<String>),
        _all: bool,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        todo!("show_list_remote")
    }

    fn get_shim_trait(&self) -> Box<dyn ShimTrait> {
        Box::new(SnmPackageManager::from_prefix(
            &self.prefix,
            self.snm_content_handler.clone(),
        ))
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) {
        decompress_tgz(&input_file_path_buf, &output_dir_path_buf);
    }
}

impl ShimTrait for SnmPackageManager {
    fn check_satisfy_strict_mode(&self, bin_name: &str) {
        let workspace = cwd();

        let package_json = match parse_package_json(workspace) {
            Some(package_json) => package_json,
            None => panic!("NoPackageManager"),
        };

        let package_manager = match package_json.package_manager {
            Some(pm) => pm,
            None => panic!("NoPackageManager"),
        };

        let name = match package_manager.name {
            Some(n) => n,
            None => panic!("NoPackageManager"),
        };

        if name != bin_name {
            let msg = format!("NotMatchPackageManager {} {}", name, bin_name.to_string());
            panic!("{msg}");
        }
    }

    fn get_strict_shim_version(&self) -> String {
        let workspace = cwd();

        let package_json = match parse_package_json(workspace) {
            Some(package_json) => package_json,
            None => panic!("NoPackageManager"),
        };

        let package_manager = match package_json.package_manager {
            Some(pm) => pm,
            None => panic!("NoPackageManager"),
        };

        match package_manager.version {
            Some(v) => v,
            None => panic!("NoPackageManager"),
        }
    }

    fn get_strict_shim_binary_path_buf(&self, bin_name: &str, version: &str) -> PathBuf {
        let node_binary_path_buf = self.get_runtime_binary_file_path_buf(&bin_name, &version);
        node_binary_path_buf
    }

    fn download_condition(&self, version: &str) -> bool {
        match self
            .snm_content_handler
            .get_package_manager_install_strategy()
        {
            snm_core::config::snm_config::InstallStrategy::Ask => {
                return Confirm::new()
                    .with_prompt(format!(
                        "🤔 {} is not installed, do you want to install it ?",
                        &version
                    ))
                    .interact()
                    .expect("download Confirm error")
            }
            snm_core::config::snm_config::InstallStrategy::Panic => {
                let msg = format!(
                    "UnsupportedPackageManager {} {}",
                    self.prefix.to_string(),
                    version.to_string()
                );
                panic!("{msg}");
            }
            snm_core::config::snm_config::InstallStrategy::Auto => true,
        }
    }

    fn get_runtime_binary_file_path_buf(&self, bin_name: &str, version: &str) -> PathBuf {
        let mut package_json_buf_path = self
            .snm_content_handler
            .get_node_modules_dir_path_buf()
            .join(self.prefix.to_string())
            .join(&version);

        if !(&package_json_buf_path.join("package.json").exists()) {
            package_json_buf_path = package_json_buf_path.join("package")
        }

        let mut package_json = match parse_package_json(package_json_buf_path.clone()) {
            Some(package_json) => package_json,
            None => panic!("NoPackageManager"),
        };

        if let Some(bin) = package_json.bin.remove(bin_name) {
            return bin;
        } else {
            let msg = format!(
                "Not found binary from {} bin property: {}",
                package_json_buf_path.display(),
                bin_name
            );
            panic!("{msg}");
        }
    }

    fn check_default_version(&self, tuple: &(Vec<String>, Option<String>)) -> String {
        let (_, default_v_dir) = tuple;
        if let Some(v) = default_v_dir {
            return v.to_string();
        } else {
            let msg = format!("NotFoundDefaultPackageManager {}", self.prefix.to_string());
            panic!("{msg}");
        }
    }
}
