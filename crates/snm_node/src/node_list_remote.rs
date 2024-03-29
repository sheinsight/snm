use snm_core::{
    config::url::SnmUrl,
    model::{NodeModel, NodeScheduleModel, SnmError},
    utils::{
        download::{DownloadBuilder, WriteStrategy},
        read_to_json,
    },
};
use std::collections::HashMap;

use crate::path::{get_node_list_json, get_node_schedule_json, get_node_tar_sha256_file_path};

pub async fn get_node_list_remote() -> Result<Vec<NodeModel>, SnmError> {
    let node_list_json = get_node_list_json()?;
    let node_list_url = SnmUrl::new().use_node_list_url();

    DownloadBuilder::new()
        .write_strategy(WriteStrategy::Nothing)
        .download(&node_list_url, &node_list_json)
        .await?;

    let node_vec = read_to_json::<Vec<NodeModel>>(&node_list_json)?;

    Ok(node_vec)
}

pub async fn get_node_schedule() -> Result<Vec<NodeScheduleModel>, SnmError> {
    let node_schedule_json_path = get_node_schedule_json()?;

    let node_schedule_url = SnmUrl::new().use_node_schedule_url();

    DownloadBuilder::new()
        .write_strategy(WriteStrategy::Nothing)
        .download(&node_schedule_url, &node_schedule_json_path)
        .await?;

    let node_schedule_vec = read_to_json::<std::collections::HashMap<String, NodeScheduleModel>>(
        &node_schedule_json_path,
    )?
    .into_iter()
    .map(|(v, mut schedule)| {
        schedule.version = Some(v[1..].to_string());
        schedule
    })
    .collect();

    Ok(node_schedule_vec)
}

pub async fn get_node_sha256_hashmap(
    node_version: &str,
) -> Result<HashMap<String, String>, SnmError> {
    // let path = SnmPath::new().get_sha256_path(node_version);
    let path = get_node_tar_sha256_file_path(node_version)?;
    let download_url = SnmUrl::new().get_node_sha256_download_url(node_version);

    DownloadBuilder::new()
        .write_strategy(WriteStrategy::Nothing)
        .download(&download_url, &path)
        .await?;

    let sha256_str = std::fs::read_to_string(path)?;

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
