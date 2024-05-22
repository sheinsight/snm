use crate::conditional_compiler::get_arch;
use crate::conditional_compiler::get_os;
use crate::conditional_compiler::get_tarball_ext;
use crate::node_model::Lts;
use crate::node_model::NodeModel;
use crate::node_schedule::NodeSchedule;
use async_trait::async_trait;
use chrono::NaiveDate;
use chrono::Utc;
use colored::*;
use dialoguer::Confirm;
use futures::*;
use semver::Version;
use semver::VersionReq;
use sha2::Digest;
use sha2::Sha256;
use snm_core::model::trait_manage::ManageTrait;
use snm_core::model::trait_shared_behavior::SharedBehaviorTrait;
use snm_core::model::trait_shim::ShimTrait;
use snm_core::utils::get_current_dir::get_current_dir;
use snm_core::{config::SnmConfig, model::SnmError, utils::tarball::decompress_xz};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Not;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct SnmNode {
    snm_config: SnmConfig,
}

impl SnmNode {
    pub fn new() -> Self {
        Self {
            snm_config: SnmConfig::new(),
        }
    }

    async fn get_node_list_remote(&self) -> Result<Vec<NodeModel>, SnmError> {
        let host = self.snm_config.get_nodejs_dist_url_prefix();
        let node_list_url = format!("{}/index.json", host);
        let node_vec: Vec<NodeModel> = reqwest::get(&node_list_url)
            .await
            .map_err(|_| SnmError::Error(format!("fetch {} failed", &node_list_url)))?
            .json::<Vec<NodeModel>>()
            .await
            .map_err(|_| {
                SnmError::Error(format!("parse {} response to json failed", &node_list_url))
            })?;
        Ok(node_vec)
    }

    async fn get_node_schedule(&self) -> Result<Vec<NodeSchedule>, SnmError> {
        let host = self.snm_config.get_nodejs_github_resource_host();

        let node_schedule_url = format!("{}/nodejs/Release/main/schedule.json", host);

        let node_schedule_vec: Vec<NodeSchedule> = reqwest::get(&node_schedule_url)
            .await
            .map_err(|_| SnmError::Error(format!("fetch {} failed", node_schedule_url)))?
            .json::<std::collections::HashMap<String, NodeSchedule>>()
            .await
            .map_err(|_| {
                SnmError::Error(format!(
                    "parse {} response to json failed",
                    node_schedule_url
                ))
            })?
            .into_iter()
            .map(|(v, mut schedule)| {
                schedule.version = Some(v[1..].to_string());
                schedule
            })
            .collect();

        Ok(node_schedule_vec)
    }

    async fn get_node_sha256_hashmap(
        &self,
        node_version: &str,
    ) -> Result<HashMap<String, String>, SnmError> {
        let host = self.snm_config.get_nodejs_dist_url_prefix();
        let url = format!("{}/v{}/SHASUMS256.txt", host, node_version);

        let sha256_str = reqwest::get(&url)
            .await
            .map_err(|_| SnmError::Error(format!("fetch {} failed", url)))?
            .text()
            .await
            .map_err(|_| SnmError::Error(format!("parse {} response to text failed", url)))?;

        let sha256_map: std::collections::HashMap<String, String> = sha256_str
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let sha256 = iter.next().unwrap();
                let file = iter.next().unwrap();
                (file.to_string(), sha256.to_string())
            })
            .collect();

        Ok(sha256_map)
    }

    fn show_off_online_node_list(&self, dir_tuple: &(Vec<String>, Option<String>)) {
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

impl SharedBehaviorTrait for SnmNode {
    fn get_anchor_file_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_node_bin_dir_path_buf()
            .join(&v)
            .join("bin")
            .join("node")
    }
}

#[async_trait(?Send)]
impl ManageTrait for SnmNode {
    fn get_download_url(&self, v: &str) -> String {
        let host = self.snm_config.get_nodejs_dist_url_prefix();
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

    fn get_downloaded_dir_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config.get_download_dir_path_buf().join(v)
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_download_dir_path_buf()
            .join(v)
            .join(format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            ))
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config.get_node_bin_dir_path_buf().join(&v)
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> PathBuf {
        self.snm_config
            .get_node_bin_dir_path_buf()
            .join(format!("{}-default", &v))
    }

    fn get_runtime_base_dir_path_buf(&self) -> PathBuf {
        self.snm_config.get_node_bin_dir_path_buf()
    }

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError> {
        let mut hashmap = self.get_node_sha256_hashmap(&v).await?;
        let tar_file_name = format!(
            "node-v{}-{}-{}.{}",
            &v,
            get_os(),
            get_arch(),
            get_tarball_ext()
        );
        let expect_sha256 = hashmap
            .remove(&tar_file_name)
            .ok_or(SnmError::NotFoundSha256ForNode(tar_file_name.to_string()))?;
        Ok(expect_sha256)
    }

