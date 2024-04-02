use crate::path::{get_node_modules_dir, get_npm_and_version_dir, get_npm_downloaded_file_path};
use semver::Version;
use snm_core::{
    config::{SNM_YARN_REGISTRY_HOST_KEY, SNM_YARN_REPO_HOST_KEY},
    model::{package_json_model::PackageManager, PackageJson, SnmError},
    print_warning, println_success,
    utils::{
        download::{DownloadBuilder, WriteStrategy},
        tarball::decompress_tgz,
    },
};
use std::io::stdout;
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

pub struct Yarn {
    pub version_parsed: PackageManager,
    pub yarn_repo_host_url: String,
    pub yarn_registry_host_url: String,
    pub is_less_2: bool,
}

impl Yarn {
    pub fn new(version_parsed: PackageManager) -> Result<Self, SnmError> {
        let ver = Version::parse(&version_parsed.version)?;
        let is_less_2 = ver < Version::parse("2.0.0")?;

        let yarn_repo_host_url = std::env::var(SNM_YARN_REPO_HOST_KEY).unwrap();
        let yarn_registry_host_url = std::env::var(SNM_YARN_REGISTRY_HOST_KEY).unwrap();

        Ok(Self {
            is_less_2,
            version_parsed,
            yarn_repo_host_url,
            yarn_registry_host_url,
        })
    }

    pub fn get_download_url(&self) -> String {
        let PackageManager { version, .. } = &self.version_parsed;
        let url = if self.is_less_2 {
            format!(
                "{}/yarn/-/yarn-{}.tgz",
                self.yarn_registry_host_url, version
            )
        } else {
            format!(
                "{}/{}/packages/yarnpkg-cli/bin/yarn.js",
                self.yarn_repo_host_url, version
            )
        };
        url
    }

    pub async fn get_bin_path(&self, bin_name: &str) -> Result<PathBuf, SnmError> {
        let PackageManager {
            package_manager,
            version,
            ..
        } = &self.version_parsed;

        let dir = get_npm_and_version_dir(package_manager, version)?;

        if self.is_less_2 {
            let package_json_path = dir.join("package.json");

            if !package_json_path.exists() {
                let mut stdout = stdout();

                print_warning!(stdout, "Downloading {}", bin_name);

                let tgz_downloaded_path = self.download().await?;

                println_success!(stdout, "Downloaded");

                print_warning!(stdout, "Waiting Decompress...");

                let mut progress = Some(|_from: &PathBuf, to: &PathBuf| {
                    // print_warning!(stdout, "Waiting Decompress...");
                });

                decompress_tgz(
                    &tgz_downloaded_path,
                    &dir,
                    |output| output.join(format!("yarn-v{}", version)),
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
        } else {
            let bin_path = dir.join(format!("{}.js", bin_name));
            // 获取标准输出的可变句柄
            if !bin_path.exists() {
                let mut stdout = stdout();

                print_warning!(stdout, "Downloading {}", bin_name);

                self.download().await?;

                println_success!(stdout, "Downloaded");

                self.set_exec_permission(&bin_path)?;
            }

            Ok(bin_path)
        }
    }

    fn set_exec_permission(&self, bin_path: &PathBuf) -> Result<(), SnmError> {
        set_exec_permission(bin_path)?;
        Ok(())
    }

    fn get_downloaded_path(&self) -> Result<PathBuf, SnmError> {
        let PackageManager {
            package_manager,
            version,
            ..
        } = &self.version_parsed;

        let path = if self.is_less_2 {
            get_npm_downloaded_file_path(package_manager, version)?
        } else {
            get_node_modules_dir()?
                .join(format!("{}@{}", package_manager, version))
                .join("yarn.js")
        };
        Ok(path)
    }

    async fn download(&self) -> Result<PathBuf, SnmError> {
        let download_url = self.get_download_url();
        let downloaded_path = self.get_downloaded_path()?;

        DownloadBuilder::new()
            .retries(3)
            .write_strategy(WriteStrategy::Nothing)
            // .progress(progress)
            .download(&download_url, &downloaded_path)
            .await?;

        Ok(downloaded_path)
    }
}

#[cfg(target_family = "unix")]
fn set_exec_permission(bin_path: &PathBuf) -> Result<(), SnmError> {
    let metadata = fs::metadata(&bin_path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(permissions.mode() | 0o111); // UNIX: 增加所有用户的执行权限
    fs::set_permissions(&bin_path, permissions)?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn set_exec_permission(bin_path: &PathBuf) -> anyhow::Result<()> {
    // Windows 上设置执行权限的方法不如 Unix 直接，
    // 通常是通过文件属性或直接关联到可执行程序去处理，
    // 暂时不需要复杂实现，因为执行权限通常默认存在
    Ok(())
}
