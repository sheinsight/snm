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

    #[error("Duplicate lock file error")]
    DuplicateLockFileError { lock_file_vec: Vec<String> },

    #[error("{stderr}")]
    SNMBinaryProxyFail { stderr: String },

    #[error("Shasum error: {file_path} , expect: {expect} , actual: {actual}")]
    ShasumError {
        file_path: String,
        expect: String,
        actual: String,
    },

    #[error("Unsupported node {actual} ")]
    UnsupportedNodeVersionError { actual: String, expect: Vec<String> },
}

pub fn friendly_error_message(error: SnmError) {
    match error {
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
                r##"You input: {} , packageManager not match. The expected packageManager is {} , but the actual packageManager is {}."##,
                raw_command.bold().red(),
                expected.green(),
                actual.red()
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
        SnmError::UnsupportedNodeVersionError { actual, expect } => {
            eprintln!(
                r##"👹 Unsupported node {} , Only the following list is supported:

{}"##,
                actual.bold().red(),
                expect
                    .iter()
                    .map(|item| format!("- {}", item).to_string())
                    .collect::<Vec<String>>()
                    .join("\r\n")
            );
        }
        SnmError::HttpStatusCodeUnOk
        | SnmError::GetWorkspaceError
        | SnmError::DeserializeError(_)
        | SnmError::NetworkError(_)
        | SnmError::DialoguerError(_)
        | SnmError::VarError(_)
        | SnmError::ZipError(_)
        | SnmError::BuildConfigError(_)
        | SnmError::IOError(_) => {
            let msg = format!("{}", error.to_string());
            // panic!("{msg}");
            eprintln!("[error]: {}", msg);
        }
    }

    exit(1);
}
