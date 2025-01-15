use std::path::Path;
use std::time::Duration;

use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget};
// use reqwest::Client;
use snm_utils::snm_error::SnmError;
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
  timeout: u64,
  write_strategy: WriteStrategy,
}

impl DownloadBuilder {
  pub fn new() -> Self {
    Self {
      retries: 0,
      timeout: 30,
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

  pub fn timeout(mut self, timeout: u64) -> Self {
    self.timeout = timeout;
    self
  }

  pub async fn download<P: AsRef<Path>>(
    &mut self,
    download_url: &str,
    abs_path: P,
  ) -> Result<P, SnmError> {
    let mut attempts = 0;

    while attempts < (self.retries + 1) {
      let result = self.original_download(download_url, &abs_path).await;
      match result {
        Ok(_) => {
          return Ok(abs_path);
        }
        Err(err) => {
          attempts += 1;

          if attempts <= self.retries {
            eprintln!(
              "Download failed, attempting retry {} . The URL is {} . Error: {:?}",
              attempts.to_string().bright_yellow().bold(),
              download_url.bright_red(),
              err.source()
            );
          }
          sleep(Duration::from_millis((self.retries + 10).into())).await;
        }
      }
    }

    return Err(SnmError::ExceededMaxRetries(download_url.to_string()));
  }

  pub async fn original_download<P: AsRef<Path>>(
    &mut self,
    download_url: &str,
    abs_path: P,
  ) -> anyhow::Result<P> {
    let abs_path_ref = abs_path.as_ref();
    if abs_path_ref.exists() {
      match self.write_strategy {
        WriteStrategy::Error => {
          anyhow::bail!("File already exists: {}", abs_path_ref.display());
        }
        WriteStrategy::WriteAfterDelete => {
          std::fs::remove_file(&abs_path_ref)?;
        }
        WriteStrategy::Nothing => {
          return Ok(abs_path);
        }
      };
    }

    if let Some(parent) = abs_path_ref.parent() {
      if !parent.exists() {
        std::fs::create_dir_all(parent)?;
      }

      #[cfg(target_os = "windows")]
      let client = reqwest::Client::builder()
        .use_native_tls() // Windows 下使用系统 TLS
        .timeout(Duration::from_secs(30))
        .build()?;

      #[cfg(not(target_os = "windows"))]
      let client = reqwest::Client::builder()
        // 其他平台使用默认的 rustls
        // .use_native_tls() // Windows 下使用系统 TLS
        .timeout(Duration::from_secs(30))
        .build()?;

      let response = client
        .get(download_url)
        .timeout(Duration::from_secs(60))
        .send()
        .await?;

      if !response.status().is_success() {
        anyhow::bail!(
          "Http status code not ok {} : {:?}",
          response.status(),
          response
        );
      }

      let total_size = response.content_length();

      let mut file = tokio::fs::File::create(abs_path_ref).await?;

      let mut stream = response.bytes_stream();

      let progress_bar = ProgressBar::with_draw_target(
        Some(total_size.unwrap_or(100)),
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
        let chunk = chunk?;

        file.write_all(&chunk).await?;

        progress_bar.inc(chunk.len() as u64);
      }

      file.flush().await?;

      progress_bar.finish();
    }

    Ok(abs_path)
  }
}