    async fn get_actual_shasum(
        &self,
        downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError> {
        let file = File::open(downloaded_file_path_buf).map_err(|_| {
            SnmError::Error(format!(
                "open file {} error",
                downloaded_file_path_buf.display()
            ))
        })?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();

        let mut buffer = [0; 1024];
        loop {
            let n = reader
                .read(&mut buffer)
                .map_err(|_| SnmError::Error("read error".to_string()))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    fn get_host(&self) -> Option<String> {
        None
    }

    async fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError> {
        let (dir_vec, default_v) = dir_tuple;
        if dir_vec.is_empty() {
            return Err(SnmError::Error(format!(
                "Node list is empty, please use {} to get the latest version.",
                "snm node list-remote".bright_green().bold()
            )));
        }

        let now = Utc::now().date_naive();

        if let Ok((remote_node_vec, node_schedule_vec)) =
            try_join!(self.get_node_list_remote(), self.get_node_schedule())
        {
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
                    if node_v == v {
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
        } else {
            self.show_off_online_node_list(dir_tuple)
        }

        Ok(())
    }

    async fn show_list_remote(
        &self,
        dir_tuple: &(Vec<String>, Option<String>),
        all: bool,
    ) -> Result<(), SnmError> {
        let (dir_vec, _default_v) = dir_tuple;

        let (mut node_vec, node_schedule_vec) =
            try_join!(self.get_node_list_remote(), self.get_node_schedule())?;

        let now = Utc::now().date_naive();

        node_vec.iter_mut().for_each(|node| {
            let eq_version = |req: VersionReq| {
                Version::parse(&node.version[1..]).map_or(false, |version| req.matches(&version))
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
    }

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError> {
        decompress_xz(&input_file_path_buf, &output_dir_path_buf).map_err(|_| {
            SnmError::Error(format!(
                "decompress_xz {} error",
                &input_file_path_buf.display().to_string()
            ))
        })?;
        Ok(())
    }

    fn get_shim_trait(&self) -> Box<dyn ShimTrait> {
        Box::new(SnmNode::new())
    }
}

impl ShimTrait for SnmNode {
    fn check_satisfy_strict_mode(&self, _bin_name: &str) -> Result<(), SnmError> {
        let node_version_path_buf = get_current_dir()?.join(".node-version");

        if node_version_path_buf.exists().not() {
            return Err(SnmError::NotFoundNodeVersionFile {
                file_path: node_version_path_buf.display().to_string(),
            });
        }

        return Ok(());
    }

    fn get_strict_shim_version(&self) -> Result<String, SnmError> {
        let node_version_path_buf = get_current_dir()?.join(".node-version");
        if node_version_path_buf.exists().not() {
            return Err(SnmError::NotFoundNodeVersionFile {
                file_path: node_version_path_buf.display().to_string(),
            });
        }
        let version_processor =
            |value: String| value.trim_start_matches(['v', 'V']).trim().to_string();
        let version = read_to_string(&node_version_path_buf)
            .map(version_processor)
            .map_err(|_| {
                SnmError::Error(format!(
                    "read_to_string {} error",
                    &node_version_path_buf.display().to_string()
                ))
            })?;
        Ok(version)
    }

    fn get_strict_shim_binary_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError> {
        Ok(self.get_runtime_binary_file_path_buf(&bin_name, &version)?)
    }

    fn download_condition(&self, version: &str) -> Result<bool, SnmError> {
        match self.snm_config.get_node_install_strategy()? {
            snm_core::config::snm_config::InstallStrategy::Ask => Ok(Confirm::new()
                .with_prompt(format!(
                    "🤔 {} is not installed, do you want to install it ?",
                    &version
                ))
                .interact()
                .map_err(|_| SnmError::Error("download_condition Confirm error".to_string()))?),
            snm_core::config::snm_config::InstallStrategy::Panic => {
                Err(SnmError::Error(format!("Unsupported version: {}", version)))
            }
            snm_core::config::snm_config::InstallStrategy::Auto => Ok(true),
        }
    }

    fn get_runtime_binary_file_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError> {
        Ok(self
            .get_runtime_dir_path_buf(&version)
            .join("bin")
            .join(bin_name))
    }

    fn check_default_version(
        &self,
        tuple: &(Vec<String>, Option<String>),
    ) -> Result<String, SnmError> {
        let (_, default_v_dir) = tuple;
        if let Some(v) = default_v_dir {
            return Ok(v.to_string());
        } else {
            return Err(SnmError::Error(format!(
                "Not found default node version, please use {} to set default node version.",
                "snm node default <version>".bright_green().bold()
            )));
        }
    }
}
