use std::{
    env::current_dir,
    fs::{self, DirEntry, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use dialoguer::Confirm;
use serde_json::Value;
use sha1::Digest;
use sha1::Sha1;
use snm_core::{
    config::SnmConfig,
    model::{
        manager::{ManagerTrait, SharedBehavior, ShimTrait},
        PackageJson, SnmError,
    },
    print_warning, println_success,
    utils::{
        download::{DownloadBuilder, WriteStrategy},
        tarball::decompress_tgz,
    },
};

#[async_trait(?Send)]
pub trait SnmNpmTrait {
    fn read_bin_dir(&self) -> Result<(Vec<String>, Option<String>), SnmError> {
        let mut default_version = None;

        let node_modules_path_buf = self.get_node_modules_dir()?;

        return match node_modules_path_buf.read_dir() {
            Ok(dir_reader) => {
                let dir_vec: Vec<String> = dir_reader
                    .filter_map(Result::ok)
                    .filter(|dir_entry| dir_entry.path().is_dir())
                    .filter(|dir_entry| {
                        dir_entry
                            .file_name()
                            .into_string()
                            .ok()
                            .map_or(false, |file_name| {
                                file_name.starts_with(self.get_prefix().as_str())
                            })
                    })
                    .filter_map(|x| {
                        let version = x.file_name().into_string().ok()?;
                        if version.ends_with("default") {
                            default_version =
                                Some(version.trim_end_matches("-default").to_string());
                        }
                        Some(version)
                    })
                    .collect();
                Ok((dir_vec, default_version))
            }
            Err(_) => Err(SnmError::ReadDirFailed {
                dir_path: node_modules_path_buf.display().to_string(),
            }),
        };
    }

    async fn use_default_bin(&self, bin: &str) -> Result<PathBuf, SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;
        let default_dir = node_modules_dir
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
            .map(|dir_name| node_modules_dir.join(dir_name))
            .ok_or(SnmError::NotFoundDefaultNpmBinary)?;

        let bin = PackageJson::from_dir_path(Some(default_dir))?
            .bin_to_hashmap()?
            .get(bin)
            .map(|bin_file_path| PathBuf::from(bin_file_path))
            .ok_or(SnmError::UnknownError)?;

        return Ok(bin);
    }

    async fn use_bin(&self, bin: &str, v: &str) -> Result<(String, PathBuf), SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;

        let wk = node_modules_dir.join(format!("{}@{}", self.get_prefix(), &v));

        if !wk.join("package.json").exists() {
            if self.ask_download(&v)? {
                let tar = self.download(&v).await?;
                self.decompress(&tar, &v)?;
            }
        }

        let bin = PackageJson::from_dir_path(Some(wk))?
            .bin_to_hashmap()?
            .get(bin)
            .map(|bin_file_path| PathBuf::from(bin_file_path))
            .ok_or(SnmError::UnknownError)?;

        return Ok((v.to_string(), bin));
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

    fn get_prefix(&self) -> String;

    fn decompress(&self, tar: &PathBuf, v: &str) -> Result<PathBuf, SnmError> {
        let npm_dir = self.get_npm_dir(v)?;

        print_warning!("Waiting Decompress...");
        let mut progress = Some(|_from: &PathBuf, _to: &PathBuf| {
            // print_warning!(stdout, "Waiting Decompress...")
        });
        decompress_tgz(
            &tar,
            &npm_dir,
            |output| output.join("package"),
            &mut progress,
        )?;
        println_success!("Decompressed");

        Ok(npm_dir)
    }

    async fn download(&self, v: &str) -> Result<PathBuf, SnmError> {
        let download_url = self.get_download_url(v)?;
        let tar_file_path = self.get_tar_file_path(v)?;
        {
            print_warning!("Waiting Download...");
            DownloadBuilder::new()
                .retries(3)
                .write_strategy(WriteStrategy::Nothing)
                .download(&download_url, &tar_file_path)
                .await?;
            println_success!("Downloaded");
        }
        Ok(tar_file_path)
    }

    fn ask_uninstall(&self, v: &str) -> Result<bool, SnmError> {
        let proceed = Confirm::new()
            .with_prompt(format!(
                "🤔 {} {} is installed, do you want to uninstall it ?",
                self.get_prefix(),
                &v
            ))
            .interact()?;
        Ok(proceed)
    }

    fn ask_reinstall(&self, v: &str) -> Result<bool, SnmError> {
        let proceed = Confirm::new()
            .with_prompt(format!(
                "🤔 {} {} is already installed, do you want to reinstall it ?",
                self.get_prefix(),
                &v
            ))
            .interact()?;
        Ok(proceed)
    }

    fn ask_download(&self, v: &str) -> Result<bool, SnmError> {
        let proceed = Confirm::new()
            .with_prompt(format!(
                "🤔 {} {} does not exist, do you want to download it ?",
                self.get_prefix(),
                &v
            ))
            .interact()?;
        Ok(proceed)
    }

    fn get_npm_package_json_file(&self, v: &str) -> Result<PathBuf, SnmError> {
        let dir = self.get_npm_dir(v)?;
        Ok(dir.join("package.json"))
    }

    fn get_npm_dir(&self, v: &str) -> Result<PathBuf, SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;
        let dir = node_modules_dir.join(format!("{}@{}", self.get_prefix(), v));
        Ok(dir)
    }

    fn get_npm_dir_for_default(&self, v: &str) -> Result<PathBuf, SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;
        let dir = node_modules_dir.join(format!("{}@{}-default", self.get_prefix(), v));
        Ok(dir)
    }

    fn get_download_url(&self, v: &str) -> Result<String, SnmError> {
        let npm_registry = self.get_npm_registry()?;
        let url = format!(
            "{}/{}/-/{}-{}.tgz",
            npm_registry,
            self.get_prefix(),
            self.get_prefix(),
            v
        );
        Ok(url)
    }

    fn get_node_modules_dir(&self) -> Result<PathBuf, SnmError> {
        let node_modules_dir = SnmConfig::new().get_node_modules_dir_path_buf()?;
        Ok(node_modules_dir)
    }

    fn get_npm_registry(&self) -> Result<String, SnmError> {
        let npm_registry_host = SnmConfig::new().get_npm_registry_host();
        Ok(npm_registry_host)
    }

    fn get_download_dir(&self) -> Result<PathBuf, SnmError> {
        let download_dir = SnmConfig::new().get_download_dir_path_buf()?;
        Ok(download_dir)
    }

    fn get_tar_file_path(&self, v: &str) -> Result<PathBuf, SnmError> {
        let download_dir_buf = self.get_download_dir()?;
        let tar_file_path = download_dir_buf.join(format!("{}@{}.tgz", self.get_prefix(), v));
        Ok(tar_file_path)
    }

    #[cfg(target_os = "windows")]
    fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        original: P,
        link: Q,
    ) -> std::io::Result<()> {
        // 在 Windows 上创建目录符号链接
        std::os::windows::fs::symlink_dir(original, link)
    }

    #[cfg(not(target_os = "windows"))]
    fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        original: P,
        link: Q,
    ) -> std::io::Result<()> {
        // 对于非 Windows 的 Unix 系统，包括 Linux 和 macOS，创建符号链接
        // macOS 基于 Unix，因此这部分代码也适用于 macOS。
        std::os::unix::fs::symlink(original, link)
    }

    #[cfg(target_family = "unix")]
    fn set_exec_permission(&self, bin_path: &PathBuf) -> Result<(), SnmError> {
        use std::os::unix::fs::PermissionsExt;

        let metadata = fs::metadata(&bin_path)?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(permissions.mode() | 0o111); // UNIX: 增加所有用户的执行权限
        fs::set_permissions(&bin_path, permissions)?;
        Ok(())
    }

    #[cfg(target_family = "windows")]
    fn set_exec_permission(&self, bin_path: &PathBuf) -> anyhow::Result<()> {
        // Windows 上设置执行权限的方法不如 Unix 直接，
        // 通常是通过文件属性或直接关联到可执行程序去处理，
        // 暂时不需要复杂实现，因为执行权限通常默认存在
        Ok(())
    }

    async fn default(&self, v: &str) -> Result<bool, SnmError> {
        let node_modules_dir = self.get_node_modules_dir()?;
        node_modules_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter(|item| item.path().is_dir())
            .filter_map(|item| {
                item.file_name()
                    .into_string()
                    .ok()
                    .filter(|file_name| file_name.ends_with("default"))
            })
            .map(|dir_name| node_modules_dir.join(dir_name))
            .into_iter()
            .try_for_each(|item| fs::remove_dir_all(item))?;

        let package_json_file = self.get_npm_package_json_file(v)?;
        if !package_json_file.exists() {
            let proceed = self.ask_download(v)?;
            if !proceed {
                self.download(v).await?;
            } else {
                return Ok(false);
            }
        }
        let npm_dir = self.get_npm_dir(v)?;
        let npm_default_dir = self.get_npm_dir_for_default(v)?;
        self.create_symlink(npm_dir, npm_default_dir)?;
        Ok(true)
    }

    fn uninstall(&self, v: &str) -> Result<(), SnmError> {
        let dirs = self
            .get_node_modules_dir()?
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter(|item| item.path().is_dir())
            .filter_map(|item| {
                item.file_name()
                    .into_string()
                    .ok()
                    .filter(|file_name| {
                        file_name.starts_with(format!("{}@{}", self.get_prefix(), &v).as_str())
                    })
                    .map(|_| item)
            })
            .collect::<Vec<DirEntry>>();

        if dirs.is_empty() {
            println!(
                "{} {} is not found , please check you input version",
                self.get_prefix(),
                &v
            );
            return Ok(());
        }

        if self.ask_uninstall(v)? {
            for dir in dirs.into_iter() {
                fs::remove_dir_all(dir.path())?;
            }
        }

        Ok(())
    }

    fn list(&self) -> Result<(), SnmError> {
        self.get_node_modules_dir()?
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter(|item| item.path().is_dir())
            .filter_map(|item| {
                item.file_name()
                    .into_string()
                    .ok()
                    .filter(|file_name| {
                        file_name.starts_with(format!("{}@", self.get_prefix()).as_str())
                    })
                    .map(|_| item)
            })
            .for_each(|item| {
                item.file_name().into_string().ok().map(|file_name| {
                    println!("{}", file_name);
                });
            });
        Ok(())
    }
}

