use async_trait::async_trait;
use semver::Version;
use snm_core::{
    config::SnmConfig,
    model::{PackageJson, SnmError},
    print_warning, println_success,
    utils::{
        download::{DownloadBuilder, WriteStrategy},
        tarball::decompress_tgz,
    },
};
use snm_npm::snm_npm::SnmNpmTrait;
use std::{fs, io::stdout, path::PathBuf};

pub struct SnmYarn {
    prefix: String,
}

impl SnmYarn {
    pub fn new() -> Self {
        Self {
            prefix: "yarn".to_string(),
        }
    }
}

impl SnmYarn {
    fn get_is_less_2(&self, v: &str) -> Result<bool, SnmError> {
        let ver = Version::parse(v)?;
        let is_less_2 = ver < Version::parse("2.0.0")?;
        Ok(is_less_2)
    }
}

#[async_trait(?Send)]
impl SnmNpmTrait for SnmYarn {
    fn get_prefix(&self) -> String {
        self.prefix.clone()
    }

    fn decompress(&self, tar: &PathBuf, v: &str) -> Result<PathBuf, SnmError> {
        let mut stdout = stdout();
        print_warning!(stdout, "Waiting Decompress...");
        let npm_dir = self.get_npm_dir(v)?;
        if self.get_is_less_2(v)? {
            let mut progress = Some(|_from: &PathBuf, _to: &PathBuf| {});
            decompress_tgz(
                &tar,
                &npm_dir,
                |output| output.join("package"),
                &mut progress,
            )?;
        } else {
            fs::create_dir_all(&npm_dir)?;
            fs::copy(tar, &npm_dir.join("yarn.js"))?;
        }
        println_success!(stdout, "Decompressed");
        return Ok(npm_dir);
    }

    fn get_npm_package_json_file(&self, v: &str) -> Result<PathBuf, SnmError> {
        let dir = self.get_npm_dir(v)?;
        if self.get_is_less_2(v)? {
            return Ok(dir.join("package.json"));
        }
        return Ok(dir.join("yarn.js"));
    }

    fn get_download_url(&self, v: &str) -> Result<String, SnmError> {
        let snm_config = SnmConfig::new();
        if self.get_is_less_2(v)? {
            let host = snm_config.get_yarn_registry_host();
            return Ok(format!("{}/yarn/-/yarn-{}.tgz", host, v));
        }
        let host = snm_config.get_yarn_repo_host();
        return Ok(format!("{}/{}/packages/yarnpkg-cli/bin/yarn.js", host, v));
    }

    fn get_tar_file_path(&self, v: &str) -> Result<PathBuf, SnmError> {
        let download_dir = self.get_download_dir()?;
        let download_dir_buf = PathBuf::from(download_dir);
        if self.get_is_less_2(v)? {
            let tar_file_path = download_dir_buf.join(format!("{}@{}.tgz", self.get_prefix(), v));
            return Ok(tar_file_path);
        }
        return Ok(download_dir_buf.join(format!("yarn@{}", v)).join("yarn.js"));
    }

    async fn download(&self, v: &str) -> Result<PathBuf, SnmError> {
        let download_url = self.get_download_url(v)?;
        let tar_file_path = self.get_tar_file_path(v)?;
        {
            let mut stdout = stdout();
            print_warning!(stdout, "Waiting Download...");
            DownloadBuilder::new()
                .retries(3)
                .write_strategy(WriteStrategy::Nothing)
                .download(&download_url, &tar_file_path)
                .await?;
            println_success!(stdout, "Downloaded");
        }
        Ok(tar_file_path)
    }

    async fn use_default_bin(&self, bin: &str) -> Result<PathBuf, SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;
        let default_dir_name = node_modules_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter(|item| item.path().is_dir())
            .find_map(|item| {
                item.file_name()
                    .into_string()
                    .ok()
                    .filter(|file_name| file_name.ends_with("default"))
                    .filter(|file_name| file_name.starts_with(self.get_prefix().as_str()))
            })
            .ok_or(SnmError::NotFoundDefaultNpmBinary)?;

        let default_version = default_dir_name
            .trim_end_matches("-default")
            .split("@")
            .last()
            .ok_or(SnmError::UnknownError)?;

        if self.get_is_less_2(default_version)? {
            let pkg = node_modules_dir.join(default_dir_name).join("package.json");
            let bin = PackageJson::from_dir_path(Some(pkg))?
                .bin_to_hashmap()?
                .get(bin)
                .map(|bin_file_path| PathBuf::from(bin_file_path))
                .ok_or(SnmError::UnknownError)?;
            return Ok(bin);
        }

        let bin = node_modules_dir.join(default_dir_name).join("yarn.js");

        return Ok(bin);
    }

    async fn use_bin(&self, bin: &str, v: &str) -> Result<(String, PathBuf), SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;
        if self.get_is_less_2(&v)? {
            let workspace = node_modules_dir.join(format!("{}@{}", self.get_prefix(), &v));

            let pkg = workspace.join("package.json");

            if !pkg.exists() {
                if self.ask_download(&v)? {
                    let tar = self.download(&v).await?;
                    self.decompress(&tar, v)?;
                }
            }

            let bin = PackageJson::from_dir_path(Some(workspace))?
                .bin_to_hashmap()?
                .get(bin)
                .map(|bin_file_path| PathBuf::from(bin_file_path))
                .ok_or(SnmError::UnknownError)?;

            return Ok((v.to_string(), bin));
        } else {
            let bin = node_modules_dir
                .join(format!("{}@{}", self.get_prefix(), &v))
                .join("yarn.js");

            if !bin.exists() {
                if self.ask_download(&v)? {
                    let tar = self.download(&v).await?;
                    self.decompress(&tar, &v)?;
                    self.set_exec_permission(&bin)?;
                }
            }

            return Ok((v.to_string(), bin));
        }
    }

    async fn install(&self, v: &str) -> Result<(), SnmError> {
        let package_json_file = self.get_npm_package_json_file(v)?;
        if package_json_file.exists() && !self.ask_reinstall(v)? {
            return Ok(());
        }
        let tar = self.download(v).await?;
        let _ = &self.set_exec_permission(&tar)?;
        self.decompress(&tar, v)?;

        Ok(())
    }
}
