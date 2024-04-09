use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use async_trait::async_trait;
use sha2::Digest;
use sha2::Sha256;
use snm_core::{
    config::{
        cfg::{get_arch, get_os, get_tarball_ext},
        url::SnmUrl,
        SnmConfig,
    },
    model::{manager::ManagerTrait, SnmError},
    utils::tarball::decompress_xz,
};

use crate::node_list_remote::get_node_sha256_hashmap;

pub struct NodeDemo {
    snm_config: SnmConfig,
}

impl NodeDemo {
    pub fn new() -> Self {
        Self {
            snm_config: SnmConfig::new(),
        }
    }
}
#[async_trait(?Send)]
impl ManagerTrait for NodeDemo {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_bin_dir_path_buf()?
            .join(&v)
            .join("bin")
            .join("node"))
    }

    fn get_download_url(&self, v: &str) -> Result<String, SnmError> {
        Ok(SnmUrl::new().get_node_tar_download_url(&v))
    }

    fn get_downloaded_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_config.get_download_dir_path_buf()?.join(v))
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_download_dir_path_buf()?
            .join(v)
            .join(format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            )))
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_config.get_node_bin_dir_path_buf()?.join(&v))
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_bin_dir_path_buf()?
            .join(format!("{}-default", &v)))
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        Ok(self.snm_config.get_node_bin_dir_path_buf()?)
    }

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError> {
        let mut hashmap = get_node_sha256_hashmap(&v).await?;
        let tar_file_name = format!(
            "node-v{}-{}-{}.{}",
            &v,
            get_os(),
            get_arch(),
            get_tarball_ext()
        );
        let expect_sha256 = hashmap
            .remove(&tar_file_name)
            .ok_or(SnmError::NotFoundSha256ForNode(tar_file_name.to_string()))?;
        Ok(expect_sha256)
    }

    async fn get_actual_shasum(
        &self,
        downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError> {
        let file = File::open(downloaded_file_path_buf)?;
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
        Ok(format!("{:x}", result))
    }

    fn get_host(&self) -> Option<String> {
        None
    }

    fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError> {
        todo!()
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress_xz(
            &input_file_path_buf,
            &output_dir_path_buf,
            &mut Some(|_from: &PathBuf, _to: &PathBuf| {}),
        )?;
        Ok(())
    }
}
