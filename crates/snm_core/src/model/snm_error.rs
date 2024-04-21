use colored::*;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SnmError {
    // 静默退出
    #[error("Silent exit")]
    SilentExit,

    #[error("Customer")]
    Error(String),

    #[error("Resource 404 {download_url}")]
    ResourceNotFound { download_url: String },

    #[error("Not match packageManager expect {expect} but actual {actual}")]
    NotMatchPackageManager { expect: String, actual: String },

    #[error("Package.json bin property not found , The absolute path {file_path}")]
    NotFoundPackageManagerProperty { file_path: String },

    #[error("Not found node version file {file_path}")]
    NotFoundNodeVersionFile { file_path: String },

    #[error("Not found sha256 for node {0}")]
    NotFoundSha256ForNode(String),

    #[error("Not found default package manager {name}")]
    NotFoundDefaultPackageManager { name: String },

    #[error("Unsupported platform {os} {arch}")]
    UnsupportedPlatform { os: String, arch: String },

    #[error("Unsupported {name}@{version}")]
    UnsupportedPackageManager { name: String, version: String },
}

pub fn handle_snm_error(error: SnmError) {
    match error {
        SnmError::NotFoundPackageManagerProperty { file_path } => {
            crate::println_error!("Not found packageManager property in {}", file_path)
        }

        SnmError::NotFoundSha256ForNode(_) => {
            crate::println_error!("NotFoundSha256ForNode")
        }

        SnmError::ResourceNotFound { download_url } => {
            crate::println_error!("Resource 404: {}", download_url.bright_red())
        }

        SnmError::NotMatchPackageManager { expect, actual } => {
            crate::println_error!(
                "No matching package manager found. You input {} but the current project is configured to use {}.",
                actual.bright_red().bold(),
                expect.bright_green().bold(),
            )
        }

        SnmError::UnsupportedPlatform { os, arch } => {
            crate::println_error!("{}-{} not supported", os, arch)
        }

        SnmError::UnsupportedPackageManager { name, version } => {
            crate::println_error!(
                "Unsupported package manager {}",
                format!("{}@{}", name, version).bright_red(),
            )
        }

        SnmError::NotFoundDefaultPackageManager { name } => {
            crate::println_error!(
                "No {} default detected. Please configure package.json -> packageManager or use {} to set the default version.",
                name.bright_green().bold(),
                format!("snm {} default [version]", name).bright_green().bold()
            )
        }

        SnmError::SilentExit => {}
        SnmError::Error(message) => {
            crate::println_error!("{}", message)
        }
        SnmError::NotFoundNodeVersionFile { file_path } => {
            crate::println_error!("Not found node version file {}", file_path)
        }
    }
    std::process::exit(1);
}
