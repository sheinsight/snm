use std::{fs, ops::Not, path::PathBuf};

use dialoguer::Confirm;

use crate::{
    config::SnmConfig,
    print_warning,
    utils::download::{DownloadBuilder, WriteStrategy},
};
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

use super::{trait_manage::ManageTrait, SnmError};

pub struct DispatchManage {
    manager: Box<dyn ManageTrait>,
    snm_config: SnmConfig,
}

impl DispatchManage {
    pub fn new(manager: Box<dyn ManageTrait>) -> Self {
        let snm_config = SnmConfig::new();
        Self {
            manager,
            snm_config,
        }
    }
}

// 分配
impl DispatchManage {
    pub async fn ensure_strict_package_manager(
        &self,
        bin_name: &str,
    ) -> Result<(String, PathBuf), SnmError> {
        let shim_trait = self.manager.get_shim_trait();

        let version = shim_trait.get_strict_shim_version()?;

        let anchor_file_path_buf = shim_trait.get_anchor_file_path_buf(&version);

        if anchor_file_path_buf.exists().not() {
            if shim_trait.download_condition(&version)? {
                self.download(&version).await?;
            } else {
                return Err(SnmError::SilentExit);
            }
        }

        let binary_path_buf = shim_trait.get_strict_shim_binary_path_buf(&bin_name, &version)?;

        return Ok((version.to_string(), binary_path_buf));
    }

    pub async fn proxy_process(&self, bin_name: &str) -> Result<(String, PathBuf), SnmError> {
        let shim_trait = self.manager.get_shim_trait();

        if self.snm_config.get_strict() {
            let version = shim_trait.get_strict_shim_version()?;

            let anchor_file_path_buf = shim_trait.get_anchor_file_path_buf(&version);

            if anchor_file_path_buf.exists().not() {
                if shim_trait.download_condition(&version)? {
                    self.download(&version).await?;
                } else {
                    return Err(SnmError::SilentExit);
                }
            }

            let binary_path_buf = shim_trait.get_strict_shim_binary_path_buf(&version, bin_name)?;

            return Ok((version, binary_path_buf));
        } else {
            let tuple = self.read_runtime_dir_name_vec()?;

            let v = shim_trait.check_default_version(&tuple)?;

            let binary_path_buf = shim_trait.get_runtime_binary_file_path_buf(&bin_name, &v)?;

            return Ok((v, binary_path_buf));
        }
    }

    pub async fn list(&self) -> Result<(), SnmError> {
        let dir_tuple = self.read_runtime_dir_name_vec()?;
        self.manager.show_list(&dir_tuple).await?;
        Ok(())
    }

    pub async fn list_remote(&self, all: bool) -> Result<(), SnmError> {
        let dir_tuple = self.read_runtime_dir_name_vec()?;
        self.manager.show_list_remote(&dir_tuple, all).await?;
        Ok(())
    }

    pub async fn install(&self, v: &str) -> Result<(), SnmError> {
        let anchor_file_path_buf = self.manager.get_anchor_file_path_buf(&v);

        if anchor_file_path_buf.exists().not() {
            self.download(v).await?;
            return Ok(());
        }

        if Confirm::new()
            .with_prompt(format!(
                "🤔 v{} is already installed, do you want to reinstall it ?",
                &v
            ))
            .interact()
            .expect("install Confirm error")
            .not()
        {
            return Ok(());
        }

        self.download(v).await?;

        Ok(())
    }

    pub async fn un_install(&self, v: &str) -> Result<(), SnmError> {
        let (dir_name_vec, default_v) = self.read_runtime_dir_name_vec()?;

        if dir_name_vec.is_empty() || dir_name_vec.iter().any(|item| item == &v).not() {
            print_warning!("Not found {}", &v);
            return Ok(());
        }

        if let Some(d_v) = default_v {
            if &d_v == &v {
                if Confirm::new()
                    .with_prompt(format!(
                        "🤔 {} is default instance, do you want to uninstall it ?",
                        &d_v
                    ))
                    .interact()
                    .expect("un_install Confirm error")
                {
                    let default_path_buf = self
                        .manager
                        .get_runtime_dir_path_buf(format!("{}-default", &v).as_str());

                    fs::remove_dir_all(&default_path_buf).expect(
                        format!(
                            "un_install remove_dir_all error {:?}",
                            &default_path_buf.display()
                        )
                        .as_str(),
                    );
                } else {
                    return Ok(());
                }
            }
        }

        let runtime_dir_path_buf = self.manager.get_runtime_dir_path_buf(&v);
        fs::remove_dir_all(&runtime_dir_path_buf).expect(
            format!(
                "un_install remove_dir_all error {:?}",
                &runtime_dir_path_buf.display()
            )
            .as_str(),
        );

        Ok(())
    }