pub struct SnmNpm {
    snm_config: SnmConfig,
    prefix: String,
}

impl SnmNpm {
    pub fn new(prefix: &str) -> Self {
        Self {
            snm_config: SnmConfig::new(),
            prefix: prefix.to_string(),
        }
    }
}

impl SharedBehavior for SnmNpm {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix)
            .join(&v)
            .join("package.json"))
    }
}

#[async_trait(?Send)]
impl ManagerTrait for SnmNpm {
    fn get_download_url(&self, v: &str) -> Result<String, SnmError> {
        let npm_registry = self.snm_config.get_npm_registry_host();
        Ok(format!(
            "{}/{}/-/{}-{}.tgz",
            npm_registry, &self.prefix, &self.prefix, &v
        ))
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_download_dir_path_buf()?
            .join(&self.prefix)
            .join(&v)
            .join(format!("{}@{}.tgz", &self.prefix, &v)))
    }

    fn get_downloaded_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_download_dir_path_buf()?
            .join(&self.prefix)
            .join(&v))
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix)
            .join(&v))
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix)
            .join(format!("{}-default", &v)))
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(&self.prefix))
    }

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError> {
        let npm_registry = self.snm_config.get_npm_registry_host();
        let download_url = format!("{}/{}/{}", npm_registry, &self.prefix, &v);

        let value: Value = reqwest::get(&download_url).await?.json().await?;

        let x = value
            .get("dist")
            .and_then(|dist| dist.get("shasum"))
            .and_then(|shasum| shasum.as_str())
            .map(|shasum| shasum.to_string())
            .ok_or(SnmError::NotFoundSha256ForNode(v.to_string()))?;

        Ok(x)
    }

    async fn get_actual_shasum(
        &self,
        downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError> {
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
        Ok(format!("{:x}", result))
    }

    fn get_host(&self) -> Option<String> {
        todo!("get_host")
    }

    async fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError> {
        let (dir_vec, _) = &dir_tuple;
        dir_vec.into_iter().for_each(|dir| {
            println!("{}", dir);
        });
        Ok(())
    }

    async fn show_list_remote(
        &self,
        dir_tuple: &(Vec<String>, Option<String>),
        all: bool,
    ) -> Result<(), SnmError> {
        todo!("show_list_remote")
    }

    fn get_shim_trait(&self) -> Box<dyn ShimTrait> {
        Box::new(SnmNpm::new(self.prefix.as_str()))
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress_tgz(
            &input_file_path_buf,
            &output_dir_path_buf,
            |output| output.join("package"),
            &mut Some(|_from: &PathBuf, _to: &PathBuf| {
                // print_warning!(stdout, "Waiting Decompress...")
            }),
        )?;
        Ok(())
    }
}

