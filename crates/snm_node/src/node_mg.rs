use crate::node_list_remote::get_node_sha256_hashmap;
use crate::{node_list_remote::get_node_list_remote, show_node_list::show_node_list};
use crate::{
    node_list_remote::get_node_schedule,
    path::{
        get_default_node_binary_file_path, get_default_node_dir, get_node_binary_base_dir,
        get_node_binary_file_path, get_node_dir, get_node_tar_file_name, get_node_tar_file_path,
    },
};
use chrono::{NaiveDate, Utc};
use colored::*;
use dialoguer::Confirm;
use futures::*;
use semver::{Version, VersionReq};
use snm_core::{
    config::url::SnmUrl,
    model::{Lts, NodeModel, NodeScheduleModel, SnmError},
    utils::{
        calc_sha256,
        download::{DownloadBuilder, WriteStrategy},
        find_up,
        tarball::decompress_xz,
    },
};
use snm_core::{print_warning, println_success};
use std::collections::HashMap;
use std::fs::read_to_string;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;
use std::{io::stdout, path::PathBuf};

fn read_bin_dir() -> Result<(Vec<String>, Option<String>), SnmError> {
    let mut default_version = None;

    let dir_path = get_node_binary_base_dir()?;

    return match dir_path.read_dir() {
        Ok(dir_reader) => {
            let dir_vec: Vec<String> = dir_reader
                .filter_map(Result::ok)
                .filter(|x| x.path().is_dir())
                .filter_map(|x| {
                    let version = x.file_name().into_string().ok()?;
                    if version.ends_with("default") {
                        default_version = Some(version.trim_end_matches("-default").to_string());
                    }
                    Some(version)
                })
                .collect();
            Ok((dir_vec, default_version))
        }
        Err(_) => Err(SnmError::ReadDirFailed {
            dir_path: dir_path.display().to_string(),
        }),
    };
}

fn create_symlink(original: &PathBuf, link: &PathBuf) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        unix_fs::symlink(original, link)
    }
    #[cfg(windows)]
    {
        windows_fs::symlink_dir(original, link)
    }
}

fn is_default_fn(node_version: &str) -> Result<(bool, PathBuf), SnmError> {
    let default_binary_dir = get_default_node_binary_file_path(node_version)?;

    let defaulted = default_binary_dir.exists();

    return Ok((defaulted, default_binary_dir));
}

fn ask_download(node_version: &str) -> Result<bool, SnmError> {
    let proceed = Confirm::new()
        .with_prompt(format!(
            "🤔 Node {} does not exist, do you want to download it ?",
            &node_version
        ))
        .interact()?;
    Ok(proceed)
}

fn ask_re_install(node_version: &str) -> Result<bool, SnmError> {
    let proceed = Confirm::new()
        .with_prompt(format!(
            "🤔 Node v{} has already been installed , Do you want to re-install it ?",
            node_version.strip_prefix('v').unwrap_or(node_version)
        ))
        .interact()?;
    Ok(proceed)
}

async fn use_default_node() -> Result<(String, PathBuf), SnmError> {
    let (versions, default_version) = read_bin_dir()?;
    if versions.is_empty() {
        return Err(SnmError::EmptyNodeList)?;
    }
    if let Some(version) = default_version {
        let node_binary_abs_path = get_node_binary_file_path(&version)?;
        Ok((version, node_binary_abs_path))
    } else {
        Err(SnmError::NotFoundDefaultNodeBinary.into())
    }
}

async fn check_node_sha256(node_version: &str, down_path: &PathBuf) -> Result<(), SnmError> {
    let hashmap = get_node_sha256_hashmap(node_version).await?;
    let tar_file_name = get_node_tar_file_name(node_version);
    let expect_sha256 = hashmap
        .get(&tar_file_name)
        .ok_or(SnmError::NotFoundSha256ForNode(
            tar_file_name.to_string().bright_red().to_string(),
        ))?;
    let actual_sha256 = calc_sha256(down_path)?;
    if *expect_sha256 == actual_sha256 {
        Ok(())
    } else {
        Err(SnmError::Sha256VerificationFailed {
            file_path: down_path.display().to_string(),
            expect: expect_sha256.to_string(),
            actual: actual_sha256,
        })
    }
}

