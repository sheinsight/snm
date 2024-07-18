use semver::{Version, VersionReq};
use serde_json::Value;
use sha1::Digest;
use sha1::Sha1;
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
    pub fn new(library_name: &str, snm_config: SnmConfig) -> Self {
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

    fn get_downloaded_file_name(&self, v: &str) -> String {
        format!("{}@{}.tgz", &self.library_name, &v)
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
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let npm_registry = self.snm_config.get_npm_registry();
            let download_url = format!("{}/{}/{}", npm_registry, &self.library_name, &v);

            let value: Value = reqwest::get(&download_url).await?.json().await?;

            let shasum = value
                .get("dist")
                .and_then(|dist| dist.get("shasum"))
                .and_then(|shasum| shasum.as_str())
                .map(|shasum| shasum.to_string());

            Ok(shasum)
        })
    }

    fn get_actual_shasum<'a>(
        &'a self,
        downloaded_file_path_buf: &'a PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let file = File::open(downloaded_file_path_buf)?;

            let mut reader = BufReader::new(file);
            let mut hasher = Sha1::new();

            let mut buffer = [0; 1024];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            let result = hasher.finalize();
            Ok(Some(format!("{:x}", result)))
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
