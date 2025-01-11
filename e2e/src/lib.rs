pub mod exec_builder;
pub mod http_mocker;
use duct::cmd;
use std::path::PathBuf;
use tempfile::tempdir;
use textwrap::dedent;

#[derive(Debug, Clone)]
pub enum SnmEnv<T: AsRef<str> = String> {
    HomeDir(T),
    Strict(T),
    NodeBinDir(T),
    DownloadDir(T),
    Lang(T),
    NodeModulesDir(T),
    CacheDir(T),
    NodeDistUrl(T),
    NpmRegistry(T),
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
            Self::NpmRegistry(v) => ("SNM_NPM_REGISTRY".to_string(), v.as_ref().to_string()),
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

pub struct Args<'a, const N: usize> {
    pub cwd: [&'a str; N],
    pub env: Vec<SnmEnv>,
}

pub struct CommandBuilder {
    name: String,
    envs: Vec<(String, String)>,
    cwd: PathBuf,
    counter: usize,
    snapshots: Vec<String>,
}

impl CommandBuilder {
    pub fn with_envs(name: &str, cwd: PathBuf, custom_envs: Vec<SnmEnv>) -> anyhow::Result<Self> {
        let tmp_dir = tempdir()?.into_path();
        let env_path = env!("PATH");
        let debug_dir = Self::get_debug_dir().to_str().unwrap().to_string();
        let mut envs: Vec<SnmEnv> = vec![
            SnmEnv::Path(format!("{}:{}", debug_dir, env_path)),
            SnmEnv::HomeDir(tmp_dir.display().to_string()),
        ];
        envs.extend(custom_envs);
        Ok(Self {
            name: name.to_string(),
            envs: envs.into_iter().map(|e| e.as_tuple()).collect::<Vec<_>>(),
            cwd: cwd,
            counter: 0,
            snapshots: vec![],
        })
    }

    pub fn get_debug_dir() -> PathBuf {
        // 获取 e2e 目录 (CARGO_MANIFEST_DIR 指向 e2e/Cargo.toml 所在目录)
        let e2e_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // 向上一级找到项目根目录
        let root_dir = e2e_dir.parent().expect("Failed to get root dir");

        // 进入 target/debug 目录
        root_dir.join("target").join("debug")
    }

    pub fn exec(&self, command: &str) -> anyhow::Result<String> {
        let expr = if cfg!(target_os = "windows") {
            cmd!("cmd", "/C", command)
        } else {
            cmd!("sh", "-c", command)
        };

        let output = expr
            .full_env(self.envs.clone())
            // .env(envs) // 设置环境变量
            .dir(self.cwd.clone()) // 设置工作目录
            .stdout_capture()
            .stderr_capture() // 同时捕获输出
            .unchecked()
            .run()?;

        if !output.status.success() {
            return Ok(String::from_utf8(output.stderr.clone())?.trim().to_string());
        }

        Ok(String::from_utf8(output.stdout.clone())?.trim().to_string())
    }

    pub fn add_snapshot(&mut self, command: &str) -> anyhow::Result<&mut Self> {
        self.counter += 1;
        // let current_dir = std::env::current_dir()?
        //     .join("tests")
        //     .join("snapshots")
        //     .join(self.name.clone());
        let res = self.exec(command)?;
        let res = dedent(&format!(
            r#"
id: {}
is: {}

{}"#,
            self.counter, command, res
        ));
        // let name = format!("{}_{}", self.name, self.counter);
        self.snapshots.push(res);
        Ok(self)
    }

    pub fn assert_snapshots<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: Fn(&str, &str),
    {
        let res = self.snapshots.join("\n");

        f(&self.name, &res);

        Ok(())
    }

    pub fn snapshot<F>(&mut self, command: &str, f: F) -> anyhow::Result<&mut Self>
    where
        F: Fn(&PathBuf, &str, &str),
    {
        self.counter += 1;
        let current_dir = std::env::current_dir()?
            .join("tests")
            .join("snapshots")
            .join(self.name.clone());
        let res = self.exec(command)?;
        let res = dedent(&format!(
            r#"
id: {}
is: {}

{}"#,
            self.counter, command, res
        ));
        let name = format!("{}_{}", self.name, self.counter);
        f(&current_dir, &name, &res);
        // insta::with_settings!({
        //     snapshot_path => current_dir,  // 指定快照目录
        // }, {
        //     insta::assert_snapshot!(
        //         format!("{}_{}", self.name, self.counter),
        //         dedent(&format!(
        //             r#"
        //                 is: {}
        //                 os: {}
        //                 "#,
        //             command,
        //             res
        //         ))
        //     );
        // });

        Ok(self)
    }
}

#[macro_export]
macro_rules! test1 {
    (
        $(#[$attr:meta])*
        $test_name:ident,
        cwd: $cwd:expr,
        // node:[$($node:expr),* $(,)?],
        // npm:[$($npm:expr),* $(,)?],
        // pnpm:[$($pnpm:expr),* $(,)?],
        // yarn:[$($yarn:expr),* $(,)?],
        envs:[$($env:expr),* $(,)?],
        |$builder:ident:$handler_type:ty| => $body:block
    ) => {
        $(#[$attr])*
        async fn $test_name() -> anyhow::Result<()> {
            let mock_server = e2e::http_mocker::HttpMocker::builder()
                // $(.with_node(vec![$node]))*
                // $(.with_npm(vec![$npm]))*
                // $(.with_pnpm(vec![$pnpm]))*
                // $(.with_yarn(vec![$yarn]))*
                .build()
                .await?;

            let mut $builder = e2e::CommandBuilder::with_envs(
                stringify!($test_name),
                $cwd,
                vec![
                    e2e::SnmEnv::NodeDistUrl(mock_server.uri()),
                    e2e::SnmEnv::NpmRegistry(mock_server.uri()),
                    $($env,)*
                ]
            )?;

            // let $snapshot = e2e::SnapshotBuilder::new();

            $body

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! assert_snapshot {
    (
        $(#[$attr:meta])*
        $test_name:ident,
        cwd: $cwd:expr,
        envs: $envs:expr,
        commands: [
            $($cmd:expr),* $(,)?
        ]
    ) => {
        $(#[$attr])*
        async fn $test_name() -> anyhow::Result<()> {
            let cwd = $cwd;
            e2e::CommandBuilder::with_envs(
                stringify!($test_name),
                cwd,
                $envs,
            )?
            $(
                .snapshot($cmd, |snapshot_path, name, res| {
                    insta::with_settings!({
                        snapshot_path => snapshot_path,  // 指定快照目录
                    }, {
                        insta::assert_snapshot!(
                            name,
                            res
                        );
                    })
                })?
            )*;

            Ok(())
        }
    };
}
