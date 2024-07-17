use crate::npm_library::NpmLibraryMeta;
use crate::npm_library::NpmLibraryVersionMeta;
use chrono::DateTime;
use chrono::NaiveDate;
use colored::*;
use dialoguer::Confirm;
use semver::{Version, VersionReq};
use serde_json::Value;
use sha1::Digest;
use sha1::Sha1;
use snm_config::InstallStrategy;
use snm_config::SnmConfig;
use snm_core::traits::atom::AtomTrait;
use snm_package_json::parse_package_json;
use snm_tarball::decompress;
use snm_utils::snm_error::SnmError;
use snm_utils::to_ok::ToOk;
use std::fs;
use std::future::Future;
use std::ops::Not as _;
use std::pin::Pin;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct SnmPackageManager {
    snm_config: SnmConfig,
    library_name: String,
}

impl SnmPackageManager {
    pub fn from_prefix(library_name: &str, snm_config: SnmConfig) -> Self {
        Self {
            library_name: library_name.to_string(),
            snm_config,
        }
    }
}

impl AtomTrait for SnmPackageManager {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_node_modules_dir()?
            .join(&self.library_name)
            .join(v)
            .join("package.json")
            .to_ok()
    }

    fn download_condition(&self, version: &str) -> bool {
        match self.snm_config.get_package_manager_install_strategy() {
            InstallStrategy::Ask => {
                return Confirm::new()
                    .with_prompt(format!(
                        "🤔 {} is not installed, do you want to install it ?",
                        &version
                    ))
                    .interact()
                    .expect("download Confirm error")
            }
            InstallStrategy::Panic => {
                let msg = format!(
                    "UnsupportedPackageManager {} {}",
                    self.library_name.to_string(),
                    version.to_string()
                );
                panic!("{msg}");
            }
            InstallStrategy::Auto => true,
        }
    }

    fn get_runtime_binary_dir_string(&self, version: &str) -> Result<String, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir()?
            .join(self.library_name.to_string())
            .join(&version)
            .join("bin")
            .display()
            .to_string())
    }

    fn get_download_url(&self, v: &str) -> String {
        let npm_registry = self.snm_config.get_npm_registry();

        let req = VersionReq::parse(">1").unwrap();

        let version = Version::parse(v).unwrap();

        if self.library_name == "yarn" && req.matches(&version) {
            format!("{}/@yarnpkg/cli-dist/-/cli-dist-{}.tgz", npm_registry, &v)
        } else {
            format!(
                "{}/{}/-/{}-{}.tgz",
                npm_registry, &self.library_name, &self.library_name, &v
            )
        }
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_download_dir()?
            .join(&self.library_name)
            .join(&v)
            .join(format!("{}@{}.tgz", &self.library_name, &v))
            .to_ok()
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        let library_name = if &self.library_name == "@yarnpkg/cli-dist" {
            "yarn"
        } else {
            &self.library_name
        };

        self.snm_config
            .get_node_modules_dir()?
            .join(&library_name)
            .join(&v)
            .to_ok()
    }

    fn get_runtime_dir_for_default_path_buf(&self) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_node_modules_dir()?
            .join(&self.library_name)
            .join("default")
            .to_ok()
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_node_modules_dir()?
            .join(&self.library_name)
            .to_ok()
    }

    fn get_expect_shasum<'a>(
        &'a self,
        v: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            let npm_registry = self.snm_config.get_npm_registry();
            let download_url = format!("{}/{}/{}", npm_registry, &self.library_name, &v);

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

    fn show_list<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let (dir_vec, default_v) = self.read_runtime_dir_name_vec()?;
            dir_vec.into_iter().for_each(|dir| {
                let prefix = if Some(dir.clone()) == default_v {
                    "⛳️"
                } else {
                    " "
                };
                println!("{:<2} {:<10}", prefix, dir.bright_green());
            });

            Ok(())
        })
    }

    fn show_list_offline<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>> {
        todo!("show_list_remote")
    }

    fn show_list_remote<'a>(
        &'a self,
        _all: bool,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let npm_registry = self.snm_config.get_npm_registry();

            let response: NpmLibraryMeta =
                reqwest::get(format!("{}/{}", npm_registry, &self.library_name).as_str())
                    .await?
                    .json::<NpmLibraryMeta>()
                    .await?;

            let mut versions: Vec<&NpmLibraryVersionMeta> = response.versions.values().collect();

            versions.sort_by_cached_key(|v| Version::parse(&v.version).ok());

            versions.iter().for_each(|item| {
                let license = if let Some(license) = &item.license {
                    license.clone().bright_green()
                } else {
                    "None".to_string().bright_black()
                };

                let publish_time = if let Some(time) = response.time.get(&item.version) {
                    let date_time_utc = DateTime::parse_from_rfc3339(time).expect("xx");

                    let naive_date: NaiveDate = date_time_utc.date_naive();

                    naive_date.format("%Y-%m-%d").to_string()
                } else {
                    "None".to_string()
                };

                let x: String = format!(
                    "{:<2} {:<20} {:<14} {}",
                    "".to_string(),
                    item.version.bright_green(),
                    license,
                    publish_time.to_string().bright_black()
                );

                println!("{}", x);
            });

            Ok(())
        })
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress(&input_file_path_buf, &output_dir_path_buf)?;
        if let Some(package_json) = parse_package_json(&output_dir_path_buf)? {
            let bin = output_dir_path_buf.join("bin");

            if bin.exists().not() {
                fs::create_dir_all(&bin)?;
            }
            for (k, v) in package_json.bin.iter() {
                let link_file = &bin.join(k);
                if link_file.exists().not() {
                    #[cfg(unix)]
                    {
                        std::os::unix::fs::symlink(v, link_file)?;
                    }
                    #[cfg(windows)]
                    {
                        std::os::windows::fs::symlink_dir(v, link_file)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn get_snm_config(&self) -> &SnmConfig {
        &self.snm_config
    }
}
