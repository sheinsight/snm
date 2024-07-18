use crate::conditional_compiler::get_arch;
use crate::conditional_compiler::get_os;
use crate::conditional_compiler::get_tarball_ext;
use futures::*;
use sha2::Digest;
use sha2::Sha256;
use snm_config::SnmConfig;
use snm_core::traits::atom::AtomTrait;
use snm_tarball::decompress;
use snm_utils::snm_error::SnmError;
use snm_utils::to_ok::ToOk;
use std::collections::HashMap;
use std::pin::Pin;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct NodeAtom {
    snm_config: SnmConfig,
}

impl NodeAtom {
    pub fn new(snm_config: SnmConfig) -> Self {
        Self { snm_config }
    }

    async fn get_node_sha256_hashmap(
        &self,
        node_version: &str,
    ) -> Result<HashMap<String, String>, SnmError> {
        let host = self.snm_config.get_node_dist_url();
        let url = format!("{}/v{}/SHASUMS256.txt", host, node_version);

        let sha256_str = reqwest::get(&url).await?.text().await?;

        let sha256_map: std::collections::HashMap<String, String> = sha256_str
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let sha256 = iter.next().unwrap();
                let file = iter.next().unwrap();
                (file.to_string(), sha256.to_string())
            })
            .collect();

        Ok(sha256_map)
    }
}

impl AtomTrait for NodeAtom {
    fn get_anchor_file_path_buf(&self, version: &str) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_node_bin_dir()?
            .join(&version)
            .join("bin")
            .join("node")
            .to_ok()
    }

    fn get_runtime_binary_dir_string(&self, version: &str) -> Result<String, SnmError> {
        Ok(self
            .get_runtime_dir_path_buf(&version)?
            .join("bin")
            .display()
            .to_string())
    }

    fn get_downloaded_file_name(&self, v: &str) -> String {
        format!(
            "node-v{}-{}-{}.{}",
            &v,
            get_os(),
            get_arch(),
            get_tarball_ext()
        )
    }

    fn get_download_url(&self, v: &str) -> String {
        let host = self.snm_config.get_node_dist_url();
        let download_url = format!("{}/v{}/{}", &host, &v, self.get_downloaded_file_name(v));
        download_url
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_download_dir()?
            .join("node")
            .join(v)
            .join(format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            ))
            .to_ok()
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        self.snm_config.get_node_bin_dir()?.join(&v).to_ok()
    }

    fn get_runtime_dir_for_default_path_buf(&self) -> Result<PathBuf, SnmError> {
        self.snm_config.get_node_bin_dir()?.join("default").to_ok()
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        self.snm_config.get_node_bin_dir()
    }

    fn get_expect_shasum<'a>(
        &'a self,
        v: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let mut hashmap = self.get_node_sha256_hashmap(&v).await?;
            let tar_file_name = format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            );
            Ok(hashmap.remove(&tar_file_name))
        })
    }

    fn get_actual_shasum<'a>(
        &'a self,
        downloaded_file_path_buf: &'a PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SnmError>> + Send + 'a>> {
        Box::pin(async move {
            if let Ok(file) = File::open(downloaded_file_path_buf) {
                let mut reader = BufReader::new(file);
                let mut hasher = Sha256::new();
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
            } else {
                Ok(None)
            }
        })
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress(&input_file_path_buf, &output_dir_path_buf)
    }

    fn get_snm_config(&self) -> &SnmConfig {
        &self.snm_config
    }
}
