pub mod exec_builder;

use std::{
    env::current_dir,
    path::PathBuf,
    process::{Command, Stdio},
};

use tempfile::tempdir;

fn get_debug_dir() -> PathBuf {
    // 获取 e2e 目录 (CARGO_MANIFEST_DIR 指向 e2e/Cargo.toml 所在目录)
    let e2e_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // 向上一级找到项目根目录
    let root_dir = e2e_dir.parent().expect("Failed to get root dir");

    // 进入 target/debug 目录
    root_dir.join("target").join("debug")
}

#[derive(Debug)]
pub enum SnmEnv<T: AsRef<str> = String> {
    HomeDir(T),
    Strict(T),
    NodeBinDir(T),
    DownloadDir(T),
    Lang(T),
    NodeModulesDir(T),
    CacheDir(T),
    NodeDistUrl(T),
    DownloadTimeoutSecs(T),
    NodeGithubResourceHost(T),
    NodeInstallStrategy(T),
    NodeWhiteList(T),
    Path(T),
}

impl<T: AsRef<str>> SnmEnv<T> {
    pub fn as_tuple(&self) -> (String, String) {
        match self {
            Self::HomeDir(v) => ("SNM_HOME_DIR".to_string(), v.as_ref().to_string()),
            Self::Strict(v) => ("SNM_STRICT".to_string(), v.as_ref().to_string()),
            Self::NodeBinDir(v) => ("SNM_NODE_BIN_DIR".to_string(), v.as_ref().to_string()),
            Self::DownloadDir(v) => ("SNM_DOWNLOAD_DIR".to_string(), v.as_ref().to_string()),
            Self::Lang(v) => ("SNM_LANG".to_string(), v.as_ref().to_string()),
            Self::NodeModulesDir(v) => ("SNM_NODE_MODULES_DIR".to_string(), v.as_ref().to_string()),
            Self::CacheDir(v) => ("SNM_CACHE_DIR".to_string(), v.as_ref().to_string()),
            Self::NodeDistUrl(v) => ("SNM_NODE_DIST_URL".to_string(), v.as_ref().to_string()),
            Self::DownloadTimeoutSecs(v) => (
                "SNM_DOWNLOAD_TIMEOUT_SECS".to_string(),
                v.as_ref().to_string(),
            ),
            Self::NodeGithubResourceHost(v) => (
                "SNM_NODE_GITHUB_RESOURCE_HOST".to_string(),
                v.as_ref().to_string(),
            ),
            Self::NodeInstallStrategy(v) => (
                "SNM_NODE_INSTALL_STRATEGY".to_string(),
                v.as_ref().to_string(),
            ),
            Self::NodeWhiteList(v) => ("SNM_NODE_WHITE_LIST".to_string(), v.as_ref().to_string()),
            Self::Path(v) => ("PATH".to_string(), v.as_ref().to_string()),
        }
    }
}

#[macro_export]
macro_rules! exec {
  (cwd: [$($path:expr),+], command: $command:expr, $(env: |$param:ident| => $block:expr,)?) => {{
        let cwd = current_dir()?
            .join("tests")
            .join("fixtures")
            $(.join($path))*;

        let tmp_dir = tempdir()?.into_path();

        let debug_dir = get_debug_dir();

        let mut envs:Vec<SnmEnv> = vec![
          SnmEnv::HomeDir(tmp_dir.display().to_string()),
          SnmEnv::Path(debug_dir.to_str().unwrap().to_string()),
        ];

        $(
            let custom_envs: Vec<SnmEnv> = (|$param: PathBuf| $block)(tmp_dir.clone());
            envs.extend(custom_envs);
        )?

        let envs: Vec<_> = envs.into_iter().map(|e| e.as_tuple()).collect::<Vec<_>>();

        let output = Command::new("node")
            .args(["-v"])
            .envs(envs)
            .current_dir(cwd)
            .output()?; // 直接执行命令

        if !output.status.success() {
            // 如果命令执行失败，返回错误信息
            anyhow::bail!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        String::from_utf8(output.stdout)?.trim().to_string()  // 添加 trim()
    }};
}
