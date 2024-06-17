use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnmError {
    #[error("Get home dir error")]
    GetHomeDirError,

    #[error("Build config error: {0}")]
    BuildConfigError(#[from] config::ConfigError),

    #[error("Get workspace dir error")]
    GetWorkspaceError,

    #[error("Read file error: {0}")]
    ReadFileError(#[from] std::io::Error),

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
}