async fn download(node_version: &str) -> anyhow::Result<PathBuf, SnmError> {
    let tar_file_path = get_node_tar_file_path(node_version)?;

    let download_url = SnmUrl::new().get_node_tar_download_url(node_version);

    let node_dir = get_node_dir(node_version)?;

    let mut stdout = stdout();

    // let download_progress = |downloaded_size, total_size| {
    //     let percentage = (downloaded_size as f64 / total_size as f64) * 100.0;
    //     // execute!(&stdout, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
    //     // warning!(
    //     //     "Downloading: {:.2}% {}/{}",
    //     //     percentage,
    //     //     downloaded_size,
    //     //     total_size
    //     // );
    //     print_warning!(
    //         stdout,
    //         "Downloading: {:.2}% {}/{}",
    //         percentage,
    //         downloaded_size,
    //         total_size
    //     );
    //     // stdout.flush().ok();
    // };

    print_warning!(stdout, "Waiting Download...");

    DownloadBuilder::new()
        .retries(3)
        .write_strategy(WriteStrategy::WriteAfterDelete)
        // .progress(download_progress)
        .download(&download_url, &tar_file_path)
        .await?;

    // execute!(stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
    // success!("Downloaded\n");
    // let mut stdout = stdout();

    println_success!(stdout, "Downloaded");

    check_node_sha256(&node_version, &tar_file_path).await?;

    // let mut stdout = stdout();

    let mut decompress_progress = Some(|_from: &PathBuf, to: &PathBuf| {
        // execute!(&stdout, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();

        // let mut lock = stdout.lock();

        // print_warning!(lock, "Decompressing: {}", to.display());

        // stdout.flush().ok();
        // todo!("终端刷的太慢了")
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    });

    print_warning!(stdout, "Waiting Decompress...");

    decompress_xz(&tar_file_path, &node_dir, &mut decompress_progress)?;

    println_success!(stdout, "Decompressed");

    Ok(node_dir)
}

pub async fn un_install_node(node_version: &str) -> Result<(), SnmError> {
    let (is_defaulted, default_binary_dir) = is_default_fn(node_version)?;

    // 如果是默认版本，请求用户确认
    if is_defaulted {
        let confirm = Confirm::new()
        .with_prompt(format!(
            "💥 Node {} is currently set as the default version. Deleting it means you'll need to manually set a new default using 'snm default <version>'.", node_version
        ))
        .interact()?;

        // 如果用户不确认，提前返回
        if !confirm {
            return Ok(());
        }
    }

    let node_binary_dir = get_node_dir(node_version)?;
    // 若目录不存在，记录日志，并提前返回
    if !node_binary_dir.exists() {
        // facade_log!("🚫 Node v{} not found!", node_version);
        return Ok(());
    }

    // 尝试删除目录，并处理可能的错误
    std::fs::remove_dir_all(&node_binary_dir)?;

    // 如果这是默认版本，还需要删除默认的二进制文件目录
    if is_defaulted {
        std::fs::remove_dir_all(&default_binary_dir)?;
    }

    // facade_log!("🗑️ Node v{} has been removed!", node_version);
    Ok(())
}

pub async fn snm_node_env() -> Result<(), SnmError> {
    if let Some(node_version_abs_path) = find_up(".node-version", None)? {
        let node_version_str = read_to_string(&node_version_abs_path)?;

        let bin_abs_path = get_node_binary_file_path(&node_version_str)?;

        // facade_log!(
        //     "Node Binary Path : {}",
        //     transform_abs_to_user(&bin_abs_path)
        // );

        // facade_log!(
        //     "Config File Path : {}",
        //     transform_abs_to_user(&node_version_abs_path)
        // );
    } else {
        let (_dir_vec, default_v) = read_bin_dir()?;
        if let Some(v) = default_v {
            let node_binary_abs_path = get_node_binary_file_path(&v);
        } else {
            return Err(SnmError::NotFoundDefaultNodeBinary.into());
        }
        // facade_log!(
        //     "Node Binary Path : {}",
        //     node_binary_abs_path.display().to_string().bright_black()
        // );
        // facade_log!("Config File Path : {}", "None".bright_black());
    }

    Ok(())
}

