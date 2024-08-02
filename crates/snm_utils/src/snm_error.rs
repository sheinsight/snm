use colored::*;
use std::{env, path::PathBuf, process::exit};
use thiserror::Error;

use crate::fmtln;

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

    #[error("Http status code not ok")]
    HttpStatusCodeUnOk,

    #[error("Get home dir error")]
    GetHomeDirError,

    #[error("Get workspace dir error")]
    GetWorkspaceError,

    #[error("No default node binary")]
    NoDefaultNodeBinary,

    #[error("File already exists {file_path}")]
    FileAlreadyExists { file_path: PathBuf },

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
        .join("\r\n".repeat(1).as_str());

    format!(
        r##"
{:<3}{}.

{:<3}{}

{}
"##,
        "👹", message, "📋", "Explain", description
    )
}

pub fn hack(error: SnmError) {
    let white_list = env::var("SNM_NODE_WHITE_LIST").unwrap();

    match error {
        SnmError::NoDefaultNodeBinary => {
            let message = format!(
                r##"
错误: 没有找到可执行的默认 Node

方案：
     如果你不想看下面繁琐的解释，只是想无脑的直接去解决问题，可以尝试直接复制以下命令在你项目的根目录下打开终端去执行。

     echo [这里写 node 版本号，注意不要携带上中括号] > .node-version

     例:   echo 20.11.1 > .node-version

     {}

解释：
     此类错误的直接原因是你当前执行命令的环境没有一个可以被执行的 Node.
     在 snm 的执行逻辑中, 通常会先检查你执行命令时所在的目录是否存在 .node-version 文件且文件内容为一个有效的 Node 版本号,
     如果存在则会使用该版本的 Node, 否则会尝试在全局去寻找一个默认的 Node 版本，如果也没有成功的找到则会抛出该错误。

注意:
     请注意 Node 的版本号需要写完整。 错误实例: 20, 正确实例: 20.11.1。
     中括号指的是  [ 或 ]
"##,
                if white_list.len() > 0 {
                    format!("目前支持的 node 版本如下 {}", white_list)
                } else {
                    "".to_string()
                }
            );
            eprintln!("{}", message);
        }
        SnmError::ExceededMaxRetries(url) => {
            let message = format!(
                r##"
错误: 超过最大重试次数

方案:
     请直接重试你的操作，如果还是失败，请检查你的网络连接和下载链接是否正常。或检查你的相关配置是否正确。
     很难给出明确的建议。
     相关配置指的是你的 Node 版本信息以及 包管理器的配置信息。
     Node 的版本号在你项目根目录的 .node-version 文件中配置，需要写完整。 错误实例: 20, 正确实例: 20.11.1。
     包管理器需要在 package.json 文件中配置 packageManager 字段，配置格式为 [包管理器的名称]@[包管理器的版本号]，不要带中括号

解释:
     snm 在获取 Node 与 包管理器的时候会进行下载操作，下载的链接是 {}

注意:
     中括号指的是  [ 或 ]
            "##,
                url
            );
            eprintln!("{}", message);
        }
        SnmError::ShasumError {
            file_path,
            expect,
            actual,
        } => {
            let message = format!(
                r##"
错误: 检查shasum错误

方案:
     建议你优先尝试重试你的操作，如果还是失败，可能下载文件受到损坏或者被篡改。

解释:
     snm 会针对下载下来的 Node 文件进行 shasum 校验，以确保下载的文件没有被篡改或者损坏。
     下载下来的文件路径是 {} , 预期的 shasum 值是 {} , 实际的 shasum 值是 {}

注意:
     请注意 Node 的版本号需要写完整。 错误实例: 20, 正确实例: 20.11.1。
     目前的版本包管理器没有进行 shasum 校验，只有 Node 的版本文件才会进行 shasum 校验。

            "##,
                file_path, expect, actual
            );
            eprintln!("{}", message);
        }
        SnmError::UnsupportedNodeVersionError {
            version,
            node_white_list,
        } => {
            let v_str = node_white_list.join(", ");
            let message = format!(
                r##"
错误: 你配置的 Node 版本不受支持

方案:
    你配置的 Node 版本是 {} , 我们只支持 {} 列表中的版本。

解释:
    当前环境针对 Node 限制了版本，你只能使用在被允许范围内的 Node 版本。

注意:
    请注意 Node 的版本号需要写完整。 错误实例: 20, 正确实例: 20.11.1。
    我们只支持 {} 列表中的版本。
            "##,
                version,
                node_white_list.join(", "),
                v_str
            );
            eprintln!("{}", message);
        }
        SnmError::NotFoundPackageJsonFileError {} => {
            let message = format!(
                r##"
错误: 没有找到 package.json 文件

方案:
    请检查当前目录下是否存在 package.json 文件，如果不存在请新建一个 package.json 文件。

解释:
    通常情况下，这类错误是由于你的项目根目录下没有 package.json 文件导致的。
    请检查当前目录下是否存在 package.json 文件，如果不存在请新建一个 package.json 文件。

注意:
    请注意 package.json 文件是一个必须的配置文件，用于描述你的项目的相关信息。
    请注意 package.json 文件的位置，必须在项目的根目录下。
    请注意 package.json 文件的内容，必须是一个合法的 JSON 格式文件。
"##,
            );
            eprintln!("{}", message);
        }
        SnmError::NotFondPackageManagerConfigError {} => {
            let message = format!(
                r##"
错误: 没有找到 packageManager 配置

方案:
    请检查 package.json 文件中是否存在 packageManager 字段，如果不存在请新建一个 packageManager 字段。

解释:
    通常情况下，这类错误是由于你的 package.json 文件中没有配置 packageManager 字段导致的。
    请检查 package.json 文件中是否存在 packageManager 字段，如果不存在请新建一个 packageManager 字段。

注意:
    请注意 packageManager 字段的配置内容，必须是 npm、yarn、pnpm 三者之一
    请注意 packageManager 字段的配置格式，必须符合 [包管理器的名称]@[包管理器的版本号] 的格式，不要带中括号
    请注意 packageManager 字段的配置内容，版本号必须写全，不支持简写版本号，例如: 9
    中括号指的是  [ 或 ]
"##,
            );
            eprintln!("{}", message);
        }
        SnmError::ParsePackageManagerError {
            raw_package_manager,
        } => {
            let message = format!(
                r##"
错误: 解析 packageManager 配置错误

方案:
    查询到你配置的 packageManager 的值是 {} 难以根据上下文准确的提供可尝试的一键式修复方案

解释:
    通常情况下，这类错误是由于 package.json 文件中的 packageManager 字段配置错误导致的。
    请检查 package.json 文件中的 packageManager 字段是否符合以下格式: [包管理器的名称]@[包管理器的版本号]，不要带中括号
    例如: npm@9.0.0

注意:
    请注意 packageManager 字段的配置格式，必须符合 [包管理器的名称]@[包管理器的版本号] 的格式，不要带中括号
    请注意 packageManager 字段的配置内容，必须是 npm、yarn、pnpm 三者之一
    请注意 packageManager 字段的配置内容，版本号必须写全，不支持简写版本号，例如: 9
    中括号指的是  [ 或 ]
"##,
                raw_package_manager
            );
            eprintln!("{}", message);
        }
        SnmError::NotMatchPackageManagerError {
            raw_command,
            expect,
            actual,
        } => {
            let message = format!(
                r##"
错误: 你执行的命令不符合 packageManager 配置

方案: 
    请使用你在 packageManager 字段配置的包管理器执行你的命令 
    你配置的内容是 {} , 预期的包管理器是 {} , 实际的包管理器是 {}


解释:
    snm 会识别出你的 packageManager 字段配置的包管理器，然后强制约定当你使用 install 、 i 、 run 这三个
    命令的时候会进行校验，如果你执行的命令不符合 packageManager 配置的包管理器，那么就会抛出这个错误。

注意:
    请注意 packageManager 字段的配置内容，必须是 npm、yarn、pnpm 三者之一
    请注意 packageManager 字段的配置内容，版本号必须写全，不支持简写版本号，错误示例: 9，正确示例: 9.0.0

            "##,
                raw_command, expect, actual
            );
            eprintln!("{}", message);
        }
        SnmError::UnsupportedPackageManagerError {
            raw,
            name,
            supported,
        } => {
            let message = format!(
                r##"
错误: packageManager 配置的包管理器不支持

方案:
    查询到你配置的 packageManager 的值是 {} , {} 可能是一个不受支持的包管理器或这是一个不存在的包管理器。

解释:
    当前 snm 默认只支持 npm、yarn、pnpm 三种包管理器，如果你的配置不是这三种包管理器之一，那么就会抛出这个错误。
    你配置的包管理器是 {}

注意:
    请注意 packageManager 字段的配置内容，必须是 npm、yarn、pnpm 三者之一
            "##,
                raw, name, name
            );
            eprintln!("{}", message);
        }

        SnmError::NotFoundCommandError { bin_name } => {
            let message = format!(
                r##"
错误: 没有找到命令 {}

方案:
    请检查你输入的命令是否正确，如果正确请检查你的环境变量是否配置正确。

解释:
    当前环境没有找到你输入的命令，这可能是因为你输入的命令不正确或者你的环境变量没有配置正确。

注意:
    请注意你输入的命令是否正确，如果正确请检查你的环境变量是否配置正确。
            "##,
                bin_name
            );
            eprintln!("{}", message);
        }

        SnmError::SNMBinaryProxyFail { stderr: _ } => {
            let message = format!(
                r##"
错误: snm 二进制代理失败

方案:
    阅读错误日志，查看其他错误

解释:
    通常这是由其他错误引起的，并不是直接性的错误原因，你可以查看错误日志，查看其他错误。

注意:
    无
            "##,
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
        | SnmError::IOError(_)
        | SnmError::GetHomeDirError
        | SnmError::FileAlreadyExists { file_path: _ } => {
            //             let msg = format!(
            //                 r##"
            // 错误:这不是一个预期内的错误

            // 方案:
            //     无

            // 解释:
            //     无

            // 注意:
            //     无
            //             "##,
            //             );
            eprintln!("error {}", error.to_string());
        }
    }
}

pub fn friendly_error_message(error: SnmError) {
    if let Ok(lang) = env::var("SNM_LANG") {
        if lang == "cn_zh" {
            hack(error);
            exit(1);
        }
    }

    match error {
        SnmError::SNMBinaryProxyFail { stderr: _ } => {
            // TODO 🤔 how to show ?
        }
        SnmError::NoDefaultNodeBinary => {
            let message = create_error_message(
                format!("No executable default Node found"),
                vec![
                    fmtln!(
                        "Please use {} set default node",
                        "snm node default [node version]".bold().bright_green()
                    ),
                    fmtln!(
                        "Or use {}",
                        "echo [node version] > .node-version".bold().bright_green()
                    ),
                ],
            );
            eprintln!("{}", message);
        }
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
                vec![
                    fmtln!("URL {}", url.to_string().bold().red()),
                    fmtln!("The download failed after 3 retries.",),
                    fmtln!("Please check the network connection and the download URL",),
                ],
            );
            eprintln!("{}", message);
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
        SnmError::FileAlreadyExists { file_path } => {
            let message = create_error_message(
                "File already exists".to_string(),
                vec![format!(
                    "The file {} already exists.",
                    file_path.to_string_lossy().bold().red()
                )],
            );
            eprintln!("{}", message);
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
            let list_message = node_white_list
                .iter()
                .map(|item| format!("- {}", item).to_string())
                .collect::<Vec<String>>();

            let message = create_error_message(
                format!("不支持 {}", version.bold().bright_red()),
                vec![
                    vec![fmtln!("{}", "只支持以下列表:")],
                    list_message,
                    vec!["\r\n".to_string()],
                    vec![
                        "🤔 如何设置当前项目的 node 版本".to_string(),
                        "请先检查项目根目录是否存在 .node-version 文件".to_string(),
                        "如果不存在，请新建 .node-version 文件".to_string(),
                        "在 .node-version 文件中写入你的 node 版本".to_string(),
                        "请注意不支持简写版本号，例: 14 。务必保证版本号写全".to_string(),
                        "务必保证版本号写全，例: 14.17.0。".to_string(),
                        "请注意以上提示种所说的 14 以及 14.17.0 仅仅只是举例".to_string(),
                        "你需要保证你写的版本号在支持的列表内。".to_string(),
                    ],
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
                        fmtln!("The raw package manager configuration is {}, Only the following list is supported:", raw.bold().bright_red()),
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
