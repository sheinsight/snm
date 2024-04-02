use snm_core::{
    config::SNM_NPM_REGISTRY_HOST_KEY,
    model::{package_json_model::PackageManager, PackageJson, SnmError},
    print_warning, println_success,
    utils::{
        download::{DownloadBuilder, WriteStrategy},
        tarball::decompress_tgz,
    },
};
use std::{io::stdout, path::PathBuf};

use crate::path::{get_npm_and_version_dir, get_npm_downloaded_file_path};

pub struct Npm {
    pub version_parsed: PackageManager,
    pub npm_registry_host_url: String,
}

impl Npm {
    pub fn new(version_parsed: PackageManager) -> Self {
        let npm_registry_host_url = std::env::var(SNM_NPM_REGISTRY_HOST_KEY).unwrap();
        Self {
            version_parsed,
            npm_registry_host_url,
        }
    }

    pub async fn get_bin_path(&self, bin_name: &str) -> Result<PathBuf, SnmError> {
        let PackageManager {
            package_manager,
            version,
            ..
        } = &self.version_parsed;

        let dir = get_npm_and_version_dir(&package_manager, &version)?;

        let package_json_path = dir.join("package.json");

        if !package_json_path.exists() {
            let mut stdout = stdout();

            print_warning!(stdout, "Waiting Download...");

            let tgz_downloaded_path = self.download().await?;

            println_success!(stdout, "Downloaded");

            print_warning!(stdout, "Waiting Decompress...");

            let mut progress = Some(|_from: &PathBuf, to: &PathBuf| {
                // print_warning!(stdout, "Waiting Decompress...")
            });

            decompress_tgz(
                &tgz_downloaded_path,
                &dir,
                |output| output.join("package"),
                &mut progress,
            )?;

            println_success!(stdout, "Decompressed");
        }

        let bin_hashmap = PackageJson::from_file_path(Some(dir))?.bin_to_hashmap()?;

        let bin_path =
            bin_hashmap
                .get(bin_name)
                .ok_or(SnmError::PackageJsonBinPropertyNotFound {
                    file_path: package_json_path.display().to_string(),
                })?;

        Ok(PathBuf::from(bin_path))
    }

    pub fn get_download_url(&self) -> String {
        let PackageManager {
            package_manager,
            version,
            ..
        } = &self.version_parsed;
        format!(
            "{}/{}/-/{}-{}.tgz",
            self.npm_registry_host_url, package_manager, package_manager, version
        )
    }

    fn get_downloaded_path(&self) -> Result<PathBuf, SnmError> {
        let PackageManager {
            package_manager,
            version,
            ..
        } = &self.version_parsed;
        let downloaded_path = get_npm_downloaded_file_path(package_manager, version)?;
        Ok(downloaded_path)
    }

    async fn download(&self) -> Result<PathBuf, SnmError> {
        let download_url = self.get_download_url();
        let downloaded_path = self.get_downloaded_path()?;
        DownloadBuilder::new()
            .retries(3)
            .write_strategy(WriteStrategy::Nothing)
            .download(&download_url, &downloaded_path)
            .await?;

        Ok(downloaded_path)
    }
}
