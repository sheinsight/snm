use crate::config::cfg::{get_arch, get_os, get_tarball_ext};

pub struct SnmUrl {
    pub node_host_url: String,
    pub node_schedule_host_url: String,
}

impl SnmUrl {
    pub fn new() -> Self {
        Self {
            node_host_url: "https://nodejs.org".to_string(),
            node_schedule_host_url: "https://raw.githubusercontent.com".to_string(),
        }
    }

    pub fn use_node_list_url(&self) -> String {
        format!("{}/dist/index.json", self.node_host_url)
    }

    pub fn get_node_sha256_download_url(&self, node_version: &str) -> String {
        format!("https://nodejs.org/dist/v{}/SHASUMS256.txt", node_version)
    }

    pub fn use_node_schedule_url(&self) -> String {
        format!(
            "{}/nodejs/Release/main/schedule.json",
            self.node_schedule_host_url
        )
    }

    pub fn get_node_tar_download_url(&self, node_version: &str) -> String {
        format!(
            "{}/dist/v{}/node-v{}-{}-{}.{}",
            self.node_host_url,
            node_version,
            node_version,
            get_os(),
            get_arch(),
            get_tarball_ext()
        )
    }
}
