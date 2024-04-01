use async_trait::async_trait;
use semver::Version;
use snm_core::{
    config::{
        DOWNLOAD_DIR_KEY, NODE_MODULES_DIR_KEY, SNM_YARN_REGISTRY_HOST_KEY, SNM_YARN_REPO_HOST_KEY,
    },
    model::SnmError,
    print_warning, println_success,
    utils::{
        download::{DownloadBuilder, WriteStrategy},
        tarball::decompress_tgz,
    },
};
use snm_npm::snm_npm::SnmNpmTrait;
use std::{fs, io::stdout, path::PathBuf, str::FromStr};

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
    fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

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
                |output| output.join(format!("yarn-v{}", v)),
                &mut progress,
            )?;
        } else {
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
        if self.get_is_less_2(v)? {
            let host = std::env::var(SNM_YARN_REGISTRY_HOST_KEY)?;
            return Ok(format!("{}/yarn/-/yarn-{}.tgz", host, v));
        }
        let host = std::env::var(SNM_YARN_REPO_HOST_KEY)?;
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

    async fn use_bin(&self, bin: &str, v: Option<String>) -> Result<PathBuf, SnmError> {
        unimplemented!("use_bin");
    }

    async fn install(&self, v: &str) -> Result<(), SnmError> {
        let package_json_file = self.get_npm_package_json_file(v)?;
        if package_json_file.exists() && !self.ask_reinstall(v)? {
            return Ok(());
        }
        let tar = self.download(v).await?;
        self.decompress(&tar, v)?;
        Ok(())
    }

    async fn default(&self, v: &str) -> Result<bool, SnmError> {
        unimplemented!("default");
    }

    fn uninstall(&self, v: &str) -> Result<(), SnmError> {
        unimplemented!("uninstall");
    }

    fn list(&self) -> Result<(), SnmError> {
        unimplemented!("list");
    }
}
