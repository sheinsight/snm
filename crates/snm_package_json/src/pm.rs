use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use colored::Colorize;
use flate2::read::GzDecoder;
use sha1::{Digest, Sha1};
use snm_config::SnmConfig;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use tar::Archive;

use crate::{
    ops::{
        npm::NpmCommandLine,
        ops::{AddArgs, InstallArgs, PackageManagerOps, RemoveArgs},
        pnpm::PnpmCommandLine,
        yarn::YarnCommandLine,
        yarn_berry::YarnBerryCommandLine,
    },
    package_json::PackageJson,
    pm_metadata::{PackageManagerMetadata, SNM_PACKAGE_MANAGER_ENV_KEY},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PackageManager<'a> {
    Npm(PackageManagerMetadata<'a>),
    Yarn(PackageManagerMetadata<'a>),
    YarnBerry(PackageManagerMetadata<'a>),
    Pnpm(PackageManagerMetadata<'a>),
}

impl<'a> From<PackageManagerMetadata<'a>> for PackageManager<'a> {
    fn from(metadata: PackageManagerMetadata<'a>) -> Self {
        match metadata.library_name.as_str() {
            "npm" => Self::Npm(metadata),
            "yarn" => Self::Yarn(metadata),
            "@yarnpkg/cli-dist" => Self::YarnBerry(metadata),
            "pnpm" => Self::Pnpm(metadata),
            _ => unreachable!(),
        }
    }
}

impl<'a> PackageManager<'a> {
    fn execute<F, T>(&self, f: F) -> T
    where
        F: Fn(&dyn PackageManagerOps) -> T,
    {
        match self {
            Self::Npm(metadata) => f(&NpmCommandLine::new(metadata)),
            Self::Yarn(metadata) => f(&YarnCommandLine::new(metadata)),
            Self::YarnBerry(metadata) => f(&YarnBerryCommandLine::new(metadata)),
            Self::Pnpm(metadata) => f(&PnpmCommandLine::new(metadata)),
        }
    }

    pub fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
        self.execute(|pm| pm.install(args.clone()))
    }

    pub fn add(&self, args: AddArgs) -> anyhow::Result<Vec<String>> {
        self.execute(|pm| pm.add(args.clone()))
    }

    pub fn remove(&self, args: RemoveArgs) -> anyhow::Result<Vec<String>> {
        self.execute(|pm| pm.remove(args.clone()))
    }
}

impl<'a> PackageManager<'a> {
    fn metadata(&self) -> &PackageManagerMetadata<'a> {
        match self {
            Self::Npm(a) | Self::Yarn(a) | Self::YarnBerry(a) | Self::Pnpm(a) => a,
        }
    }

    pub fn library_name(&self) -> &str {
        self.metadata().library_name.as_str()
    }

    pub fn name(&self) -> &str {
        self.metadata().name.as_str()
    }

    pub fn version(&self) -> &str {
        self.metadata().version.as_str()
    }

    pub fn hash_name(&self) -> Option<&str> {
        self.metadata().hash_name.as_deref()
    }

    pub fn hash_value(&self) -> Option<&str> {
        self.metadata().hash_value.as_deref()
    }

    pub fn try_from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
        Self::from_env(config).or_else(|_| match PackageJson::from(&config.workspace) {
            Ok(json) => match json.package_manager {
                Some(raw) => Self::parse(&raw, config),
                None => bail!("packageManager config not found in package.json"),
            },
            Err(err) => bail!(err),
        })
    }

    pub fn from_env(config: &'a SnmConfig) -> anyhow::Result<Self> {
        let raw = env::var(SNM_PACKAGE_MANAGER_ENV_KEY)?;
        Self::parse(&raw, config)
    }

    pub fn from_str(raw: &str, config: &'a SnmConfig) -> anyhow::Result<Self> {
        Self::parse(raw, config)
    }

    pub fn parse(raw: &str, config: &'a SnmConfig) -> anyhow::Result<Self> {
        let metadata = PackageManagerMetadata::from(&raw, config)?;

        let package_manager = Self::from(metadata);

        Ok(package_manager)
    }
}

impl<'a> PackageManager<'a> {
    pub async fn get_bin(&self, args: &Vec<String>) -> anyhow::Result<String> {
        let actual_bin_name = args.get(0).context("bin name not found")?;

        let actual_bin_name = if actual_bin_name == "pnpx" {
            "pnpm"
        } else if actual_bin_name == "npx" {
            "npm"
        } else {
            actual_bin_name
        };

        let command = args.get(1).context("command not found")?;

        let metadata = self.metadata();

        let version = self.version();

        if metadata.config.restricted_list.contains(command) {
            bail!(
                "Package manager mismatch, expect: {}, actual: {} . Restricted list: {}",
                self.library_name().green(),
                actual_bin_name.red(),
                self.metadata().config.restricted_list.join(", ").black()
            );
        }

        if self.name() != actual_bin_name {
            return Ok(String::new());
        }

        let pkg_dir = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join(version);

        let pkg = pkg_dir.join("package.json");

        if pkg.try_exists()? {
            let json = PackageJson::from(pkg_dir)?;
            let bin_path_buf = json.get_bin_with_name(actual_bin_name)?;
            let dir = bin_path_buf.parent().context("No parent dir")?;
            return Ok(dir.to_string_lossy().into_owned());
        };

        let downloaded_file_path_buf = self.download(version).await?;

        self.verify_shasum(&downloaded_file_path_buf, version)
            .await?;

        let decompressed_dir_path_buf = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join(version);

        self.decompress_download_file(&downloaded_file_path_buf, &decompressed_dir_path_buf)?;

        let json = PackageJson::from(decompressed_dir_path_buf)?;

        let file = json.get_bin_with_name(actual_bin_name)?;

        let bin_dir = file.parent().context("No parent dir")?;

        Ok(bin_dir.to_string_lossy().into_owned())
    }

