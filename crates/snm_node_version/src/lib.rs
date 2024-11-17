use std::{
    env,
    fs::{read_to_string, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use snm_config::SnmConfig;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::snm_error::SnmError;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

const FILE_NAME: &str = ".node-version";

const SNM_NODE_VERSION_ENV_KEY: &str = "SNM_NODE_VERSION";

#[derive(Debug)]
enum ArchiveFormat {
    Tgz,
    Xz,
    Zip,
}

impl ArchiveFormat {
    fn from_path<T: AsRef<Path>>(path: T) -> anyhow::Result<Self> {
        let ext = path
            .as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .with_context(|| "Invalid file extension")?;

        match ext {
            "tgz" | "gz" => Ok(Self::Tgz),
            "xz" => Ok(Self::Xz),
            "zip" => Ok(Self::Zip),
            _ => bail!("Unsupported archive format: {}", ext),
        }
    }
}
pub struct NodeVersionReader<'a> {
    pub version: String,
    pub config: &'a SnmConfig,
}

impl<'a> NodeVersionReader<'a> {
    pub fn from<P: AsRef<Path>>(cwd: P, config: &'a SnmConfig) -> anyhow::Result<Self> {
        let file_path = cwd.as_ref().join(FILE_NAME);

        let version =
            Self::read_version_file(&file_path).with_context(|| "Invalid node version file")?;

        Ok(Self { version, config })
    }

    // TODO 从环境变量中读取版本号
    pub fn from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
        let version = env::var(SNM_NODE_VERSION_ENV_KEY)?;
        Ok(Self { version, config })
    }

    pub fn from_default(config: &'a SnmConfig) -> anyhow::Result<Self> {
        if !config.node_bin_dir.try_exists()? {
            bail!("Node binary directory does not exist");
        }

        let default_dir = config.node_bin_dir.join("default");

        let version = default_dir
            .read_link()?
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .with_context(|| "Invalid default version link")?;
        Ok(Self { version, config })
    }

    pub fn read_version(&self) -> String {
        self.version.clone()
    }

    fn read_version_file<T: AsRef<Path>>(version_path: T) -> Option<String> {
        let file_path = version_path.as_ref();
        if !file_path.exists() {
            return None;
        }
        let raw = read_to_string(file_path).ok()?;
        let version = raw.to_lowercase().trim_start_matches('v').trim().to_owned();
        let version_parts: Vec<_> = version.split('.').collect();
        if version_parts.len() != 3 || version_parts.iter().any(|s| s.parse::<u32>().is_err()) {
            return None;
        }
        env::set_var(SNM_NODE_VERSION_ENV_KEY, &version);
        Some(version)
    }
}

impl<'a> NodeVersionReader<'a> {
    pub async fn get_bin(&self) -> anyhow::Result<String> {
        let version = &self.version;

        self.check_v(&version)?;

        let node_dir = self.config.node_bin_dir.join(&version);

        let node_bin_dir = node_dir.join("bin");

        let node_bin_file = node_bin_dir.join("node");

        if node_bin_file.try_exists()? {
            return Ok(node_bin_dir.to_string_lossy().into_owned());
        }

        let downloaded_file = self.download_node(&version).await?;

        self.verify_shasum(&downloaded_file, &version).await?;

        self.decompress(&downloaded_file, &node_dir)?;

        Ok(node_bin_dir.to_string_lossy().into_owned())
    }

    async fn verify_shasum(&self, file_path: &PathBuf, version: &str) -> anyhow::Result<()> {
        let actual = self.get_actual_shasum(file_path).await?;
        let expected = self.get_expect_shasum(version).await?;

        if actual != expected {
            bail!("Node binary shasum mismatch");
        }
        Ok(())
    }

    async fn download_node(&self, version: &str) -> anyhow::Result<PathBuf> {
        let download_url = self.get_download_url(version);
        let downloaded_file_path_buf = self.get_downloaded_path_buf(version);
        DownloadBuilder::new()
            .retries(3)
            .timeout(self.config.download_timeout_secs)
            .write_strategy(WriteStrategy::WriteAfterDelete)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;
        Ok(downloaded_file_path_buf)
    }

