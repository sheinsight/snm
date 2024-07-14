use crate::conditional_compiler::get_arch;
use crate::conditional_compiler::get_os;
use crate::conditional_compiler::get_tarball_ext;
use crate::node_model::Lts;
use crate::node_model::NodeModel;
use crate::node_schedule::NodeSchedule;
use chrono::NaiveDate;
use chrono::Utc;
use colored::*;
use dialoguer::Confirm;
use futures::*;
use semver::Version;
use semver::VersionReq;
use sha2::Digest;
use sha2::Sha256;
use snm_config::InstallStrategy;
use snm_config::SnmConfig;
use snm_core::traits::atom::AtomTrait;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_tarball::decompress;
use snm_utils::snm_error::SnmError;
use snm_utils::to_ok::ToOk;
use std::collections::HashMap;
use std::fs;
use std::ops::Not;
use std::pin::Pin;
use std::time::Duration;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use tokio::try_join;

pub struct SnmNode {
    snm_config: SnmConfig,
}

impl SnmNode {
    async fn download(&self, version: &str) -> Result<(), SnmError> {
        let download_url = self.get_download_url(version);
        let downloaded_file_path_buf = self.get_downloaded_file_path_buf(version)?;

        DownloadBuilder::new()
            .retries(3)
            .write_strategy(WriteStrategy::Nothing)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        let runtime = self.get_runtime_dir_path_buf(version)?;

        if runtime.exists() {
            fs::remove_dir_all(&runtime)?;
        }

        self.decompress_download_file(&downloaded_file_path_buf, &runtime)?;

        fs::remove_file(&downloaded_file_path_buf)?;

        Ok(())
    }