    async fn download(&self, version: &str) -> anyhow::Result<PathBuf> {
        let metadata = self.metadata();

        let download_url = self.get_download_url(version);

        let downloaded_file_path_buf = metadata
            .config
            .download_dir
            .join(&metadata.library_name)
            .join(version)
            .join(format!("{}-{}.tgz", &metadata.library_name, version));

        DownloadBuilder::new()
            .retries(3)
            .timeout(metadata.config.download_timeout_secs)
            .write_strategy(WriteStrategy::WriteAfterDelete)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        Ok(downloaded_file_path_buf)
    }

    async fn verify_shasum<T: AsRef<Path>>(
        &self,
        file_path: T,
        version: &str,
    ) -> anyhow::Result<()> {
        let expect_shasum = self.get_expect_shasum(version).await?;

        let actual_shasum = self.get_actual_shasum(file_path)?;

        if expect_shasum != actual_shasum {
            bail!("SHASUM mismatch");
        }

        Ok(())
    }

    async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
        let url = self.get_expect_shasum_url(version);

        let resp = reqwest::get(&url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        resp.get("dist")
            .and_then(|item| item.get("shasum"))
            .and_then(|item| item.as_str())
            .map(|item| item.to_string())
            .with_context(|| format!("Invalid SHASUM line format"))
    }

    pub fn get_expect_shasum_url(&self, version: &str) -> String {
        let metadata = self.metadata();
        // https://registry.npmjs.org/react/0.0.1
        // https://registry.npmjs.org/@yarnpkg/cli-dist/2.4.1
        format!(
            "{host}/{library_name}/{version}",
            host = &metadata.config.npm_registry,
            library_name = &metadata.library_name,
            version = version
        )
    }

    fn get_actual_shasum<T: AsRef<Path>>(
        &self,
        downloaded_file_path_buf: T,
    ) -> anyhow::Result<String> {
        let file = File::open(downloaded_file_path_buf)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();
        std::io::copy(&mut reader, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn decompress_download_file<P: AsRef<Path>>(
        &self,
        input_file_path_buf: P,
        output_dir_path_buf: P,
    ) -> anyhow::Result<()> {
        let temp_dir = output_dir_path_buf.as_ref().join("temp");
        std::fs::create_dir_all(&temp_dir)?;

        let tar_gz = File::open(input_file_path_buf)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(&temp_dir)?;

        // 获取解压后的第一个目录
        let entry = std::fs::read_dir(&temp_dir)?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No files found"))??;

        // 移动文件
        for entry in std::fs::read_dir(entry.path())? {
            let entry = entry?;
            let target = output_dir_path_buf.as_ref().join(entry.file_name());
            std::fs::rename(entry.path(), target)?;
        }

        // 清理临时目录
        std::fs::remove_dir_all(&temp_dir)?;

        let json = PackageJson::from(&output_dir_path_buf)?;

        json.enumerate_bin(|k, v| {
            let dir = v.parent().unwrap();
            let link_file = dir.join(k);
            if !link_file.exists() {
                #[cfg(unix)]
                std::os::unix::fs::symlink(v, link_file).unwrap();
                #[cfg(windows)]
                std::os::windows::fs::symlink_file(v, link_file).unwrap();
            }
        });

        Ok(())
    }

    fn get_download_url(&self, version: &str) -> String {
        let metadata = self.metadata();
        let npm_registry = &metadata.config.npm_registry;

        // 使用 rsplit_once 直接获取最后一个部分，避免创建 Vec
        let file_name = metadata
            .library_name
            .rsplit_once('/')
            .map_or(metadata.library_name.clone(), |(_, name)| name.to_owned());

        format!(
            "{host}/{name}/-/{file}-{version}.tgz",
            host = npm_registry,
            name = metadata.library_name,
            file = file_name
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_package_manager_with_pnpm() {
        let config = SnmConfig::from(PathBuf::from(".")).unwrap();

        let pm = PackageManager::parse("pnpm@9.0.0", &config)
            .expect("Should parse PNPM package manager");

        assert!(matches!(pm, PackageManager::Pnpm(_)));

        let info = match pm {
            PackageManager::Pnpm(a) => a,
            _ => panic!("Expected Pnpm variant"),
        };

        assert_eq!(info.library_name, "pnpm");
        assert_eq!(info.version, "9.0.0");
    }

    #[test]
    fn test_parse_package_manager_with_pnpm_and_hash() {
        let config = SnmConfig::from(PathBuf::from(".")).unwrap();

        let pm = PackageManager::parse("pnpm@9.0.0+sha.1234567890", &config)
            .expect("Should parse PNPM package manager");

        assert_eq!(pm.library_name(), "pnpm");
        assert_eq!(pm.version(), "9.0.0");
        assert_eq!(pm.hash_name().as_deref(), Some("sha"));
        assert_eq!(pm.hash_value().as_deref(), Some("1234567890"));
    }

    #[test]
    fn test_parse_package_manager_with_npm() {
        let config = SnmConfig::from(PathBuf::from(".")).unwrap();

        let pm =
            PackageManager::parse("npm@10.0.0", &config).expect("Should parse NPM package manager");

        assert_eq!(pm.library_name(), "npm");
        assert_eq!(pm.version(), "10.0.0");
    }
}