impl ShimTrait for SnmNpm {
    fn get_strict_shim_version(&self) -> Result<String, SnmError> {
        let package_json_path_buf = current_dir()?.join("package.json");

        let package_json = PackageJson::from_file_path(&package_json_path_buf)?;

        let package_manager = package_json.parse_package_manager()?;

        let version = package_manager.version;

        Ok(version)
    }

    fn get_strict_shim_binary_path_buf(&self, version: &str) -> Result<PathBuf, SnmError> {
        let node_binary_path_buf = self.get_runtime_binary_file_path_buf(&version)?;
        Ok(node_binary_path_buf)
    }

    fn download_condition(&self, version: &str) -> Result<bool, SnmError> {
        match self.snm_config.get_package_manager_install_strategy()? {
            snm_core::config::snm_config::InstallStrategy::Ask => Ok(Confirm::new()
                .with_prompt(format!(
                    "🤔 {} is not installed, do you want to install it ?",
                    &version
                ))
                .interact()?),
            snm_core::config::snm_config::InstallStrategy::Panic => {
                Err(SnmError::UnsupportedPackageManager {
                    name: self.prefix.to_string(),
                    version: version.to_string(),
                })
            }
            snm_core::config::snm_config::InstallStrategy::Auto => Ok(true),
        }
    }

    fn get_runtime_binary_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        let package_json_buf_path = self
            .snm_config
            .get_node_modules_dir_path_buf()?
            .join(self.prefix.to_string())
            .join(&v)
            .join("package.json");

        let mut hashmap = PackageJson::from_file_path(&package_json_buf_path)?.bin_to_hashmap()?;

        if let Some(bin) = hashmap.remove(&self.prefix) {
            return Ok(bin);
        } else {
            return Err(SnmError::UnknownError);
        }
    }

    fn check_default_version(
        &self,
        tuple: &(Vec<String>, Option<String>),
    ) -> Result<String, SnmError> {
        let (_, default_v_dir) = tuple;
        if let Some(v) = default_v_dir {
            return Ok(v.to_string());
        } else {
            return Err(SnmError::NotFoundDefaultPackageManager {
                name: self.prefix.to_string(),
            });
        }
    }
}