    pub async fn set_default(&self, version: &str) -> Result<(), SnmError> {
        if self.get_anchor_file_path_buf(version)?.exists().not() {
            let msg = format!(
                "🤔 v{} is not installed, do you want to install it ?",
                version
            );
            if Confirm::new().with_prompt(msg).interact()? {
                self.install(version).await?;
            }
        } else {
            let default_dir = self.get_runtime_dir_for_default_path_buf()?;
            if default_dir.exists() {
                fs::remove_dir_all(&default_dir)?;
            }

            let from_dir = self.get_runtime_dir_path_buf(version)?;

            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&from_dir, &default_dir)?;
            }
            #[cfg(windows)]
            {
                std::os::windows::fs::symlink_dir(&version_dir, &default_dir)?;
            }
        }

        Ok(())
    }

    pub async fn un_install(&self, version: &str) -> Result<(), SnmError> {
        let default_dir = self.get_runtime_dir_for_default_path_buf()?;
        let version_dir = self.get_runtime_dir_path_buf(&version)?;
        if fs::read_link(&default_dir)?.eq(&version_dir) {
            let msg = format!(
                "🤔 {} is default instance, do you want to uninstall it ?",
                version
            );
            if Confirm::new().with_prompt(msg).interact()? {
                fs::remove_file(&default_dir)?;
                fs::remove_dir_all(version_dir)?;
            }
        } else {
            fs::remove_dir_all(version_dir)?;
        }
        Ok(())
    }

    pub async fn install(&self, version: &str) -> Result<(), SnmError> {
        let anchor_file = self.get_anchor_file_path_buf(&version)?;
        let version_dir = self.get_runtime_dir_path_buf(&version)?;

        if anchor_file.exists().not() {
            self.download(version).await?;
        } else {
            let confirm = Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is already installed, do you want to reinstall it ?",
                    &version
                ))
                .interact()
                .expect("install Confirm error");

            if confirm {
                fs::remove_dir_all(&version_dir)?;
                self.download(version).await?;
            }

            let default_dir = self.get_runtime_dir_for_default_path_buf()?;

            if default_dir.exists().not() {
                #[cfg(unix)]
                {
                    std::os::unix::fs::symlink(&version_dir, &default_dir)?;
                }
                #[cfg(windows)]
                {
                    std::os::windows::fs::symlink_dir(&version_dir, &default_dir)?;
                }
            }
        }

        Ok(())
    }

    pub fn new(snm_config: SnmConfig) -> Self {
        Self { snm_config }
    }

    async fn get_node_list_remote(&self) -> Result<Vec<NodeModel>, SnmError> {
        let host = self.snm_config.get_node_dist_url();
        let node_list_url = format!("{}/index.json", host);

        let client = reqwest::Client::new();

        let node_vec: Vec<NodeModel> = client
            .get(&node_list_url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .json::<Vec<NodeModel>>()
            .await?;
        Ok(node_vec)
    }

    async fn get_node_schedule(&self) -> Result<Vec<NodeSchedule>, SnmError> {
        let host = self.snm_config.get_node_github_resource_host();

        let node_schedule_url = format!("{}/nodejs/Release/main/schedule.json", host);

        let client = reqwest::Client::new();

        let node_schedule_vec: Vec<NodeSchedule> = client
            .get(&node_schedule_url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .json::<std::collections::HashMap<String, NodeSchedule>>()
            .await?
            .into_iter()
            .map(|(v, mut schedule)| {
                schedule.version = Some(v[1..].to_string());
                schedule
            })
            .collect();

        Ok(node_schedule_vec)
    }

    async fn get_node_sha256_hashmap(&self, node_version: &str) -> HashMap<String, String> {
        let host = self.snm_config.get_node_dist_url();
        let url = format!("{}/v{}/SHASUMS256.txt", host, node_version);

        let sha256_str = reqwest::get(&url)
            .await
            .expect(format!("fetch {} failed", url).as_str())
            .text()
            .await
            .expect(format!("parse {} response to text failed", url).as_str());

        let sha256_map: std::collections::HashMap<String, String> = sha256_str
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let sha256 = iter.next().unwrap();
                let file = iter.next().unwrap();
                (file.to_string(), sha256.to_string())
            })
            .collect();

        sha256_map
    }

    fn _show_off_online_node_list(&self, dir_tuple: &(Vec<String>, Option<String>)) {
        let (dir_vec, default_v) = dir_tuple;
        for v in dir_vec {
            let prefix = if Some(v) == default_v.as_ref() {
                "⛳️"
            } else {
                " "
            };
            // 标记
            println!(
                "{:<2} {}  {}",
                prefix,
                v,
                "Network exception, degraded to offline mode.".bright_black()
            );
        }
    }

    fn show_node_list<F>(&self, node_vec: Vec<NodeModel>, get_tag_fn: F)
    where
        F: Fn(&String) -> &str,
    {
        for node in node_vec.iter() {
            let lts = match &node.lts {
                Lts::Str(s) => s,
                Lts::Bool(_) => "",
            };

            let deprecated = node.deprecated.unwrap_or(false);

            let version = if deprecated {
                format!(
                    "{:<10} {:<10}",
                    node.version.bright_black(),
                    lts.bright_black()
                )
            } else {
                format!(
                    "{:<10} {:<10}",
                    node.version.bright_green(),
                    lts.bright_green()
                )
            };

            let died = format!("died on {}", node.end.as_deref().unwrap_or("")).bright_black();

            let npm = format!("npm {}", node.npm.as_deref().unwrap_or("None")).bright_black();

            let openssl =
                format!("openssl {}", node.openssl.as_deref().unwrap_or("None")).bright_black();

            let desc_width = 22;

            let tag = get_tag_fn(&node.version);

            // 标记
            println!(
                "{:<2} {} {:<desc_width$} {:<desc_width$} {:<desc_width$}",
                tag, version, died, openssl, npm,
            );
        }
    }
}

