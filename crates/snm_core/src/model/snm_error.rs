use colored::*;
use std::{env::VarError, path::StripPrefixError};
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
    NotFoundPackageJsonBinProperty { file_path: String },

    #[error("Not found package.json file here {package_json_file_path}")]
    NotFoundPackageJsonFileError { package_json_file_path: String },

    #[error("Download failed , The URL {download_url}")]
    DownloadFailed { download_url: String },

    #[error("File already exists {file_path}")]
    FileAlreadyExists { file_path: String },

    #[error("File {file_path} not exist")]
    FileNotExist { file_path: String },

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

    #[error("Not found default npm binary")]
    NotFoundDefaultNpmBinary,

    #[error("Not found default package manager {name}")]
    NotFoundDefaultPackageManager { name: String },

    #[error("Not found .node-version file")]
    NotFoundNodeVersionFileError { file_path: String },

    #[error("Unsupported platform {os} {arch}")]
    UnsupportedPlatform { os: String, arch: String },

    #[error("Unknown install strategy")]
    UnknownInstallStrategy,

    #[error("UnSupportNodeVersion {version}")]
    UnsupportedNodeVersion { version: String },

    #[error("Unsupported {name}@{version}")]
    UnsupportedPackageManager { name: String, version: String },

    #[error("Not found valid package manager {name}")]
    EmptyPackageManagerList { name: String },

    #[error("Not found package manager {name}@{version}")]
    NotFoundPackageManager { name: String, version: String },
}

impl From<serde_json::Error> for SnmError {
    fn from(_error: serde_json::Error) -> Self {
        SnmError::UnknownError
    }
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

// impl From<StripPrefixError> for SnmError {
//     fn from(_error: StripPrefixError) -> Self {
//         SnmError::UnknownError
//     }
// }

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
    match error {
        SnmError::CreateDirFailed { dir_path } => {
            crate::println_error!("Create dir failed {}", dir_path)
        }
        SnmError::SerdeJsonError { file_path } => {
            crate::println_error!("Parse json error {}", file_path)
        }
        SnmError::ReadFileToStringError { file_path } => {
            crate::println_error!("Read file to string error {}", file_path)
        }
        SnmError::PackageJsonBinPropertyUnknownTypeError { file_path } => {
            crate::println_error!(
                "Package.json bin property unknown type error , The absolute path {}",
                file_path
            )
        }
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
        SnmError::UnknownError => {
            crate::println_error!("Unknown Error")
        }
        SnmError::EmptyNodeList => {
            crate::println_error!(
                "Node list is empty, please use {} to get the latest version.",
                "snm node list-remote".bright_green().bold()
            )
        }

        SnmError::NotFoundSha256ForNode(_) => {
            crate::println_error!("NotFoundSha256ForNode")
        }
        SnmError::RefuseToInstallNode => todo!("RefuseToInstallNode"),
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
        SnmError::ReadDirFailed { dir_path } => {
            crate::println_error!("Read dir failed {}", dir_path)
        }
        SnmError::GetHomeDirError => {
            crate::println_error!("Unable to retrieve user directory correctly ! ")
        }
        SnmError::NotFoundBinDirConfig => {
            crate::println_error!(
                "Unable to get the {} environment variable.",
                "SNM_NODE_BIN_DIR".bright_red()
            )
        }
        SnmError::NotFoundDownloadDirConfig => {
            crate::println_error!(
                "Unable to get the {} environment variable.",
                "SNM_DOWNLOAD_DIR".bright_red()
            )
        }
        SnmError::NotFoundNodeModulesDirConfig => {
            crate::println_error!(
                "Unable to get the {} environment variable.",
                "SNM_NODE_MODULES_DIR".bright_red()
            )
        }
        SnmError::NotFoundDefaultNodeBinary => {
            crate::println_error!(
                "No Node.js default detected. Set it with {} or specify it in `.node-version` file here.","snm node default [version]".bright_green().bold()
            )
        }
        SnmError::ParsePackageManagerConfigError { raw_value } => {
            crate::println_error!("Parse package manager config error: {}", raw_value)
        }
        SnmError::NoPackageManagerError { file_path } => {
            crate::println_error!("No packageManager config error: {}", file_path)
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
        SnmError::NotFoundDefaultNpmBinary => {
            crate::println_error!(
                "No npm default detected. Please configure package.json -> packageManager or use {} to set the default version.",
                "snm npm default [version]".bright_green().bold()
            )
        }
        SnmError::NotFoundPackageJsonFileError {
            package_json_file_path,
        } => {
            crate::println_error!(
                "{} not found. Please create one and configure the 'packageManager' field.",
                package_json_file_path
            )
        }
        SnmError::NotFoundNodeVersionFileError { file_path } => {
            crate::println_error!(
                "{} not found. Please create .node-version file and write node version into.",
                file_path
            )
        }
        SnmError::FileNotExist { file_path } => {
            crate::println_error!("File {} not exist", file_path)
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
        SnmError::EmptyPackageManagerList { name: _ } => {
            crate::println_error!(
                "No package manager found. Please use {} to get the latest version.",
                "snm npm list-remote".bright_green().bold()
            )
        }
        SnmError::NotFoundDefaultPackageManager { name } => {
            crate::println_error!(
                "No {} default detected. Please configure package.json -> packageManager or use {} to set the default version.",
                name.bright_green().bold(),
                format!("snm {} default [version]", name).bright_green().bold()
            )
        }
        SnmError::NotFoundPackageManager { name, version } => {
            crate::println_error!(
                "No {}@{} found. Please use {} to get the latest version.",
                name.bright_green().bold(),
                version.bright_green().bold(),
                format!("snm {} list-remote", name).bright_green().bold()
            )
        }
    }
    std::process::exit(1);
}
