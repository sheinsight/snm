use std::{collections::HashMap, fmt, time::Duration};

use chrono::{NaiveDate, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use snm_config::snm_config::SnmConfig;

use super::lts::Lts;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMetadata {
  pub version: String,
  pub date: String,
  // pub files: Vec<String>,
  pub npm: Option<String>,
  pub v8: String,
  pub uv: Option<String>,
  pub zlib: Option<String>,
  pub openssl: Option<String>,
  // pub modules: Option<String>,
  pub lts: Lts,
  pub security: bool,
  // pub end: Option<String>,
  // pub current: Option<String>,
  // pub deprecated: Option<bool>,
  pub schedule: Option<ScheduleMetadata>,

  #[serde(skip)]
  pub default: Option<bool>,
}

impl fmt::Display for NodeMetadata {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = &self.version[1..];

    let lts = match &self.lts {
      Lts::Str(s) => format!("lts ( {} )", s),
      Lts::Bool(b) if *b => "lts".to_string(),
      _ => "".to_string(),
    };

    let died_on = self
      .schedule
      .as_ref()
      .map(|s| format!("died on {}", s.end.clone()))
      .unwrap_or(String::new());

    let ssl = self
      .openssl
      .as_ref()
      .map(|s| format!("openssl {}", s.clone()))
      .unwrap_or(String::new());

    let npm = self
      .npm
      .as_ref()
      .map(|s| format!("npm {}", s.clone()))
      .unwrap_or(String::new());

    let now = Utc::now().date_naive();

    let end = NaiveDate::parse_from_str(&self.schedule.as_ref().unwrap().end, "%Y-%m-%d").unwrap();

    write!(
      f,
      r#"{pdd:<2} {v:<12} {npm:<20} {ssl:<20} {died_on:<22} {nike_name:<10}"#,
      pdd = self
        .default
        .map(|d| if d { "->" } else { "" })
        .unwrap_or("")
        .bright_green(),
      v = v.green(),
      npm = npm.bright_green(),
      ssl = ssl.bright_black(),
      died_on = if now > end {
        died_on.bright_red()
      } else {
        died_on.bright_black()
      },
      nike_name = lts.bright_green()
    )
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleMetadata {
  pub start: String,
  pub end: String,
  pub maintenance: Option<String>,
  pub lts: Option<String>,
  pub codename: Option<String>,
  pub version: Option<String>,
}

impl ScheduleMetadata {
  pub async fn fetch(snm_config: &SnmConfig) -> anyhow::Result<HashMap<String, ScheduleMetadata>> {
    let url = format!(
      "{host}/nodejs/Release/main/schedule.json",
      host = snm_config.node_github_resource_host
    );

    let client = reqwest::Client::builder()
      .timeout(Duration::from_secs(snm_config.download_timeout_secs))
      .build()?;
    let resp = client
      .get(&url)
      .send()
      .await?
      .json::<HashMap<String, ScheduleMetadata>>()
      .await?;

    Ok(resp)
  }
}
