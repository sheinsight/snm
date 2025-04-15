use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platform {
  pub os: String,
  pub arch: String,
  pub ext: String,
}

fn os() -> &'static str {
  #[cfg(target_os = "macos")]
  {
    "darwin"
  }
  #[cfg(target_os = "windows")]
  {
    "win"
  }
  #[cfg(target_os = "linux")]
  {
    "linux"
  }
  #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
  {
    "unknown"
  }
}

fn arch() -> &'static str {
  #[cfg(target_arch = "x86_64")]
  {
    "x64"
  }
  #[cfg(target_arch = "aarch64")]
  {
    "arm64"
  }
  #[cfg(target_arch = "arm")]
  {
    "armv7l"
  }
  #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm")))]
  {
    "unknown"
  }
}

fn ext() -> &'static str {
  #[cfg(target_os = "windows")]
  {
    "zip"
  }
  #[cfg(not(target_os = "windows"))]
  {
    "tar.xz"
  }
}

impl Default for Platform {
  fn default() -> Self {
    Self {
      os: os().to_string(),
      arch: arch().to_string(),
      ext: ext().to_string(),
    }
  }
}
