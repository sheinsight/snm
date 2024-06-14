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
}
