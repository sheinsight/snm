use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnmError {
    #[error("Build config error: {0}")]
    BuildConfigError(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Not found: {0}")]
    NotFoundResourceError(String),

    #[error("Http status code not ok")]
    HttpStatusCodeUnOk,

    #[error("Get home dir error")]
    GetHomeDirError,

    #[error("Get workspace dir error")]
    GetWorkspaceError,

    #[error("Not found node version file")]
    NotFoundNodeVersionConfigFile,

    #[error("Not found valid node version")]
    NotFoundValidNodeVersion,

    #[error(
        "Not found default node version , please use `snm node use` to set default node version"
    )]
    NotFoundDefaultNodeVersion,

    #[error("Not found package manager")]
    NotFoundPackageManager,

    #[error("File already exists {0}")]
    FileAlreadyExists(PathBuf),

    #[error("Exceeded maximum retry attempts: {0}")]
    ExceededMaxRetries(String),
}
