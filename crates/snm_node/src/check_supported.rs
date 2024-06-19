use regex::Regex;
use std::time::Duration;

use crate::conditional_compiler::{get_arch, get_os};

pub async fn _check_supported(node_version: &str, node_dist_url: &str) {
    let client = reqwest::Client::new();
    let response = client
        .get(node_dist_url)
        .timeout(Duration::from_secs(60))
        .send()
        .await;

    // if request err, like timeout
    if response.is_err() {
        let msg = format!("request error {:?}", response.err());
        panic!("{msg}");
    }

    let response = response.unwrap();
    let response_status = response.status();

    let os = get_os();
    let arch = get_arch();

    // response is not 200-299
    if !response_status.is_success() {
        panic!("UnsupportedPlatform {os} {arch}");
    }

    let text = response.text().await;

    if text.is_err() {
        let msg = format!("parse {} response to text failed", node_dist_url);
        panic!("{msg}");
    }

    let text = text.unwrap();

    let re = Regex::new(r#"<a\s+href="([^"]+)">[^<]+</a>"#).unwrap();

    let prefix = format!("node-v{}", node_version);

    let os = get_os();
    let arch = get_arch();

    let temp_su = format!("{prefix}-{os}-{arch}");

    let mut supported_list: Vec<String> = vec![];

    let mut is_supported = false;

    for cap in re.captures_iter(&text) {
        let node_item = &cap[1];
        if node_item.contains(&temp_su) {
            is_supported = true;
            break;
        }

        if node_item.starts_with(&prefix) {
            supported_list.push(node_item.to_string())
        }
    }

    if !is_supported {
        eprintln!("not support {}", &temp_su);

        eprintln!("supported list {:?}", &supported_list);
        panic!("UnsupportedPlatform {os} {arch}")
    }
}
