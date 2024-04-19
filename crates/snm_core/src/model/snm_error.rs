use std::path::PathBuf;

use colored::*;
use thiserror::Error;

use crate::println_error;

#[derive(Error, Debug)]
pub enum SnmError {
    #[error("Download failed , The URL {download_url}")]
    DownloadFailed { download_url: String },

    #[error("File already exists {file_path}")]
    FileAlreadyExists { file_path: String },

    // 静默退出
    #[error("Silent exit")]
    SilentExit,

    #[error("Customer")]
    Error(String),

    #[error(
        "File {file_path} Sha256 verification failed, expected {expect} but received {actual}."
    )]
    Sha256VerificationFailed {
        file_path: String,
        expect: String,
        actual: String,
    },

    #[error("Resource 404 {download_url}")]
    ResourceNotFound { download_url: String },

    #[error("Not match packageManager expect {expect} but actual {actual}")]
    NotMatchPackageManager { expect: String, actual: String },

    #[error("Multi package manager lock file error")]
    MultiPackageManagerLockFileError { lock_file: Vec<String> },

    #[error("Can not find valid node binary , Please use `snm node default [version]` or create .node-version file.")]
    NotFoundDefaultNodeBinary,

    #[error("Package.json bin property not found , The absolute path {file_path}")]
    NotFoundPackageJsonBinProperty { file_path: String },

    #[error("Not found package.json file here {file_path}")]
    NotFoundPackageJsonFileError { file_path: PathBuf },

    #[error("Not found sha256 for node {0}")]
    NotFoundSha256ForNode(String),

    #[error("Not found default package manager {name}")]
    NotFoundDefaultPackageManager { name: String },

    #[error("Not found .node-version file")]
    NotFoundNodeVersionFileError { file_path: PathBuf },

    #[error("Not found binary {bin_name} {file_path}")]
    NotFoundBinaryFromPackageJsonBinProperty {
        bin_name: String,
        file_path: PathBuf,
    },

    #[error("Unknown install strategy")]
    UnknownInstallStrategy,

    #[error("Unsupported platform {os} {arch}")]
    UnsupportedPlatform { os: String, arch: String },

    #[error("UnSupportNodeVersion {version}")]
    UnsupportedNodeVersion { version: String },

    #[error("Unsupported {name}@{version}")]
    UnsupportedPackageManager { name: String, version: String },
}

pub fn handle_snm_error(error: SnmError) {
    match error {
        SnmError::NotFoundPackageJsonBinProperty { file_path } => {
            crate::println_error!(
                "Package.json bin property not found , The absolute path {}",
                file_path
            )
        }
        SnmError::DownloadFailed { download_url } => {
            crate::println_error!("Download failed , The URL {}", download_url)
        }
        SnmError::FileAlreadyExists { file_path } => {
            crate::println_error!("File already exists {}", file_path)
        }

        SnmError::NotFoundSha256ForNode(_) => {
            crate::println_error!("NotFoundSha256ForNode")
        }

        SnmError::ResourceNotFound { download_url } => {
            crate::println_error!("Resource 404: {}", download_url.bright_red())
        }
        SnmError::Sha256VerificationFailed {
            file_path,
            expect,
            actual,
        } => {
            crate::println_error!(
                "File {} verification sha256 failed , expected {} but actual {}",
                file_path,
                expect,
                actual
            )
        }

        SnmError::NotFoundDefaultNodeBinary => {
            crate::println_error!(
                "No Node.js default detected. Set it with {} or specify it in `.node-version` file here.","snm node default [version]".bright_green().bold()
            )
        }

        SnmError::NotMatchPackageManager { expect, actual } => {
            crate::println_error!(
                "No matching package manager found. You input {} but the current project is configured to use {}.",
                actual.bright_red().bold(),
                expect.bright_green().bold(),
            )
        }
        SnmError::MultiPackageManagerLockFileError { lock_file } => {
            crate::println_error!(
                "Multiple package manager lock files found: {} , Please remove the unnecessary ones.",
                lock_file.join(", ").bright_red()
            )
        }

        SnmError::NotFoundPackageJsonFileError { file_path } => {
            crate::println_error!(
                "Not found {}.",
                file_path.display().to_string().bright_black()
            )
        }
        SnmError::NotFoundNodeVersionFileError { file_path } => {
            crate::println_error!(
                "Not found {}.",
                file_path.display().to_string().bright_black()
            )
        }

        SnmError::UnsupportedPlatform { os, arch } => {
            crate::println_error!("{}-{} not supported", os, arch)
        }
        SnmError::UnknownInstallStrategy => {
            crate::println_error!("Unknown install strategy")
        }
        SnmError::UnsupportedNodeVersion { version } => {
            crate::println_error!("Unsupported node version {}", version.bright_red())
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
        SnmError::NotFoundBinaryFromPackageJsonBinProperty {
            bin_name,
            file_path,
        } => {
            println_error!("Not found binary {} {:?}", bin_name, file_path)
        }
        SnmError::SilentExit => {}
        SnmError::Error(message) => {
            crate::println_error!("{}", message)
        }
    }
    std::process::exit(1);
}
