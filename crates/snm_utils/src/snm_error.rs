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

    #[error("{stderr}")]
    SNMBinaryProxyFail { stderr: String },

    #[error("Shasum error: {file_path} , expect: {expect} , actual: {actual}")]
    ShasumError {
        file_path: String,
        expect: String,
        actual: String,
    },

    #[error("Unsupported node version: {version}")]
    UnsupportedNodeVersionError {
        version: String,
        node_white_list: Vec<String>,
    },

    #[error("Not found command: {bin_name}")]
    NotFoundCommandError { bin_name: String },

    #[error("Not found package.json file")]
    NotFoundPackageJsonFileError {},

    #[error("Not found package manager config")]
    NotFondPackageManagerConfigError {},

    #[error("{raw_package_manager}")]
    ParsePackageManagerError { raw_package_manager: String },

    #[error("Package manager version not match, expected: {expect}, actual: {actual}")]
    NotMatchPackageManagerError {
        raw_command: String,
        expect: String,
        actual: String,
    },

    #[error("Unsupported package manager: {name}")]
    UnsupportedPackageManagerError {
        raw: String,
        name: String,
        supported: Vec<String>,
    },
}

pub fn create_error_message(message: String, descriptions: Vec<String>) -> String {
    let description = descriptions
        .iter()
        .map(|value| format!("{:<4}{}", "", value))
        .collect::<Vec<String>>()
        .join("\r\n".repeat(2).as_str());

    format!(
        r##"
{:<3}{}.

{:<3}{}

{}
    "##,
        "👹", message, "📋", "Explain", description
    )
}

pub fn friendly_error_message(error: SnmError) {
    match error {
        SnmError::ParsePackageManagerError {
            raw_package_manager,
        } => {
            let message = create_error_message(
                "Parse package manager error".to_string(),
                vec![
                    format!(
                        "Please check the raw package manager configuration: {}",
                        raw_package_manager.bold().red()
                    ),
                    format!(
                        "Should satisfy {}, Example: {}",
                        "[packageManager]@[version]".bold().green(),
                        "npm@9.0.0".bold().green()
                    ),
                ],
            );
            eprintln!("{}", message);
        }
        SnmError::ExceededMaxRetries(url) => {
            let message = create_error_message(
                "Exceeded max retries".to_string(),
                vec![format!(
                    "The download failed after {} retries.",
                    url.to_string().bold().red()
                )],
            );
            eprintln!("{}", message);
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
        SnmError::NotFoundCommandError { bin_name } => {
            let message = create_error_message(
                format!("Not found command {}", bin_name.bold().red()),
                vec![format!(
                    "The command {} is not found in the current environment.",
                    bin_name.bold().red()
                )],
            );
            eprintln!("{}", message);
        }
        SnmError::NotMatchPackageManagerError {
            raw_command,
            expect,
            actual,
        } => {
            let message = create_error_message(
                "Not match package manager".to_string(),
                vec![
                    format!("You input: {}", raw_command.bright_black()),
                    format!("Expect {}", expect.green()),
                    format!("Actual {}", actual.red()),
                ],
            );
            eprintln!("{}", message);
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
            let message = create_error_message(
                "Check shasum error".to_string(),
                vec![
                    format!("File path {}", file_path.bright_black()),
                    format!("Expect {}", expect.bold().green()),
                    format!("Actual {}", actual.bold().red()),
                ],
            );
            eprintln!("{}", message);
        }
        SnmError::NotFoundPackageJsonFileError {} => {
            let message = create_error_message(
                "Not found package.json file".to_string(),
                vec![format!(
                    "Please check the current directory, whether the package.json file exists."
                )],
            );
            eprintln!("{}", message);
        }
        SnmError::NotFondPackageManagerConfigError {} => {
            let message = create_error_message(
                "Not found packageManager config".to_string(),
                vec![format!(
                    "Please check the package.json file, whether the packageManager field exists."
                )],
            );
            eprintln!("{}", message);
        }
        SnmError::UnsupportedNodeVersionError {
            version,
            node_white_list,
        } => {
            let message = create_error_message(
                format!("Unsupported node {}", version.bold().bright_red()),
                vec![
                    vec!["Only the following list is supported:".to_string()],
                    node_white_list
                        .iter()
                        .map(|item| format!("- {}", item).to_string())
                        .collect::<Vec<String>>(),
                ]
                .concat(),
            );
            eprintln!("{}", message);
        }
        SnmError::UnsupportedPackageManagerError {
            raw,
            name,
            supported,
        } => {
            let message = create_error_message(
                format!("Unsupported packageManager {}", name.bold().bright_red()),
                vec![
                    vec![
                        format!(
                            "The raw package manager configuration is {}, Only the following list is supported:",
                            raw.bold().bright_red()
                        ),
                    ],
                    supported
                        .iter()
                        .map(|item| format!("- {}", item).to_string())
                        .collect::<Vec<String>>(),
                ]
                .concat(),
            );
            eprintln!("{}", message);
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
