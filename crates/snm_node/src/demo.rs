use async_trait::async_trait;
use chrono::NaiveDate;
use chrono::Utc;
use dialoguer::Confirm;
use futures::*;
use semver::Version;
use semver::VersionReq;
use sha2::Digest;
use sha2::Sha256;
use snm_core::model::manager::SharedBehavior;
use snm_core::model::manager::ShimTrait;
use snm_core::model::Lts;
use snm_core::model::NodeModel;
use snm_core::model::NodeSchedule;
use snm_core::{
    config::{
        cfg::{get_arch, get_os, get_tarball_ext},
        url::SnmUrl,
        SnmConfig,
    },
    model::{manager::ManagerTrait, SnmError},
    utils::tarball::decompress_xz,
};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;
use std::ops::Not;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::node_list_remote::get_node_list_remote;
use crate::node_list_remote::get_node_schedule;
use crate::node_list_remote::get_node_sha256_hashmap;
use crate::show_node_list::show_node_list;

pub struct NodeDemo {
    snm_config: SnmConfig,
}

impl NodeDemo {
    pub fn new() -> Self {
        Self {
            snm_config: SnmConfig::new(),
        }
    }
}

impl SharedBehavior for NodeDemo {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_bin_dir_path_buf()?
            .join(&v)
            .join("bin")
            .join("node"))
    }
}

#[async_trait(?Send)]
impl ManagerTrait for NodeDemo {
    fn get_download_url(&self, v: &str) -> Result<String, SnmError> {
        Ok(SnmUrl::new().get_node_tar_download_url(&v))
    }

    fn get_downloaded_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_config.get_download_dir_path_buf()?.join(v))
    }

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_download_dir_path_buf()?
            .join(v)
            .join(format!(
                "node-v{}-{}-{}.{}",
                &v,
                get_os(),
                get_arch(),
                get_tarball_ext()
            )))
    }

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.snm_config.get_node_bin_dir_path_buf()?.join(&v))
    }

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self
            .snm_config
            .get_node_bin_dir_path_buf()?
            .join(format!("{}-default", &v)))
    }

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        Ok(self.snm_config.get_node_bin_dir_path_buf()?)
    }

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError> {
        let mut hashmap = get_node_sha256_hashmap(&v).await?;
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
        let file = File::open(downloaded_file_path_buf)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();

        let mut buffer = [0; 1024];
        loop {
            let n = reader.read(&mut buffer)?;
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
            return Err(SnmError::EmptyNodeList)?;
        }

        let now = Utc::now().date_naive();

        let (node_vec, node_schedule_vec) = try_join!(get_node_list_remote(), get_node_schedule())?;

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

        let mut hashmap = node_vec
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

                    node.deprecated = node_schedule.map(map_deprecated).map_or(Some(true), Some);
                }

                node
            })
            .collect::<Vec<NodeModel>>();

        node_vec.sort_by_cached_key(|v| Version::parse(&v.version[1..]).ok());

        if let Some(v) = default_v {
            show_node_list(node_vec, |node_v| {
                if node_v == v {
                    return "⛳️";
                } else {
                    return "";
                }
            });
        } else {
            show_node_list(node_vec, |_node_v| {
                return "";
            });
        }

        Ok(())
    }

    async fn show_list_remote(
        &self,
        dir_tuple: &(Vec<String>, Option<String>),
        all: bool,
    ) -> Result<(), SnmError> {
        let (dir_vec, default_v) = dir_tuple;

        let (mut node_vec, node_schedule_vec) =
            try_join!(get_node_list_remote(), get_node_schedule(),)?;

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

        // let (bin_vec, _) = read_bin_dir()?;

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

        show_node_list(node_vec, |node_v| {
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
        decompress_xz(
            &input_file_path_buf,
            &output_dir_path_buf,
            &mut Some(|_from: &PathBuf, _to: &PathBuf| {}),
        )?;
        Ok(())
    }

    fn get_shim_trait(&self) -> Box<dyn ShimTrait> {
        Box::new(NodeDemo::new())
    }
}

// #[async_trait(?Send)]
impl ShimTrait for NodeDemo {
    fn get_strict_shim_version(&self) -> Result<String, SnmError> {
        let node_version_path_buf = current_dir()?.join(".node-version");
        if node_version_path_buf.exists().not() {
            return Err(SnmError::NotFoundNodeVersionFileError {
                file_path: node_version_path_buf.display().to_string(),
            });
        }
        let version_processor =
            |value: String| value.trim_start_matches(['v', 'V']).trim().to_string();
        let version = read_to_string(node_version_path_buf).map(version_processor)?;
        Ok(version)
    }

    fn get_strict_shim_binary_path_buf(&self, version: &str) -> Result<PathBuf, SnmError> {
        let node_binary_path_buf = self.get_runtime_binary_file_path_buf(&version)?;
        Ok(node_binary_path_buf)
    }

    fn download_condition(&self, version: &str) -> Result<bool, SnmError> {
        match self.snm_config.get_node_install_strategy()? {
            snm_core::config::snm_config::InstallStrategy::Ask => Ok(Confirm::new()
                .with_prompt(format!(
                    "🤔 {} is not installed, do you want to install it ?",
                    &version
                ))
                .interact()?),
            snm_core::config::snm_config::InstallStrategy::Panic => {
                Err(SnmError::UnsupportedNodeVersion {
                    version: version.to_string(),
                })
            }
            snm_core::config::snm_config::InstallStrategy::Auto => Ok(true),
        }
    }

    fn get_runtime_binary_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError> {
        Ok(self.get_runtime_dir_path_buf(v)?.join("bin").join("node"))
    }

    fn check_default_version(
        &self,
        tuple: &(Vec<String>, Option<String>),
    ) -> Result<String, SnmError> {
        let (_, default_v_dir) = tuple;
        if let Some(v) = default_v_dir {
            return Ok(v.to_string());
        } else {
            return Err(SnmError::NotFoundDefaultNodeBinary);
        }
    }
}
