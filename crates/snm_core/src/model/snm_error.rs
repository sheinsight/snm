use colored::*;
use std::{env::VarError, io::stdout, path::StripPrefixError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnmError {
    #[error("Read dir failed {dir_path}")]
    ReadDirFailed { dir_path: String },

    #[error("can not find use home dir")]
    GetHomeDirError,

    #[error("can not find node binary dir config")]
    NotFoundBinDirConfig,

    #[error("Can not find download dir config")]
    NotFoundDownloadDirConfig,

    #[error("can not find node_modules dir config")]
    NotFoundNodeModulesDirConfig,

    #[error("Can not find valid node binary , Please use `snm node default [version]` or create .node-version file.")]
    NotFoundDefaultNodeBinary,

    #[error("Create dir failed {dir_path}")]
    CreateDirFailed { dir_path: String },

    #[error("Parse json error {file_path}")]
    SerdeJsonError { file_path: String },

    #[error("Read file to string error {file_path}")]
    ReadFileToStringError { file_path: String },

    #[error("Package.json bin property unknown type error , The absolute path {file_path}")]
    PackageJsonBinPropertyUnknownTypeError { file_path: String },

    #[error("Package.json bin property not found , The absolute path {file_path}")]
    PackageJsonBinPropertyNotFound { file_path: String },

    #[error("Download failed , The URL {download_url}")]
    DownloadFailed { download_url: String },

    #[error("File already exists {file_path}")]
    FileAlreadyExists { file_path: String },

    #[error("Unknown error")]
    UnknownError,

    #[error("Not found valid node version")]
    EmptyNodeList,

    #[error("Not found sha256 for node {0}")]
    NotFoundSha256ForNode(String),

    #[error(
        "File {file_path} Sha256 verification failed, expected {expect} but received {actual}."
    )]
    Sha256VerificationFailed {
        file_path: String,
        expect: String,
        actual: String,
    },

    #[error("User refuse to install node")]
    RefuseToInstallNode,

    #[error("Resource 404 {download_url}")]
    ResourceNotFound { download_url: String },

    #[error("parse package manager config error")]
    ParsePackageManagerConfigError { raw_value: String },

    #[error("No packageManager config error {file_path}")]
    NoPackageManagerError { file_path: String },

    #[error("Not match packageManager expect {expect} but actual {actual}")]
    NotMatchPackageManager { expect: String, actual: String },

    #[error("Multi package manager lock file error")]
    MultiPackageManagerLockFileError { lock_file: Vec<String> },
}

impl From<VarError> for SnmError {
    fn from(_error: VarError) -> Self {
        SnmError::UnknownError
    }
}

impl From<semver::Error> for SnmError {
    fn from(_error: semver::Error) -> Self {
        SnmError::UnknownError
    }
}

impl From<regex::Error> for SnmError {
    fn from(_error: regex::Error) -> Self {
        SnmError::UnknownError
    }
}

impl From<std::io::Error> for SnmError {
    fn from(_error: std::io::Error) -> Self {
        SnmError::UnknownError
    }
}

impl From<StripPrefixError> for SnmError {
    fn from(_error: StripPrefixError) -> Self {
        SnmError::UnknownError
    }
}

impl From<reqwest::Error> for SnmError {
    fn from(_error: reqwest::Error) -> Self {
        SnmError::UnknownError
    }
}

impl From<dialoguer::Error> for SnmError {
    fn from(_error: dialoguer::Error) -> Self {
        SnmError::UnknownError
    }
}

pub fn handle_snm_error(error: SnmError) {
    let mut stdout = stdout();
    match error {
        SnmError::CreateDirFailed { dir_path } => {
            crate::println_error!(stdout, "Create dir failed {}", dir_path)
        }
        SnmError::SerdeJsonError { file_path } => {
            crate::println_error!(stdout, "Parse json error {}", file_path)
        }
        SnmError::ReadFileToStringError { file_path } => {
            crate::println_error!(stdout, "Read file to string error {}", file_path)
        }
        SnmError::PackageJsonBinPropertyUnknownTypeError { file_path } => {
            crate::println_error!(
                stdout,
                "Package.json bin property unknown type error , The absolute path {}",
                file_path
            )
        }
        SnmError::PackageJsonBinPropertyNotFound { file_path } => {
            crate::println_error!(
                stdout,
                "Package.json bin property not found , The absolute path {}",
                file_path
            )
        }
        SnmError::DownloadFailed { download_url } => {
            crate::println_error!(stdout, "Download failed , The URL {}", download_url)
        }
        SnmError::FileAlreadyExists { file_path } => {
            crate::println_error!(stdout, "File already exists {}", file_path)
        }
        SnmError::UnknownError => {
            crate::println_error!(stdout, "Unknown Error")
        }
        SnmError::EmptyNodeList => {
            crate::println_error!(
                stdout,
                "Node list is empty, please use {} to get the latest version.",
                "snm node list-remote".bright_green().bold()
            )
        }
        SnmError::NotFoundSha256ForNode(_) => {
            crate::println_error!(stdout, "NotFoundSha256ForNode")
        }
        SnmError::RefuseToInstallNode => todo!("RefuseToInstallNode"),
        SnmError::ResourceNotFound { download_url } => {
            crate::println_error!(stdout, "Resource 404: {}", download_url.bright_red())
        }
        SnmError::Sha256VerificationFailed {
            file_path,
            expect,
            actual,
        } => {
            crate::println_error!(
                stdout,
                "File {} verification sha256 failed , expected {} but actual {}",
                file_path,
                expect,
                actual
            )
        }
        SnmError::ReadDirFailed { dir_path } => {
            crate::println_error!(stdout, "Read dir failed {}", dir_path)
        }
        SnmError::GetHomeDirError => {
            crate::println_error!(stdout, "Unable to retrieve user directory correctly ! ")
        }
        SnmError::NotFoundBinDirConfig => {
            crate::println_error!(
                stdout,
                "Unable to get the {} environment variable.",
                "SNM_NODE_BIN_DIR".bright_red()
            )
        }
        SnmError::NotFoundDownloadDirConfig => {
            crate::println_error!(
                stdout,
                "Unable to get the {} environment variable.",
                "SNM_DOWNLOAD_DIR".bright_red()
            )
        }
        SnmError::NotFoundNodeModulesDirConfig => {
            crate::println_error!(
                stdout,
                "Unable to get the {} environment variable.",
                "SNM_NODE_MODULES_DIR".bright_red()
            )
        }
        SnmError::NotFoundDefaultNodeBinary => {
            crate::println_error!(
                stdout,
                "No Node.js default detected. Set it with {} or specify it in `.node-version` file here.","snm node default [version]".bright_green().bold()
            )
        }
        SnmError::ParsePackageManagerConfigError { raw_value } => {
            crate::println_error!(stdout, "Parse package manager config error: {}", raw_value)
        }
        SnmError::NoPackageManagerError { file_path } => {
            crate::println_error!(stdout, "No packageManager config error: {}", file_path)
        }
        SnmError::NotMatchPackageManager { expect, actual } => {
            crate::println_error!(
                stdout,
                "No matching package manager found. You input {} but the current project is configured to use {}.",
                expect.bright_green().bold(),
                actual.bright_red().bold()
            )
        }
        SnmError::MultiPackageManagerLockFileError { lock_file } => {
            crate::println_error!(
                stdout,
                "Multiple package manager lock files found: {} , Please remove the unnecessary ones.",
                lock_file.join(", ").bright_red()
            )
        }
    }
    std::process::exit(1);
}