    fn get_downloaded_path_buf(&self, version: &str) -> PathBuf {
        let file_name = self.get_file_name(version);
        self.config
            .download_dir
            .join("node")
            .join(version)
            .join(file_name)
    }

    fn get_file_name(&self, version: &str) -> String {
        format!(
            "node-v{version}-{os}-{arch}.{ext}",
            version = version,
            os = get_os(),
            arch = get_arch(),
            ext = get_tarball_ext()
        )
    }

    async fn get_actual_shasum<T: AsRef<Path>>(
        &self,
        downloaded_file_path_buf: T,
    ) -> anyhow::Result<String> {
        let file = File::open(downloaded_file_path_buf)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        std::io::copy(&mut reader, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
        let sha256_url = format!(
            "{host}/v{version}/SHASUMS256.txt",
            host = self.config.node_dist_url,
            version = version
        );

        let file_name = self.get_file_name(version);

        let sha256_str = reqwest::get(&sha256_url).await?.text().await?;

        sha256_str
            .lines()
            .find_map(|line| {
                let mut parts = line.split_whitespace();
                match (parts.next(), parts.next()) {
                    (Some(sha256), Some(file)) if file == file_name => Some(sha256.to_owned()),
                    _ => None,
                }
            })
            .with_context(|| "Invalid SHASUM line format")
    }

    fn get_download_url(&self, version: &str) -> String {
        format!(
            "{host}/v{version}/node-v{version}-{os}-{arch}.{ext}",
            host = self.config.node_dist_url,
            version = version,
            os = get_os(),
            arch = get_arch(),
            ext = get_tarball_ext()
        )
    }

    fn check_v(&self, version: &str) -> anyhow::Result<()> {
        if self.config.node_white_list.is_empty() {
            return Ok(());
        }
        if self.config.node_white_list.contains(&version) {
            return Ok(());
        }
        bail!(SnmError::UnsupportedNodeVersionError {
            version: version.to_owned(),
            node_white_list: self
                .config
                .node_white_list
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        })
    }

    fn decompress<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        downloaded_file_path_buf: T,
        output_dir: U,
    ) -> anyhow::Result<()> {
        let format = ArchiveFormat::from_path(&downloaded_file_path_buf)?;
        let file = File::open(&downloaded_file_path_buf)?;

        let temp_dir = output_dir.as_ref().join("temp");
        std::fs::create_dir_all(&temp_dir)?;
        match format {
            ArchiveFormat::Tgz => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(&temp_dir)?;
            }
            ArchiveFormat::Xz => {
                // 处理 xz
                let xz = XzDecoder::new(file);
                let mut archive = Archive::new(xz);
                archive.unpack(&temp_dir)?;
            }
            ArchiveFormat::Zip => {
                // 处理 zip
                let mut archive = ZipArchive::new(file)?;
                archive.extract(&temp_dir)?;
            }
            _ => bail!("Unsupported tarball format"),
        }

        // 获取解压后的第一个目录
        let entry = std::fs::read_dir(&temp_dir)?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No files found"))??;

        // 移动文件
        for entry in std::fs::read_dir(entry.path())? {
            let entry = entry?;
            let target = output_dir.as_ref().join(entry.file_name());
            std::fs::rename(entry.path(), target)?;
        }

        // 清理临时目录
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }
}

pub const fn get_tarball_ext() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "zip"
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        "tar.xz"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "unknown"
    }
}

pub const fn get_arch() -> &'static str {
    #[cfg(target_arch = "x86")]
    {
        "x86"
    }
    #[cfg(target_arch = "x86_64")]
    {
        "x64"
    }
    #[cfg(target_arch = "arm")]
    {
        "armv7l"
    }
    #[cfg(target_arch = "aarch64")]
    {
        "arm64"
    }
    #[cfg(target_arch = "powerpc64")]
    {
        "ppc64"
    }
    #[cfg(target_arch = "powerpc64le")]
    {
        "ppc64le"
    }
    #[cfg(target_arch = "s390x")]
    {
        "s390x"
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "powerpc64le",
        target_arch = "s390x"
    )))]
    {
        "unknown"
    }
}

pub const fn get_os() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "darwin"
    }
    #[cfg(target_os = "windows")]
    {
        "win"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        "unknown"
    }
}
