pub const YARN_PACKAGE_NAME: &str = "yarn";
pub const YARNPKG_PACKAGE_NAME: &str = "@yarnpkg/cli-dist";

pub const NODE_VERSION_FILE_NAME: &str = ".node-version";

pub const SNM_PREFIX: &str = "SNM";

pub const fn os() -> &'static str {
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

pub const fn arch() -> &'static str {
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

pub const fn ext() -> &'static str {
  #[cfg(target_os = "windows")]
  {
    "zip"
  }
  #[cfg(not(target_os = "windows"))]
  {
    "tar.xz"
  }
}
