use std::path::Path;
use std::time::Duration;

use backoff::ExponentialBackoff;
use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone)]
pub enum WriteStrategy {
  Error,
  WriteAfterDelete,
  Nothing,
}

#[derive(Clone)]
pub struct DownloadBuilder {
  retries: u8,
  timeout: u64,
  max_elapsed_time: u64,
  write_strategy: WriteStrategy,
  client: reqwest::Client,
}

impl DownloadBuilder {
  pub fn new() -> anyhow::Result<Self> {
    Ok(Self {
      retries: 0,
      timeout: 30,
      max_elapsed_time: 60,
      write_strategy: WriteStrategy::WriteAfterDelete,
      client: reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?,
    })
  }

  pub fn retries(mut self, retries: u8) -> Self {
    self.retries = retries;
    self
  }

  pub fn write_strategy(mut self, write_strategy: WriteStrategy) -> Self {
    self.write_strategy = write_strategy;
    self
  }

  pub fn max_elapsed_time(mut self, max_elapsed_time: u64) -> Self {
    self.max_elapsed_time = max_elapsed_time;
    self
  }

  pub fn timeout(mut self, timeout: u64) -> Self {
    self.timeout = timeout;
    self
  }

  pub async fn download<P: AsRef<Path> + Clone>(
    &mut self,
    download_url: &str,
    abs_path: P,
  ) -> anyhow::Result<P> {
    let backoff = ExponentialBackoff {
      initial_interval: Duration::from_millis(100),
      max_interval: Duration::from_secs(10),
      multiplier: 2.0,
      max_elapsed_time: Some(Duration::from_secs(self.max_elapsed_time)),
      ..Default::default()
    };

    let abs_path_clone = abs_path.clone();

    let operation = move || {
      let owned_path = abs_path.clone();
      let owned_url = download_url.to_string();
      let mut this = self.clone();
      async move {
        match this.download_with_progress(&owned_url, &owned_path).await {
          Ok(_) => Ok(owned_path),
          Err(err) => {
            // tracing::warn!(
            //   "Download failed for URL: {}. Error: {:?}",
            //   owned_url.bright_red(),
            //   err
            // );
            eprintln!(
              r#"Download failed for URL: {}. 
Error: {:?}"#,
              owned_url.bright_red(),
              err
            );
            Err(backoff::Error::transient(err))
          }
        }
      }
    };

    // 使用 backoff 提供的重试机制
    backoff::future::retry(backoff, operation)
      .await
      .map_err(|e| anyhow::anyhow!("Download failed after retries: {}", e))?;

    Ok(abs_path_clone)
  }

  pub async fn download_with_progress<P: AsRef<Path> + Clone>(
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

      let response = self
        .client
        .get(download_url)
        .timeout(Duration::from_secs(60))
        .send()
        .await?;

      if !response.status().is_success() {
        anyhow::bail!(
          "Get request failed, Http status code not ok {} : {:?}",
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
