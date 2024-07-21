use colored::*;
use std::{path::PathBuf, process::exit};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnmError {
    #[error("Build config error: {0}")]
    BuildConfigError(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Dialoguer error: {0}")]
    DialoguerError(#[from] dialoguer::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Get var error :{0}")]
    VarError(#[from] std::env::VarError),

    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFoundResourceError(String),

    #[error("Not found package.json {0}")]
    NotFoundPackageJsonError(PathBuf),

    #[error("Package manager version not match, expected: {expected}, actual: {actual}")]
    NotMatchPackageManagerError {
        raw_command: String,
        expected: String,
        actual: String,
    },

    #[error("Http status code not ok")]
    HttpStatusCodeUnOk,

    #[error("Get home dir error")]
    GetHomeDirError,

    #[error("Get workspace dir error")]
    GetWorkspaceError,

    #[error("Not found valid version")]
    NotFoundValidNodeVersionDeclaration,

    #[error("No default node binary")]
    NoDefaultNodeBinary,

    #[error("File already exists {0}")]
    FileAlreadyExists(PathBuf),

    #[error("Exceeded maximum retry attempts: {0}")]
    ExceededMaxRetries(String),

    #[error("Parse package manager error , raw is {0}")]
    ParsePackageManagerError(String),

    #[error("Unsupported command: {raw_command}")]
    UnsupportedCommandError { raw_command: String },

    #[error("Duplicate lock file error")]
    DuplicateLockFileError { lock_file_vec: Vec<String> },

    #[error("{stderr}")]
    SNMBinaryProxyFail { stderr: String },

    #[error("Cannot find command: {command}")]
    CannotFindDefaultCommand { command: String },

    #[error("Shasum error: {file_path} , expect: {expect} , actual: {actual}")]
    ShasumError {
        file_path: String,
        expect: String,
        actual: String,
    },

    #[error("Unsupported node v{version} ")]
    UnsupportedNodeVersionError { version: String },
}

pub fn friendly_error_message(error: SnmError) {
    match error {
        SnmError::BuildConfigError(_) => {
            eprintln!(
                r##"
        👹  Build snm config error

            The following is a list of configurations supported by snm:

            SNM_STRICT: 

                Whether to enable strict mode, default is false.
                In strict mode, 
                Must be a .node-version file in current_dir and the correct version number configured.
                Must be a package.json in current_dir with the correct configuration of packageManager, for example: npm@8.0.0

            SNM_NODE_BIN_DIR: 

                The directory where the node binary is stored, default is node_bin.

            SNM_DOWNLOAD_DIR: 

                The directory where the downloaded file is stored, default is downloads.

            SNM_NODE_MODULES_DIR: 

                The directory where the node_modules is stored, default is node_modules.

            SNM_NODE_DIST_URL: 

                The download address of the node binary, the default is https://nodejs.org/dist .

            SNM_GITHUB_RESOURCE_HOST: 

                The download address of the node binary, the default is https://raw.githubusercontent.com .

            SNM_NODE_INSTALL_STRATEGY: 

                The installation strategy of the node binary, the default is auto. You can choose ask, panic, auto.

            SNM_DOWNLOAD_TIMEOUT_SECS: 

                The download timeout time, the default is 60s.

            SNM_PACKAGE_MANAGER_INSTALL_STRATEGY: 

                The installation strategy of the package manager, the default is auto. You can choose ask, panic, auto.

            "##
            );
        }
        SnmError::ParsePackageManagerError(raw) => {
            eprintln!(
                r##"
        👹  Parse packageManager Error

            The packageManager {} configured in your package.json is not being parsed correctly. 
            "##,
                raw.bold().red()
            );
        }
        SnmError::ExceededMaxRetries(url) => {
            eprintln!(
                r##"
        👹  Exceeded max retries

            The download failed after {} retries. 
            "##,
                url.to_string().bold().red()
            );
        }
        SnmError::NotFoundResourceError(url) => {
            eprintln!(
                r##"
        👹  Not found resource

            The resource {} was not found. 
            "##,
                url.to_string().bold().red()
            );
        }
        SnmError::GetHomeDirError => {
            eprintln!(
                r##"
        👹  Get home dir failed

            I think the possible reasons are:

            · The HOME environment variable is not set.
            · The HOME environment variable is not set correctly.
            · The HOME environment variable is not set to a directory.

            Platform	    Value	                Example
            Linux	        $HOME	                /home/alice
            macOS	        $HOME	                /Users/Alice
            Windows	        FOLDERID_Profile	C:\Users\Alice

            Linux and macOS:
            Use $HOME if it is set and not empty.
            If $HOME is not set or empty, then the function getpwuid_r is used to determine the home directory of the current user.
            If getpwuid_r lacks an entry for the current user id or the home directory field is empty, then the function returns None.
            Windows:
            This function retrieves the user profile folder using SHGetKnownFolderPath.

            All the examples on this page mentioning $HOME use this behavior.

            Note: This function's behavior differs from std::env::home_dir, which works incorrectly on Linux, macOS and Windows.
            "##
            );
        }
        SnmError::FileAlreadyExists(path_buf) => {
            eprintln!(
                r##"
        👹  File already exists

            The file {} already exists. 
            "##,
                path_buf.to_string_lossy().bold().red()
            );
        }
        SnmError::NotFoundValidNodeVersionDeclaration => {
            eprintln!(r##"[error]: Not found valid node version declaration"##);
        }
        SnmError::NoDefaultNodeBinary => {
            eprintln!(r##"[error]: No default node binary"##);
        }
        SnmError::NotMatchPackageManagerError {
            raw_command,
            expected,
            actual,
        } => {
            eprintln!(
                r##"
        👹  You Input: {} , Package Manager not match

            The expected packageManager is {} , but the actual packageManager is {}.
            "##,
                raw_command.bold().red(),
                expected.green(),
                actual.red()
            );
        }
        SnmError::UnsupportedCommandError { raw_command } => {
            eprintln!(
                r##"
        👹  You exec command is unsupported

            {}
                "##,
                raw_command
            );
        }
        SnmError::DuplicateLockFileError { lock_file_vec } => {
            eprintln!(
                r##"
        👹  Duplicate lock file error
            
            Multiple package manager lock files found: {} , Please remove the unnecessary ones.
            "##,
                lock_file_vec.join(", ").bold().red()
            );
        }
        SnmError::SNMBinaryProxyFail { stderr } => {
            eprintln!(
                r##"
        👹  SNM proxy error info:

            {}
            "##,
                stderr
            )
        }
        SnmError::ShasumError {
            file_path,
            expect,
            actual,
        } => {
            eprintln!(
                r##"
        👹  Shasum error
            
            {} 

            expect  {} , 

            actual  {}.

            Please try again
                "##,
                file_path,
                expect.green(),
                actual.red(),
            );
        }
        SnmError::HttpStatusCodeUnOk
        | SnmError::NotFoundPackageJsonError(_)
        | SnmError::GetWorkspaceError
        | SnmError::DeserializeError(_)
        | SnmError::NetworkError(_)
        | SnmError::DialoguerError(_)
        | SnmError::VarError(_)
        | SnmError::UnsupportedNodeVersionError { version: _ }
        | SnmError::CannotFindDefaultCommand { command: _ }
        | SnmError::ZipError(_)
        | SnmError::IOError(_) => {
            let msg = format!("{}", error.to_string());
            // panic!("{msg}");
            eprintln!("[error]: {}", msg);
        }
    }

    exit(1);
}