pub async fn set_default(node_version: &str) -> Result<(), SnmError> {
    let (dir_vec, _) = read_bin_dir()?;

    let default_dir = dir_vec.iter().find(|x| x.ends_with("-default"));

    let expect_node_dir = get_node_dir(node_version)?;

    let default_abs_path: PathBuf = get_default_node_dir(node_version)?;

    if expect_node_dir.exists() {
        if !default_abs_path.exists() {
            if expect_node_dir.is_dir() {
                create_symlink(&expect_node_dir, &default_abs_path)?;
            }

            if let Some(dir) = default_dir {
                let old_default = get_node_binary_base_dir()?.join(dir);
                tokio::fs::remove_dir_all(old_default).await?;
            }
        }
    } else {
        if ask_download(node_version)? {
            let node_dir = install_node(&node_version).await?;
            create_symlink(&node_dir, &default_abs_path)?;
        }
    }

    Ok(())
}

pub async fn use_bin(v: &str) -> Result<PathBuf, SnmError> {
    let mut stdout = stdout();
    let node_binary_abs_path = get_node_binary_file_path(&v)?;

    if !node_binary_abs_path.exists() {
        if ask_download(&v)? {
            install_node(&v).await?;
        }
    }

    println_success!(stdout, "Use Node {} .", format!("{}", v.green()));

    Ok(node_binary_abs_path)

    // let node_version_file = find_up(".node-version", None)?;

    // if let Some(node_version_file_abs_path) = node_version_file {
    //     let node_version_str = tokio::fs::read_to_string(&node_version_file_abs_path)
    //         .await?
    //         .trim()
    //         .trim_start_matches("v")
    //         .to_string();

    //     let node_binary_abs_path = get_node_binary_file_path(&node_version_str)?;

    //     if !node_binary_abs_path.exists() {
    //         if ask_download(&node_version_str)? {
    //             install_node(&node_version_str).await?;
    //         }
    //     }

    //     println_success!(
    //         stdout,
    //         "Use Node {} .",
    //         format!("{}", node_version_str.green())
    //     );

    //     Ok(node_binary_abs_path)
    // } else {
    //     let (node_version_str, node_binary_abs_path) = use_default_node().await?;
    //     println_success!(
    //         stdout,
    //         "Use Node {} . {}",
    //         node_version_str.green(),
    //         "by default".bright_black()
    //     );

    //     Ok(node_binary_abs_path)
    // }
}

pub async fn install_node(node_version: &str) -> Result<PathBuf, SnmError> {
    if get_node_binary_file_path(node_version)?.exists() {
        if ask_re_install(node_version)? {
            let node_dir = download(&node_version).await?;
            return Ok(node_dir);
        } else {
            return Err(SnmError::RefuseToInstallNode);
        }
    } else {
        let node_dir = download(&node_version).await?;
        return Ok(node_dir);
    }
}

pub async fn list_remote(all: bool) -> Result<(), SnmError> {
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

    let (bin_vec, _) = read_bin_dir()?;

    let mut marking_version: HashMap<String, String> = HashMap::new();

    bin_vec.iter().for_each(|v| {
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

pub async fn list() -> Result<(), SnmError> {
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
        .collect::<Vec<(VersionReq, NodeScheduleModel)>>();

    let (dir_vec, default_version) = read_bin_dir()?;

    if dir_vec.is_empty() {
        return Err(SnmError::EmptyNodeList)?;
    }

    let now = Utc::now().date_naive();

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
                let map_deprecated = |schedule: &NodeScheduleModel| {
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

    if let Some(v) = default_version {
        show_node_list(node_vec, |node_v| {
            if *node_v == v {
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

#[cfg(test)]
mod tests {
    use super::*;
}