impl AtomTrait for SnmNode {
    fn get_anchor_file_path_buf(&self, version: &str) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_node_bin_dir()?
            .join(&version)
            .join("bin")
            .join("node")
            .to_ok()
    }

    fn download_condition(&self, version: &str) -> bool {
        match self.snm_config.get_node_install_strategy() {
            InstallStrategy::Ask => Confirm::new()
                .with_prompt(format!(
                    "🤔 {} is not installed, do you want to install it ?",
                    &version
                ))
                .interact()
                .expect("download_condition Confirm error"),
            InstallStrategy::Panic => {
                let msg = format!("Unsupported version: {}", version);
                panic!("{msg}");
            }
            InstallStrategy::Auto => true,
        }
    }

    fn get_runtime_binary_dir_string(&self, version: &str) -> Result<String, SnmError> {
        Ok(self
            .get_runtime_dir_path_buf(&version)?
            .join("bin")
            .display()
            .to_string())
    }

    fn get_download_url(&self, v: &str) -> String {
        let host = self.snm_config.get_node_dist_url();
        let download_url = format!(
            "{}/v{}/node-v{}-{}-{}.{}",
            &host,
            &v,
            &v,
            get_os(),
            get_arch(),
            get_tarball_ext()
        );
        download_url
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        self.snm_config
            .get_download_dir()?
            .join("node")
            .join(v)
            .join(format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            ))
            .to_ok()
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        self.snm_config.get_node_bin_dir()?.join(&v).to_ok()
    }

    fn get_runtime_dir_for_default_path_buf(&self) -> Result<PathBuf, SnmError> {
        self.snm_config.get_node_bin_dir()?.join("default").to_ok()
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        self.snm_config.get_node_bin_dir()
    }

    fn get_expect_shasum<'a>(
        &'a self,
        v: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            let mut hashmap = self.get_node_sha256_hashmap(&v).await;
            let tar_file_name = format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            );
            hashmap.remove(&tar_file_name)
        })
    }

    fn get_actual_shasum<'a>(
        &'a self,
        downloaded_file_path_buf: &'a PathBuf,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            if let Ok(file) = File::open(downloaded_file_path_buf) {
                let mut reader = BufReader::new(file);
                let mut hasher = Sha256::new();
                let mut buffer = [0; 1024];
                loop {
                    let n = reader.read(&mut buffer).expect("read error");
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                let result = hasher.finalize();
                Some(format!("{:x}", result))
            } else {
                None
            }
        })
    }

    fn show_list<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let (dir_vec, default_v) = self.read_runtime_dir_name_vec()?;
            if dir_vec.is_empty() {
                let msg = format!(
                    "Node list is empty, please use {} to get the latest version.",
                    "snm node list-remote".bright_green().bold()
                );
                panic!("{msg}");
            }

            let now = Utc::now().date_naive();
            let (remote_node_vec, node_schedule_vec) =
                try_join!(self.get_node_list_remote(), self.get_node_schedule())?;

            let version_req_vec = node_schedule_vec
                .into_iter()
                .filter_map(|schedule| {
                    schedule
                        .version
                        .as_ref()
                        .and_then(|v| VersionReq::parse(v).ok())
                        .map(|vr| (vr, schedule))
                })
                .collect::<Vec<(VersionReq, NodeSchedule)>>();

            let mut hashmap = remote_node_vec
                .into_iter()
                .map(|node| (node.version.as_str().to_string(), node))
                .collect::<HashMap<String, NodeModel>>();

            let mut node_vec = dir_vec
                .into_iter()
                .filter_map(|v| hashmap.remove(format!("v{}", v).as_str()))
                .map(|mut node| {
                    node.version = node.version.trim_start_matches("v").to_string();

                    let version = Version::parse(&node.version);

                    let eq_version = |req: &VersionReq| {
                        version
                            .as_ref()
                            .map_or(false, |version| req.matches(version))
                    };

                    let node_schedule = version_req_vec
                        .iter()
                        .find_map(|(req, schedule)| eq_version(req).then_some(schedule));

                    {
                        node.end = node_schedule
                            .map(|schedule| schedule.end.clone())
                            .map_or(Some("None".to_string()), Some);
                    }

                    {
                        let map_deprecated = |schedule: &NodeSchedule| {
                            NaiveDate::parse_from_str(&schedule.end, "%Y-%m-%d")
                                .map(|end| now > end)
                                .unwrap_or(true)
                        };

                        node.deprecated =
                            node_schedule.map(map_deprecated).map_or(Some(true), Some);
                    }

                    node
                })
                .collect::<Vec<NodeModel>>();

            node_vec.sort_by_cached_key(|v| Version::parse(&v.version[1..]).ok());

            if let Some(v) = default_v {
                self.show_node_list(node_vec, |node_v| {
                    if *node_v == v {
                        return "⛳️";
                    } else {
                        return "";
                    }
                });
            } else {
                self.show_node_list(node_vec, |_node_v| {
                    return "";
                });
            }
            Ok(())
        })
    }

    fn show_list_offline<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let (dir_vec, default_v) = self.read_runtime_dir_name_vec()?;
            if dir_vec.is_empty() {
                let msg = format!(
                    "Node list is empty, please use {} to get the latest version.",
                    "snm node list-remote".bright_green().bold()
                );
                panic!("{msg}");
            }

            dir_vec.iter().for_each(|item| {
                let prefix = if Some(item) == default_v.as_ref() {
                    "⛳️"
                } else {
                    " "
                };
                println!("{:<2} {}", prefix, item);
            });
            Ok(())
        })
    }

    fn show_list_remote<'a>(
        &'a self,
        all: bool,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>> {
        Box::pin(async move {
            let (dir_vec, _default_v) = self.read_runtime_dir_name_vec()?;

            let (mut node_vec, node_schedule_vec) =
                try_join!(self.get_node_list_remote(), self.get_node_schedule())?;

            let now = Utc::now().date_naive();

            node_vec.iter_mut().for_each(|node| {
                let eq_version = |req: VersionReq| {
                    Version::parse(&node.version[1..])
                        .map_or(false, |version| req.matches(&version))
                };
                // 查找匹配的调度 生命周期
                let node_schedule = node_schedule_vec.iter().find(|&schedule| {
                    // 确保 schedule.version 是 Some，并且 VersionReq 和 Version 都可以被成功解析
                    schedule
                        .version
                        .as_ref()
                        .and_then(|v| VersionReq::parse(v).ok())
                        .map_or(false, eq_version)
                });

                if let Some(schedule) = node_schedule {
                    // 更新节点的调度数据
                    node.end = Some(schedule.end.clone());

                    let _ = NaiveDate::parse_from_str(&schedule.end, "%Y-%m-%d").map(|end| {
                        if now > end {
                            node.deprecated = Some(true);
                        } else {
                            node.deprecated = Some(false);
                        }
                    });
                } else {
                    node.end = Some("None".to_string());
                    node.deprecated = Some(true);
                }
            });

            node_vec.sort_by_cached_key(|v| Version::parse(&v.version[1..]).ok());

            let mut marking_version: HashMap<String, String> = HashMap::new();

            dir_vec.iter().for_each(|v| {
                marking_version.insert(v.clone(), v.clone());
            });

            let node_vec = node_vec
                .into_iter()
                .filter(|node| {
                    if all {
                        true
                    } else if let Lts::Str(_) = node.lts {
                        node.deprecated == Some(false)
                    } else {
                        false
                    }
                })
                .collect::<Vec<NodeModel>>();

            self.show_node_list(node_vec, |node_v| {
                let v = node_v.trim_start_matches("v");
                if marking_version.contains_key(v) {
                    return "🫐";
                } else {
                    return "";
                }
            });
            Ok(())
        })
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress(&input_file_path_buf, &output_dir_path_buf)
    }

    fn get_snm_config(&self) -> &SnmConfig {
        &self.snm_config
    }
}