    pub async fn set_default(&self, v: &str) -> Result<(), SnmError> {
        let (_, default_v) = self.read_runtime_dir_name_vec()?;

        let anchor_file_path_buf = self.manager.get_anchor_file_path_buf(&v);

        if anchor_file_path_buf.exists().not() {
            if Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is not installed, do you want to install it ?",
                    &v
                ))
                .interact()
                .expect("set_default Confirm error")
            {
                self.install(&v).await?;
            } else {
                return Ok(());
            }
        }

        if let Some(d_v) = default_v {
            let default_dir_path_buf = self.manager.get_runtime_dir_for_default_path_buf(&d_v);
            fs::remove_dir_all(&default_dir_path_buf).expect(
                format!(
                    "set_default remove_dir_all error {:?}",
                    &default_dir_path_buf.display()
                )
                .as_str(),
            );
        }

        let from_dir_path_buf = self.manager.get_runtime_dir_path_buf(&v);
        let to_dir_path_buf = self.manager.get_runtime_dir_for_default_path_buf(&v);

        create_symlink(&from_dir_path_buf, &to_dir_path_buf).expect(
            format!(
                "set_default create_symlink error from: {:?} to: {:?}",
                &from_dir_path_buf.display(),
                &to_dir_path_buf.display()
            )
            .as_str(),
        );

        Ok(())
    }

    fn read_runtime_dir_name_vec(&self) -> Result<(Vec<String>, Option<String>), SnmError> {
        let runtime_dir_path_buf = self.manager.get_runtime_base_dir_path_buf();

        let mut default_dir = None;

        if runtime_dir_path_buf.exists().not() {
            // TODO here create not suitable , should be find a better way
            fs::create_dir_all(&runtime_dir_path_buf).expect(
                format!(
                    "read_runtime_dir_name_vec create_dir_all error {:?}",
                    &runtime_dir_path_buf.display()
                )
                .as_str(),
            );
        }

        let dir_name_vec = runtime_dir_path_buf
            .read_dir()
            .expect(
                format!(
                    "read_runtime_dir_name_vec read_dir error {:?}",
                    &runtime_dir_path_buf.display()
                )
                .as_str(),
            )
            .filter_map(|dir_entry| dir_entry.ok())
            .filter(|dir_entry| dir_entry.path().is_dir())
            .filter_map(|dir_entry| {
                let file_name = dir_entry.file_name().into_string().ok()?;
                if file_name.ends_with("-default") {
                    default_dir = Some(file_name.trim_end_matches("-default").to_string());
                    return None;
                }

                return Some(file_name);
            })
            .collect::<Vec<String>>();

        Ok((dir_name_vec, default_dir))
    }

    async fn download(&self, v: &str) -> Result<(), SnmError> {
        let download_url = self.manager.get_download_url(v);
        let downloaded_file_path_buf = self.manager.get_downloaded_file_path_buf(v);
        DownloadBuilder::new()
            .retries(3)
            .write_strategy(WriteStrategy::Nothing)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        let expect_sha256 = self.manager.get_expect_shasum(v).await?;

        let actual_sha256 = self
            .manager
            .get_actual_shasum(&downloaded_file_path_buf)
            .await?;

        if expect_sha256 != actual_sha256 {
            Err(SnmError::Sha256VerificationFailed {
                file_path: downloaded_file_path_buf.display().to_string(),
                expect: expect_sha256,
                actual: actual_sha256,
            })?;
        }

        let runtime_dir_path_buf = self.manager.get_runtime_dir_path_buf(v);
        self.manager
            .decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf)?;

        fs::remove_file(&downloaded_file_path_buf).expect(
            format!(
                "download remove_file error {:?}",
                &downloaded_file_path_buf.display()
            )
            .as_str(),
        );

        Ok(())
    }
}

fn create_symlink(original: &PathBuf, link: &PathBuf) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        unix_fs::symlink(original, link)
    }
    #[cfg(windows)]
    {
        windows_fs::symlink_dir(original, link)
    }
}
