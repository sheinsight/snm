use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget};
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
    write_strategy: WriteStrategy,
}

impl DownloadBuilder {
    pub fn new() -> Self {
        Self {
            retries: 0,
            write_strategy: WriteStrategy::WriteAfterDelete,
        }
    }

    pub fn retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }

    pub fn write_strategy(mut self, write_strategy: WriteStrategy) -> Self {
        self.write_strategy = write_strategy;
        self
    }

    pub async fn download<P: AsRef<Path>>(&mut self, download_url: &str, abs_path: P) -> P {
        let mut attempts = 0;
        while attempts < (self.retries + 1) {
            let result = self.original_download(download_url, &abs_path).await;
            match result {
                Ok(_) => {
                    // 假设下载成功，返回Ok(())
                    return abs_path;
                }
                Err(_) => {
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
            }
        }

        let msg = format!(
            "Download {} failed after {} attempts",
            download_url, self.retries
        );
        panic!("{msg}");
    }

    pub async fn original_download<P: AsRef<Path>>(
        &mut self,
        download_url: &str,
        abs_path: P,
    ) -> Result<P, ()> {
        let abs_path_ref = abs_path.as_ref();
        if abs_path_ref.exists() {
            match self.write_strategy {
                WriteStrategy::Error => {
                    let msg = format!("file already exists {}", &abs_path_ref.display());
                    panic!("{msg}");
                }
                WriteStrategy::WriteAfterDelete => {
                    std::fs::remove_file(&abs_path_ref).expect(
                        format!("download remove file error {:?}", &abs_path_ref.display())
                            .as_str(),
                    );
                }
                WriteStrategy::Nothing => {
                    // 如果选择不覆盖已存在的文件，则直接返回成功
                    return Ok(abs_path);
                }
            };
        }

        if let Some(parent) = abs_path_ref.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .expect(format!("create dir error {}", &parent.display()).as_str());
            }

            let response = reqwest::Client::new()
                .get(download_url)
                .timeout(Duration::from_secs(60))
                .send()
                .await
                .expect(format!("download error {}", &download_url).as_str());

            let response_status = response.status();

            if response_status.as_str() == "404" {
                let msg = format!("ResourceNotFound {}", download_url.to_string());
                panic!("{msg}");
            }

            if !response_status.is_success() {
                let msg = format!("download error {}", response_status.as_str());
                panic!("{msg}");
            }

            let total_size = response.content_length();

            let mut file = tokio::fs::File::create(abs_path_ref).await.expect(
                format!("download create file error {:?}", &abs_path_ref.display()).as_str(),
            );

            let mut stream = response.bytes_stream();

            let progress_bar = ProgressBar::with_draw_target(
                Some(total_size.unwrap_or(0)),
                ProgressDrawTarget::stdout(),
            );

            progress_bar.set_style(
                indicatif::ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] {bar:25.green/white.dim} {bytes}/{total_bytes} {wide_msg:.dim}",
                )
                .unwrap()
                .progress_chars("━━"),
            );

            progress_bar.set_message(download_url.to_string());

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.expect("download stream chunk error");

                file.write_all(&chunk).await.expect(
                    format!("download write file error {:?}", &abs_path_ref.display()).as_str(),
                );

                progress_bar.inc(chunk.len() as u64);
            }

            file.flush().await.expect(
                format!("download flush file error {:?}", &abs_path_ref.display()).as_str(),
            );

            progress_bar.finish();
        }

        Ok(abs_path)
    }
}
