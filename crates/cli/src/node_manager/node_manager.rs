use crate::node_manager::node_model::Lts;

use super::{node_model::NodeModel, node_schedule::NodeSchedule};
use chrono::{NaiveDate, Utc};
use colored::*;
use dialoguer::Confirm;
use semver::Version;
use semver::VersionReq;
use snm_atom::atom::AtomTrait;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::snm_error::SnmError;
use std::{collections::HashMap, fs, ops::Not as _, path::PathBuf, time::Duration};
use tokio::try_join;

pub struct NodeManager<'a, T: AtomTrait> {
    node_atom: &'a T,
}

pub struct ListArgs {
    pub offline: bool,
}

pub struct ListRemoteArgs {
    pub all: bool,
}

impl<'a, T> NodeManager<'a, T>
where
    T: AtomTrait,
{
    async fn internal_download(&self, version: &str) -> Result<(), SnmError> {
        let download_url = self.node_atom.get_download_url(version);
        let downloaded_file_path_buf = self.node_atom.get_downloaded_file_path_buf(version)?;

        DownloadBuilder::new()
            .retries(3)
            .write_strategy(WriteStrategy::Nothing)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        let runtime = self.node_atom.get_runtime_dir_path_buf(version)?;

        if runtime.exists() {
            fs::remove_dir_all(&runtime)?;
        }

        let expect = self.node_atom.get_expect_shasum(version).await?;

        let actual = self
            .node_atom
            .get_actual_shasum(&downloaded_file_path_buf)
            .await?;

        if actual.is_none() || expect.is_none() {
            fs::remove_file(&downloaded_file_path_buf)?;
            return Err(SnmError::ShasumError {
                file_path: downloaded_file_path_buf.display().to_string(),
                expect: "None".to_string(),
                actual: "None".to_string(),
            });
        }

        if actual.eq(&expect).not() {
            fs::remove_file(&downloaded_file_path_buf)?;
            return Err(SnmError::ShasumError {
                file_path: downloaded_file_path_buf.display().to_string(),
                expect: expect.unwrap_or("None".to_string()),
                actual: actual.unwrap_or("None".to_string()),
            });
        }

        self.node_atom
            .decompress_download_file(&downloaded_file_path_buf, &runtime)?;

        fs::remove_file(&downloaded_file_path_buf)?;

        Ok(())
    }

    fn internal_set_default(&self, version: &str) -> Result<PathBuf, SnmError> {
        let default_dir = self.node_atom.get_runtime_dir_for_default_path_buf()?;
        if default_dir.exists() {
            fs::remove_dir_all(&default_dir)?;
        }

        let from_dir = self.node_atom.get_runtime_dir_path_buf(version)?;

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&from_dir, &default_dir)?;
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(&version_dir, &default_dir)?;
        }
        Ok(default_dir)
    }

    async fn get_node_list_remote(&self) -> Result<Vec<NodeModel>, SnmError> {
        let host = self.node_atom.get_snm_config().get_node_dist_url();
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
        let host = self
            .node_atom
            .get_snm_config()
            .get_node_github_resource_host();

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

impl<'a, T> NodeManager<'a, T>
where
    T: AtomTrait,
{
    pub fn new(node_atom: &'a T) -> Self {
        Self { node_atom }
    }

    pub async fn set_default(&self, version: &str) -> Result<(), SnmError> {
        if self
            .node_atom
            .get_anchor_file_path_buf(version)?
            .exists()
            .not()
        {
            let msg = format!(
                "🤔 v{} is not installed, do you want to install it ?",
                version
            );
            if Confirm::new().with_prompt(msg).interact()? {
                self.install(version).await?;
            }
        }

        self.internal_set_default(version)?;

        Ok(())
    }

    pub async fn install(&self, version: &str) -> Result<(), SnmError> {
        let anchor_file = self.node_atom.get_anchor_file_path_buf(&version)?;
        let version_dir = self.node_atom.get_runtime_dir_path_buf(&version)?;

        let downloaded_file_path_buf = self.node_atom.get_downloaded_file_path_buf(version)?;

        if downloaded_file_path_buf.exists() {
            fs::remove_file(&downloaded_file_path_buf)?;
        }

        let node_install_strategy = self.node_atom.get_snm_config().get_node_install_strategy();

        if anchor_file.exists() {
            let confirm = Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is already installed, do you want to reinstall it ?",
                    &version
                ))
                .interact()?;

            if confirm {
                fs::remove_dir_all(&version_dir)?;
                self.internal_download(version).await?;
            }
        } else {
            match node_install_strategy {
                snm_config::InstallStrategy::Ask => {
                    let confirm = Confirm::new()
                        .with_prompt(format!("🤔 Do you want to install v{} ?", &version))
                        .interact()?;
                    if confirm {
                        self.internal_download(version).await?;
                    }
                }
                snm_config::InstallStrategy::Panic => todo!(),
                snm_config::InstallStrategy::Auto => {
                    self.internal_download(version).await?;
                }
            }
        }

        self.internal_set_default(version)?;

        Ok(())
    }

    pub async fn un_install(&self, version: &str) -> Result<(), SnmError> {
        let default_dir = self.node_atom.get_runtime_dir_for_default_path_buf()?;
        let version_dir = self.node_atom.get_runtime_dir_path_buf(&version)?;
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

    pub async fn list_remote(&self, args: ListRemoteArgs) -> Result<(), SnmError> {
        let (dir_vec, _default_v) = self.node_atom.read_runtime_dir_name_vec()?;

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
                if args.all {
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

    pub async fn list(&self, args: ListArgs) -> Result<(), SnmError> {
        let (dir_vec, default_v) = self.node_atom.read_runtime_dir_name_vec()?;

        if args.offline {
            dir_vec.iter().for_each(|item| {
                let prefix = if Some(item) == default_v.as_ref() {
                    "⛳️"
                } else {
                    " "
                };
                println!("{:<2} {}", prefix, item);
            });
        } else {
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
        }

        Ok(())
    }
}
