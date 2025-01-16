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

#[cfg(target_os = "windows")]
fn init_windows_network() -> anyhow::Result<()> {
  use std::sync::Once;
  static INIT: Once = Once::new();

  INIT.call_once(|| {
    // 确保 WinSock 初始化
    unsafe {
      let mut wsa_data = std::mem::zeroed();
      let result = winapi::um::winsock2::WSAStartup(0x202, &mut wsa_data);
      if result != 0 {
        let error = winapi::um::winsock2::WSAGetLastError();
        panic!("Failed to initialize WinSock: error code {}", error);
      }
      println!("WinSock initialized successfully");
      println!(
        "WinSock version: {}.{}",
        wsa_data.wVersion & 0xFF,
        wsa_data.wVersion >> 8
      );
    }
  });

  Ok(())
}

#[cfg(target_os = "windows")]
fn check_network_status() {
  use std::process::Command;

  println!("\nChecking network status before download:");

  // 检查 TCP 连接
  let output = Command::new("netstat")
    .args(["-n", "-p", "TCP"])
    .output()
    .expect("Failed to execute netstat");
  println!(
    "Active TCP connections:\n{}",
    String::from_utf8_lossy(&output.stdout)
  );

  // 检查本地端口
  let output = Command::new("netstat")
    .args(["-an", "|", "findstr", "LISTENING"])
    .output()
    .expect("Failed to execute netstat");
  println!(
    "Listening ports:\n{}",
    String::from_utf8_lossy(&output.stdout)
  );
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

      // 在发送请求前调用
      #[cfg(target_os = "windows")]
      check_network_status();

      // 在创建 reqwest client 之前调用
      #[cfg(target_os = "windows")]
      init_windows_network()?;

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
        .no_proxy()
        // .tcp_nodelay(true)
        .build()?;

      let response = client
        .head(download_url)
        .timeout(Duration::from_secs(60))
        .send()
        .await?;

      if !response.status().is_success() {
        anyhow::bail!(
          "Head request failed, Http status code not ok {} : {:?}",
          response.status(),
          response
        );
      }

      let response = client
        .get(download_url)
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| {
          println!("\nRequest failed:");
          println!("URL: {}", download_url);
          println!("Error: {:?}", e);

          println!("Caused by: {:?}", e);

          #[cfg(target_os = "windows")]
          unsafe {
            println!("WSA Error: {}", winapi::um::winsock2::WSAGetLastError());
          }
          e
        })?;

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
