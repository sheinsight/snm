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
        let anchor_file_path_buf = match self.manager.get_anchor_file_path_buf(&v) {
            Ok(anchor_file_path_buf) => anchor_file_path_buf,
            Err(_) => panic!("set_default get_anchor_file_path_buf error"),
        };

        if anchor_file_path_buf.exists().not() {
            self.download(v).await?;
            return Ok(());
        }

        let confirm = Confirm::new()
            .with_prompt(format!(
                "🤔 v{} is already installed, do you want to reinstall it ?",
                &v
            ))
            .interact()
            .expect("install Confirm error");

        if confirm {
            self.download(v).await?;
        }

        Ok(())
    }

    pub async fn un_install(&self, v: &str) -> Result<(), SnmError> {
        let (dir_name_vec, default_v) = self.read_runtime_dir_name_vec()?;

        if dir_name_vec.is_empty() || dir_name_vec.iter().any(|item| item == &v).not() {
            let msg = format!("Not found {}", &v);
            panic!("{msg}");
        }

        if let Some(d_v) = default_v {
            if &d_v == &v {
                let result = Confirm::new()
                    .with_prompt(format!(
                        "🤔 {} is default instance, do you want to uninstall it ?",
                        &d_v
                    ))
                    .interact()
                    .expect("un_install Confirm error");
                if result {
                    let default_path_buf = self
                        .manager
                        .get_runtime_dir_path_buf(format!("{}-default", &v).as_str())?;

                    fs::remove_dir_all(&default_path_buf).expect(
                        format!(
                            "un_install remove_dir_all error {:?}",
                            &default_path_buf.display()
                        )
                        .as_str(),
                    );
                }
            }
        }

        let runtime_dir_path_buf = self.manager.get_runtime_dir_path_buf(&v)?;
        let msg = format!(
            "un_install remove_dir_all error {:?}",
            &runtime_dir_path_buf.display()
        );
        fs::remove_dir_all(&runtime_dir_path_buf).expect(&msg);

        Ok(())
    }

    pub async fn set_default(&self, v: &str) -> Result<(), SnmError> {
        let (_, default_v) = self.read_runtime_dir_name_vec()?;

        let anchor_file_path_buf = match self.manager.get_anchor_file_path_buf(&v) {
            Ok(anchor_file_path_buf) => anchor_file_path_buf,
            Err(_) => panic!("set_default get_anchor_file_path_buf error"),
        };

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
            }
        }

        if let Some(d_v) = default_v {
            let default_dir_path_buf = self.manager.get_runtime_dir_for_default_path_buf(&d_v)?;
            fs::remove_dir_all(&default_dir_path_buf).expect(
                format!(
                    "set_default remove_dir_all error {:?}",
                    &default_dir_path_buf.display()
                )
                .as_str(),
            );
        }

        let from_dir_path_buf = self.manager.get_runtime_dir_path_buf(&v)?;
        let to_dir_path_buf = self.manager.get_runtime_dir_for_default_path_buf(&v)?;

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
