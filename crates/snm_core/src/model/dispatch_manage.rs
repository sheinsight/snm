use std::{fs, ops::Not, path::PathBuf};

use dialoguer::Confirm;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

pub struct DispatchManage {
    manager: Box<dyn AtomTrait>,
}

impl DispatchManage {
    pub fn new(manager: Box<dyn AtomTrait>) -> Self {
        Self { manager }
    }
}

// 分配
impl DispatchManage {
    pub async fn list(&self) -> Result<(), SnmError> {
        let dir_tuple = self.read_runtime_dir_name_vec()?;
        self.manager.show_list(&dir_tuple).await;
        Ok(())
    }

    pub async fn list_offline(&self) -> Result<(), SnmError> {
        let dir_tuple = self.read_runtime_dir_name_vec()?;
        self.manager.show_list_offline(&dir_tuple).await;
        Ok(())
    }

    pub async fn list_remote(&self, all: bool) -> Result<(), SnmError> {
        let dir_tuple = self.read_runtime_dir_name_vec()?;
        self.manager.show_list_remote(&dir_tuple, all).await?;
        Ok(())
    }

    pub async fn install(&self, v: &str) -> Result<(), SnmError> {
        let anchor_file_path_buf = self.manager.get_anchor_file_path_buf(&v)?;
        let current_version_dir = self.manager.get_runtime_dir_path_buf(&v)?;
        if anchor_file_path_buf.exists().not() {
            self.download(v).await?;
            return Ok(());
        }

        let confirm = Confirm::new()
            .with_prompt(format!(
                "🤔 v{} is already installed, do you want to reinstall it ?",
                &v
            ))
            .interact()?;

        if confirm {
            fs::remove_dir_all(&current_version_dir)?;
            self.download(v).await?;
        }

        let default_path_buf = self.manager.get_runtime_dir_for_default_path_buf()?;

        if default_path_buf.exists().not() {
            create_symlink(&current_version_dir, &default_path_buf)?;
        }

        Ok(())
    }

    pub async fn un_install(&self, v: &str) -> Result<(), SnmError> {
        let default_path_buf = self.manager.get_runtime_dir_for_default_path_buf()?;
        let current_version_path_buf = self.manager.get_runtime_dir_path_buf(&v)?;
        if fs::read_link(&default_path_buf)?.eq(&current_version_path_buf) {
            let msg = format!(
                "🤔 {} is default instance, do you want to uninstall it ?",
                &v
            );
            if Confirm::new().with_prompt(msg).interact()? {
                fs::remove_dir_all(&default_path_buf)?;
                fs::remove_dir_all(current_version_path_buf)?;
            }
        }
        Ok(())
    }

    pub async fn set_default(&self, v: &str) -> Result<(), SnmError> {
        if self.manager.get_anchor_file_path_buf(&v)?.exists().not() {
            if Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is not installed, do you want to install it ?",
                    &v
                ))
                .interact()?
            {
                self.install(&v).await?;
            }
        }

        let default_dir_path_buf = self.manager.get_runtime_dir_for_default_path_buf()?;

        if default_dir_path_buf.exists() {
            fs::remove_dir_all(&default_dir_path_buf)?;
        }

        let from_dir_path_buf = self.manager.get_runtime_dir_path_buf(&v)?;
        let to_dir_path_buf = self.manager.get_runtime_dir_for_default_path_buf()?;

        create_symlink(&from_dir_path_buf, &to_dir_path_buf)?;

        Ok(())
    }

    fn read_runtime_dir_name_vec(&self) -> Result<(Vec<String>, Option<String>), SnmError> {
        let runtime_dir_path_buf = self.manager.get_runtime_base_dir_path_buf()?;

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

                if file_name.eq("default") {
                    if let Some(o) = fs::read_link(dir_entry.path()).ok() {
                        if let Some(last) =
                            o.components().last().and_then(|x| x.as_os_str().to_str())
                        {
                            default_dir = Some(String::from(last));
                        }
                    }

                    return None;
                }

                return Some(file_name);
            })
            .collect::<Vec<String>>();

        Ok((dir_name_vec, default_dir))
    }

    async fn download(&self, v: &str) -> Result<(), SnmError> {
        let download_url = self.manager.get_download_url(v);
        let downloaded_file_path_buf = self.manager.get_downloaded_file_path_buf(v)?;

        DownloadBuilder::new()
            .retries(3)
            .write_strategy(WriteStrategy::Nothing)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        let runtime_dir_path_buf = self.manager.get_runtime_dir_path_buf(v)?;

        if runtime_dir_path_buf.exists() {
            fs::remove_dir_all(&runtime_dir_path_buf)?;
        }

        self.manager
            .decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf)?;

        fs::remove_file(&downloaded_file_path_buf)?;

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
