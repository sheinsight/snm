use std::{collections::HashMap, time::Duration};

use snm_config::snm_config::SnmConfig;

use super::metadata::ScheduleMetadata;

#[derive(Debug)]
pub struct Schedule {
  cache: HashMap<String, ScheduleMetadata>,
}

impl Schedule {
  pub async fn new(snm_config: &SnmConfig) -> anyhow::Result<Self> {
    let node_schedule_url = format!(
      "{host}/nodejs/Release/main/schedule.json",
      host = snm_config.node_github_resource_host
    );
    let client = reqwest::Client::new();
    let cache = client
      .get(&node_schedule_url)
      .timeout(Duration::from_secs(10))
      .send()
      .await?
      .json::<HashMap<String, ScheduleMetadata>>()
      .await?;

    Ok(Self { cache })
  }

  pub fn get(&self, version: &str) -> Option<ScheduleMetadata> {
    self.cache.get(version).cloned()
  }
}
