use crate::model::SnmError;
use colored::*;
use futures_util::StreamExt;
use std::path::Path;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

#[derive(Debug)]
pub enum WriteStrategy {
    Error,
    WriteAfterDelete,
    Nothing,
}

pub struct DownloadBuilder {
    retries: u8,
    progress: Option<Box<dyn FnMut(u64, u64)>>,
    write_strategy: WriteStrategy,
}

impl DownloadBuilder {
    pub fn new() -> Self {
        Self {
            retries: 0,
            progress: None,
            write_strategy: WriteStrategy::WriteAfterDelete,
        }
    }

    pub fn retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }

    pub fn progress<F: FnMut(u64, u64) + 'static>(mut self, progress: F) -> Self {
        self.progress = Some(Box::new(progress));
        self
    }

    pub fn write_strategy(mut self, write_strategy: WriteStrategy) -> Self {
        self.write_strategy = write_strategy;
        self
    }

    pub async fn download<P: AsRef<Path>>(
        &mut self,
        download_url: &str,
        abs_path: P,
    ) -> Result<P, SnmError> {
        let mut attempts = 0;
        while attempts < (self.retries + 1) {
            match self.original_download(download_url, &abs_path).await {
                Ok(_) => {
                    // 假设下载成功，返回Ok(())
                    return Ok(abs_path);
                }
                Err(e) => match e {
                    SnmError::ResourceNotFound { download_url: _ } => {
                        return Err(e);
                    }
                    _ => {
                        attempts += 1;

                        if attempts <= self.retries {
                            crate::println_error!(
                                "Download failed, attempting retry {} . The URL is {} .",
                                attempts.to_string().bright_yellow().bold(),
                                download_url.bright_red()
                            );
                        }
                        sleep(Duration::from_millis((self.retries + 10).into())).await;
                    }
                },
            }
        }
        Err(SnmError::DownloadFailed {
            download_url: download_url.to_string(),
        })
    }

    pub async fn original_download<P: AsRef<Path>>(
        &mut self,
        download_url: &str,
        abs_path: P,
    ) -> Result<P, SnmError> {
        let abs_path_ref = abs_path.as_ref();
        if abs_path_ref.exists() {
            match self.write_strategy {
                WriteStrategy::Error => {
                    return Err(SnmError::FileAlreadyExists {
                        file_path: abs_path_ref.display().to_string(),
                    });
                }
                WriteStrategy::WriteAfterDelete => {
                    std::fs::remove_file(&abs_path_ref)?;
                }
                WriteStrategy::Nothing => {
                    // 如果选择不覆盖已存在的文件，则直接返回成功
                    return Ok(abs_path);
                }
            };
        }

        if let Some(parent) = abs_path_ref.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }

            let client = reqwest::Client::new();

            let response = client.get(download_url).send().await?;

            let response_status = response.status();

            if response_status.as_str() == "404" {
                return Err(SnmError::ResourceNotFound {
                    download_url: download_url.to_string(),
                });
            }

            if !response_status.is_success() {
                return Err(SnmError::DownloadFailed {
                    download_url: download_url.to_string(),
                });
            }

            let total_size = response.content_length();

            let mut file = tokio::fs::File::create(abs_path_ref).await?;

            let mut stream = response.bytes_stream();

            let mut downloaded_size: u64 = Default::default();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;

                file.write_all(&chunk).await?;

                downloaded_size += chunk.len() as u64;

                if let Some(progress) = &mut self.progress {
                    progress(downloaded_size, total_size.unwrap_or(0));
                }
            }

            file.flush().await?;
        }

        Ok(abs_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
